use std::{
    any::{Any, TypeId},
    hash::Hash,
    slice::{Iter, IterMut},
};

use handle_map::{map::SparseHandleMap, Handle};
use hashbrown::HashMap;
use indexmap::IndexMap;

use crate::{
    pearl::{Event, SimpleEvent},
    world::{maps::PearlMap, View},
    Pearl,
};

use self::sealed::EventMap;

use super::{maps::AnyPearlMap, view::ViewWalker};

type MapHandle = handle_map::Handle<Box<dyn AnyPearlMap>>;
type PearlHandle<P> = handle_map::Handle<P>;

pub struct Link<P> {
    map_handle: MapHandle,
    pearl_handle: PearlHandle<P>,
}

impl<P> Copy for Link<P> {}
impl<P> Clone for Link<P> {
    fn clone(&self) -> Self {
        Self {
            map_handle: self.map_handle.clone(),
            pearl_handle: self.pearl_handle.clone(),
        }
    }
}

impl<P> Hash for Link<P> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.map_handle.hash(state);
        self.pearl_handle.hash(state);
    }
}

impl<P> Eq for Link<P> {}
impl<P> PartialEq for Link<P> {
    fn eq(&self, other: &Self) -> bool {
        self.map_handle == other.map_handle && self.pearl_handle == other.pearl_handle
    }
}

impl<P> Link<P> {
    #[doc(hidden)]
    pub fn from_raw(map: u64, pearl: u64) -> Self {
        Self {
            map_handle: Handle::from_raw(map),
            pearl_handle: Handle::from_raw(pearl),
        }
    }

    #[doc(hidden)]
    pub fn into_type<T>(self) -> Link<T> {
        Link {
            map_handle: self.map_handle,
            pearl_handle: self.pearl_handle.into_type(),
        }
    }
}

pub struct RemoveContext<'a, P> {
    pub world: &'a mut World,
    pub pearl: &'a mut P,
    _private: (),
}

pub struct InsertContext<'a, 'view, P: Pearl> {
    pub view: &'a mut View<'view, P>,
    pub link: Link<P>,
    _private: (),
}

struct PearlData {
    map_handle: MapHandle,
    on_remove: Vec<fn(&mut World)>,
}

impl PearlData {
    pub fn new(map_handle: MapHandle) -> Self {
        Self {
            map_handle,
            on_remove: Vec::new(),
        }
    }
}

#[derive(Default)]
pub struct World {
    pearl_data: HashMap<TypeId, PearlData>,
    pearl_maps: SparseHandleMap<Box<dyn AnyPearlMap>>,
    event_runners: HashMap<TypeId, Box<dyn Any>>,
}

impl World {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn contains<P: 'static>(&self, link: Link<P>) -> bool {
        self.get(link).is_some()
    }

    pub fn get<P: 'static>(&self, link: Link<P>) -> Option<&P> {
        let anymap = self.pearl_maps.get_data(link.map_handle)?;
        anymap.as_map::<P>().get_data(link.pearl_handle)
    }

    pub fn get_mut<P: 'static>(&mut self, link: Link<P>) -> Option<&mut P> {
        let anymap = self.pearl_maps.get_data_mut(link.map_handle)?;
        anymap.as_map_mut::<P>().get_data_mut(link.pearl_handle)
    }

    pub fn iter<P: 'static>(&self) -> Option<Iter<P>> {
        let data = self.pearl_data.get(&TypeId::of::<P>())?;
        let anymap = self.pearl_maps.get_data(data.map_handle).unwrap();
        Some(anymap.as_map::<P>().iter())
    }

    pub fn iter_mut<P: 'static>(&mut self) -> Option<IterMut<P>> {
        let data = self.pearl_data.get(&TypeId::of::<P>())?;
        let anymap = self.pearl_maps.get_data_mut(data.map_handle).unwrap();
        Some(anymap.as_map_mut::<P>().iter_mut())
    }

    pub fn view<P: Pearl>(&mut self, link: Link<P>) -> Option<View<P>> {
        View::new(self, link)
    }

    pub fn view_walk<P: Pearl>(&mut self) -> Option<ViewWalker<P>> {
        ViewWalker::new(self)
    }

    pub fn remove<P: Pearl>(&mut self, link: Link<P>) -> Option<P> {
        let anymap = self.pearl_maps.get_data_mut(link.map_handle)?;
        let map = anymap.as_map_mut::<P>();
        let mut pearl = map.remove(link.pearl_handle)?;

        // early return if map is not empty
        if !map.is_empty() {
            return Some(pearl);
        }

        // remove map and run all event removers
        let pearl_id = TypeId::of::<P>();
        self.pearl_maps.remove(link.map_handle);
        let pearl_data = self.pearl_data.remove(&pearl_id).unwrap();
        for remover in pearl_data.on_remove.iter() {
            remover(self);
        }

        P::on_remove(RemoveContext {
            world: self,
            pearl: &mut pearl,
            _private: (),
        });
        Some(pearl)
    }

    pub fn insert<P: Pearl>(&mut self, pearl: P) -> Link<P> {
        self.insert_and(pearl, |_| {})
    }

    pub fn insert_and<P: Pearl>(&mut self, pearl: P, f: impl FnOnce(&mut View<P>)) -> Link<P> {
        use hashbrown::hash_map::Entry as E;
        let link = match self.pearl_data.entry(TypeId::of::<P>()) {
            E::Occupied(e) => {
                let map_handle = e.get().map_handle;
                let map = self.pearl_maps[map_handle].as_map_mut::<P>();
                let pearl_handle = map.insert(pearl);
                Link {
                    map_handle,
                    pearl_handle,
                }
            }
            E::Vacant(e) => {
                let mut map = PearlMap::new();
                let pearl_handle = map.insert(pearl);
                let map_handle = self.pearl_maps.insert(Box::new(map));
                e.insert(PearlData::new(map_handle));
                P::register(self);
                Link {
                    map_handle,
                    pearl_handle,
                }
            }
        };

        let mut view = self.view(link).unwrap();
        P::on_insert(InsertContext {
            view: &mut view,
            link,
            _private: (),
        });
        f(&mut view);
        link
    }

    pub fn trigger_simple<E: SimpleEvent>(&mut self, event: &E) {
        self.trigger::<E>(event)
    }

    pub fn trigger<E: Event>(&mut self, event: &E::Data<'_>) {
        let event_id = TypeId::of::<E>();
        let Some(anymap) = self.event_runners.remove(&event_id) else {
            return;
        };

        let event_map = anymap.downcast_ref::<EventMap<E>>().unwrap();
        for runner in event_map.values() {
            runner(self, event);
        }

        use hashbrown::hash_map::Entry;
        match self.event_runners.entry(event_id) {
            Entry::Vacant(e) => {
                e.insert(anymap);
            }
            Entry::Occupied(e) => {
                e.into_mut()
                    .downcast_mut::<EventMap<E>>()
                    .unwrap()
                    .extend(event_map);
            }
        }
    }
}

// seal event source implementation so it cannot be called manually
mod sealed {
    use log::info;

    use crate::pearl::Listener;

    use super::*;

    pub type EventFn<E> = for<'a> fn(&mut World, &<E as Event>::Data<'a>);
    pub type EventMap<E> = IndexMap<TypeId, EventFn<E>>;

    /// the function to be called when an event is run on a world
    fn event_runner<'a, E: Event, P: Pearl + Listener<E>>(world: &mut World, data: &E::Data<'a>) {
        info!(
            "Running `{}` event for all '{}' pearls",
            std::any::type_name::<E>(),
            std::any::type_name::<P>()
        );

        let Some(mut view_walker) = world.view_walk::<P>() else {
            return;
        };

        while let Some(mut view) = view_walker.walk_next() {
            P::update(&mut view, data)
        }
    }

    /// the function to be called when pearls are removed from a world
    fn event_remover<E: Event, P: Pearl>(world: &mut World) {
        let anymap = world.event_runners.get_mut(&TypeId::of::<E>()).unwrap();
        let map = anymap.downcast_mut::<EventMap<E>>().unwrap();
        map.remove(&TypeId::of::<P>());
    }

    impl<P: Pearl> crate::pearl::EventSource<P> for World {
        fn listen<E: crate::pearl::Event>(&mut self)
        where
            P: crate::pearl::Listener<E>,
        {
            // create event and pearl ids
            let event_id = TypeId::of::<E>();
            let pearl_id = TypeId::of::<P>();

            // insert new event runner function
            use hashbrown::hash_map::Entry;
            match self.event_runners.entry(event_id) {
                Entry::Occupied(e) => {
                    let map = e.into_mut().downcast_mut::<EventMap<E>>().unwrap();
                    map.insert(pearl_id, event_runner::<E, P>);
                }
                Entry::Vacant(e) => {
                    let mut map = EventMap::new();
                    map.insert(pearl_id, event_runner::<E, P>);
                    e.insert(Box::new(map));
                }
            }

            // insert new event remover function
            let data = self.pearl_data.get_mut(&pearl_id).unwrap();
            data.on_remove.push(event_remover::<E, P>);
        }
    }
}
