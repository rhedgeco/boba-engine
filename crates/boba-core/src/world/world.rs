use std::{
    any::{Any, TypeId},
    hash::Hash,
};

use handle_map::{map::SparseHandleMap, Handle};
use hashbrown::HashMap;
use indexmap::IndexMap;

use crate::{
    pearl::{Event, SimpleEvent},
    world::{map::WorldMap, view::DestroyQueue},
    Pearl,
};

use super::{
    map::{self, AnyMap},
    view::{View, Walker},
};

pub(super) type AnyMapBox = Box<dyn AnyMap>;

pub struct Link<P> {
    pub(super) map_handle: Handle<AnyMapBox>,
    pub(super) data_handle: Handle<P>,
}

impl<P> Copy for Link<P> {}
impl<P> Clone for Link<P> {
    fn clone(&self) -> Self {
        Self {
            map_handle: self.map_handle.clone(),
            data_handle: self.data_handle.clone(),
        }
    }
}

impl<P> Hash for Link<P> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.map_handle.hash(state);
        self.data_handle.hash(state);
    }
}

impl<P> Eq for Link<P> {}
impl<P> PartialEq for Link<P> {
    fn eq(&self, other: &Self) -> bool {
        self.map_handle == other.map_handle && self.data_handle == other.data_handle
    }
}

impl<P> Link<P> {
    #[doc(hidden)]
    pub fn from_raw(map: u64, data: u64) -> Self {
        Self {
            map_handle: Handle::from_raw(map),
            data_handle: Handle::from_raw(data),
        }
    }

    #[doc(hidden)]
    pub fn into_type<P2>(self) -> Link<P2> {
        Link {
            map_handle: self.map_handle,
            data_handle: self.data_handle.into_type(),
        }
    }
}

pub struct RemoveContext<'a, P> {
    pub world: &'a mut World,
    pub pearl: &'a mut P,
    pub old_link: Link<P>,
    _private: (),
}

pub struct InsertContext<'a, 'view, P: Pearl> {
    pub view: &'a mut View<'view, P>,
    pub link: Link<P>,
    _private: (),
}

pub type Iter<'a, P> = map::Iter<'a, P>;
pub type IterMut<'a, P> = map::IterMut<'a, P>;

struct PearlData {
    map_handle: Handle<AnyMapBox>,
    on_remove: Vec<fn(&mut World)>,
}

impl PearlData {
    pub fn new(map_handle: Handle<AnyMapBox>) -> Self {
        Self {
            map_handle,
            on_remove: Vec::new(),
        }
    }
}

#[derive(Default)]
pub struct World {
    pearl_data: IndexMap<TypeId, PearlData>,
    pearl_maps: SparseHandleMap<AnyMapBox>,
    global_pearls: HashMap<TypeId, Link<()>>,
    event_runners: HashMap<TypeId, Box<dyn Any>>,
}

impl World {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn count<P: Pearl>(&self) -> usize {
        match self.get_map::<P>() {
            Some(map) => map.len(),
            None => 0,
        }
    }

    pub fn has_type<P: Pearl>(&self) -> bool {
        self.pearl_data.get(&TypeId::of::<P>()).is_some()
    }

    pub fn contains<P: Pearl>(&self, link: Link<P>) -> bool {
        let Some(map) = self.get_map_with(link.map_handle) else {
            return false;
        };

        map.contains(link.data_handle)
    }

    pub fn contains_global<P: Pearl>(&self) -> bool {
        let Some(global_link) = self.get_global_link::<P>() else {
            return false;
        };

        self.contains(global_link)
    }

    pub fn get_global<P: Pearl>(&self) -> Option<&P> {
        self.get(self.get_global_link::<P>()?)
    }

    pub fn get<P: Pearl>(&self, link: Link<P>) -> Option<&P> {
        self.get_map_with(link.map_handle)?.get(link.data_handle)
    }

    pub fn get_global_link<P: Pearl>(&self) -> Option<Link<P>> {
        Some(self.global_pearls.get(&TypeId::of::<P>())?.into_type())
    }

    pub fn get_global_mut<P: Pearl>(&mut self) -> Option<&mut P> {
        self.get_mut(self.get_global_link::<P>()?)
    }

    pub fn get_mut<P: Pearl>(&mut self, link: Link<P>) -> Option<&mut P> {
        self.get_map_with_mut(link.map_handle)?
            .get_mut(link.data_handle)
    }

    pub fn iter<P: Pearl>(&self) -> Iter<P> {
        match self.get_map::<P>() {
            Some(map) => map.iter(),
            None => Iter::empty(),
        }
    }

    pub fn iter_mut<P: Pearl>(&mut self) -> IterMut<P> {
        match self.get_map_mut::<P>() {
            Some(map) => map.iter_mut(),
            None => IterMut::empty(),
        }
    }

    pub fn insert<P: Pearl>(&mut self, data: P) -> Link<P> {
        self.insert_and(data, |_| {})
    }

    pub fn insert_global<P: Pearl>(&mut self, data: P) -> Option<P> {
        self.insert_global_and(data, |_| {})
    }

    pub fn insert_and<P: Pearl>(&mut self, data: P, f: impl FnOnce(&mut View<P>)) -> Link<P> {
        let link = self.raw_insert(data);
        self.run_post_insert(link, f);
        link
    }

    pub fn insert_global_and<P: Pearl>(
        &mut self,
        data: P,
        f: impl FnOnce(&mut View<P>),
    ) -> Option<P> {
        let pearl_id = TypeId::of::<P>();
        use hashbrown::hash_map::Entry as E;
        if let E::Occupied(e) = self.global_pearls.entry(pearl_id) {
            let link = e.get().into_type::<P>();
            let anymap = self.pearl_maps.get_data_mut(link.map_handle).unwrap();
            let map = anymap.as_map_mut::<P>().unwrap();
            let pearl = map.get_mut(link.data_handle).unwrap();
            let mut old_pearl = core::mem::replace(pearl, data);
            P::on_remove(RemoveContext {
                world: self,
                pearl: &mut old_pearl,
                old_link: link,
                _private: (),
            });
            self.run_post_insert(link, f);
            return Some(old_pearl);
        }

        let new_link = self.raw_insert(data);
        self.global_pearls.insert(pearl_id, new_link.into_type());
        self.run_post_insert(new_link, f);
        None
    }

    fn raw_insert<P: Pearl>(&mut self, data: P) -> Link<P> {
        use indexmap::map::Entry as E;
        match self.pearl_data.entry(TypeId::of::<P>()) {
            E::Occupied(e) => {
                let map_handle = e.get().map_handle;
                let map = self.pearl_maps[map_handle].as_map_mut::<P>().unwrap();
                let data_handle = map.insert(data);
                Link {
                    map_handle,
                    data_handle,
                }
            }
            E::Vacant(e) => {
                let mut map = WorldMap::new();
                let data_handle = map.insert(data);
                let map_handle = self.pearl_maps.insert(Box::new(map));
                e.insert(PearlData::new(map_handle));
                P::register(self);
                Link {
                    map_handle,
                    data_handle,
                }
            }
        }
    }

    fn run_post_insert<P: Pearl>(&mut self, link: Link<P>, f: impl FnOnce(&mut View<P>)) {
        // run insertion functions in scope so view is dropped immidiately
        let mut destroy_queue = DestroyQueue::new();
        {
            let mut view = View::<P>::new(link, self, &mut destroy_queue).unwrap();
            P::on_insert(InsertContext {
                view: &mut view,
                link,
                _private: (),
            });
            f(&mut view);
        }

        // execute pending destroy queue
        destroy_queue.execute_on(self);
    }

    pub fn remove_global<P: Pearl>(&mut self) -> Option<P> {
        let link = self.global_pearls.remove(&TypeId::of::<P>())?;
        self.remove(link.into_type())
    }

    pub fn remove<P: Pearl>(&mut self, link: Link<P>) -> Option<P> {
        let map = self.get_map_with_mut(link.map_handle)?;
        let mut data = map.remove(link.data_handle)?;

        // if map was emptied, remove its data from the world
        if map.is_empty() {
            self.pearl_data.remove(&TypeId::of::<P>()).unwrap();
            self.pearl_maps.remove(link.map_handle).unwrap();
        }

        P::on_remove(RemoveContext {
            world: self,
            pearl: &mut data,
            old_link: link,
            _private: (),
        });

        Some(data)
    }

    pub fn trigger_simple<E: SimpleEvent>(&mut self, event: &E) -> bool {
        self.trigger::<E>(event)
    }

    pub fn trigger<E: Event>(&mut self, event: &E::Data<'_>) -> bool {
        let mut destroy_queue = DestroyQueue::new();
        if !self.nested_trigger::<E>(event, &mut destroy_queue) {
            return false;
        }

        destroy_queue.execute_on(self);
        true
    }

    pub(super) fn nested_trigger<'a, E: Event>(
        &mut self,
        event: &E::Data<'a>,
        destroy_queue: &mut DestroyQueue,
    ) -> bool {
        let event_id = TypeId::of::<E>();
        let Some(anymap) = self.event_runners.get(&event_id) else {
            return false;
        };

        let event_map = anymap.downcast_ref::<sealed::EventMap<E>>().unwrap();
        for runner in event_map.clone().values() {
            runner(self, event, destroy_queue);
        }

        true
    }

    pub(super) fn get_map_handle<P: Pearl>(&self) -> Option<Handle<AnyMapBox>> {
        Some(self.pearl_data.get(&TypeId::of::<P>())?.map_handle)
    }

    pub(super) fn get_map<P: Pearl>(&self) -> Option<&WorldMap<P>> {
        let map_handle = self.get_map_handle::<P>()?;
        let anymap = self.pearl_maps.get_data(map_handle).unwrap();
        Some(anymap.as_map::<P>().unwrap())
    }

    pub(super) fn get_map_mut<P: Pearl>(&mut self) -> Option<&mut WorldMap<P>> {
        let map_handle = self.get_map_handle::<P>()?;
        let anymap = self.pearl_maps.get_data_mut(map_handle).unwrap();
        Some(anymap.as_map_mut::<P>().unwrap())
    }

    pub(super) fn get_map_with<P: Pearl>(&self, handle: Handle<AnyMapBox>) -> Option<&WorldMap<P>> {
        self.pearl_maps.get_data(handle)?.as_map::<P>()
    }

    pub(super) fn get_map_with_mut<P: Pearl>(
        &mut self,
        handle: Handle<AnyMapBox>,
    ) -> Option<&mut WorldMap<P>> {
        self.pearl_maps.get_data_mut(handle)?.as_map_mut::<P>()
    }
}

// seal event source implementation so it cannot be called externally
mod sealed {
    use super::*;

    pub type EventFn<E> = for<'a> fn(&mut World, &<E as Event>::Data<'a>, &mut DestroyQueue);
    pub type EventMap<E> = IndexMap<TypeId, EventFn<E>>;

    /// the function to be called when pearls are removed from a world
    fn event_remover<E: Event, P: Pearl>(world: &mut World) {
        let anymap = world.event_runners.get_mut(&TypeId::of::<E>()).unwrap();
        let map = anymap.downcast_mut::<EventMap<E>>().unwrap();
        map.remove(&TypeId::of::<P>());
    }

    /// the function to be called when an event is run on a world
    fn event_runner<'a, E: Event, P: Pearl + crate::pearl::Listener<E>>(
        world: &mut World,
        data: &E::Data<'a>,
        destroy_queue: &mut DestroyQueue,
    ) {
        log::info!(
            "Running `{}` event for all '{}' pearls",
            std::any::type_name::<E>(),
            std::any::type_name::<P>()
        );

        let Some(mut view_walker) = Walker::new(world, destroy_queue) else {
            return;
        };

        while let Some(mut view) = view_walker.next() {
            P::update(&mut view, data)
        }
    }

    impl<P: Pearl> crate::pearl::EventSource<P> for World {
        fn listen<E: Event>(&mut self)
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
