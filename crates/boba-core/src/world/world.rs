use std::{
    any::{Any, TypeId},
    hash::Hash,
    marker::PhantomData,
    ops::{Deref, DerefMut},
};

use handle_map::{map::SparseHandleMap, Handle};
use hashbrown::HashMap;
use indexmap::{IndexMap, IndexSet};

use crate::{
    pearl::{Event, SimpleEvent},
    world::map::WorldMap,
    Pearl,
};

use super::map::{self, AnyMap};

type AnyMapBox = Box<dyn AnyMap>;

pub struct Link<P> {
    map_handle: Handle<AnyMapBox>,
    data_handle: Handle<P>,
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
    _private: (),
}

pub struct InsertContext<'a, 'view, P: Pearl> {
    pub view: &'a mut View<'view, P>,
    pub link: Link<P>,
    _private: (),
}

pub struct DropContext<'a, 'view, P: Pearl> {
    pub view: &'a mut View<'view, P>,
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

    pub fn get<P: Pearl>(&self, link: Link<P>) -> Option<&P> {
        self.get_map_with(link.map_handle)?.get(link.data_handle)
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

    pub fn view_walker<P: Pearl>(&mut self) -> Option<ViewWalker<P>> {
        ViewWalker::new(self)
    }

    pub fn insert<P: Pearl>(&mut self, data: P) -> Link<P> {
        self.insert_and(data, |_| {})
    }

    pub fn insert_and<P: Pearl>(&mut self, data: P, f: impl FnOnce(&mut View<P>)) -> Link<P> {
        use indexmap::map::Entry as E;
        let (map_handle, data_handle, data_index) = match self.pearl_data.entry(TypeId::of::<P>()) {
            E::Occupied(e) => {
                let map_handle = e.get().map_handle;
                let map = self.pearl_maps[map_handle].as_map_mut::<P>().unwrap();
                let data_handle = map.insert(data);
                let data_index = map.index_of(data_handle).unwrap();
                (map_handle, data_handle, data_index)
            }
            E::Vacant(e) => {
                let mut map = WorldMap::new();
                let data_handle = map.insert(data);
                let data_index = map.index_of(data_handle).unwrap();
                let map_handle = self.pearl_maps.insert(Box::new(map));
                e.insert(PearlData::new(map_handle));
                P::register(self);
                (map_handle, data_handle, data_index)
            }
        };

        let mut destroy_queue = IndexSet::new();
        let mut view = View::<P> {
            world: self,
            destroy_queue: &mut destroy_queue,
            map_handle,
            data_index,
            _type: PhantomData,
        };
        let link = Link {
            map_handle,
            data_handle,
        };
        P::on_insert(InsertContext {
            view: &mut view,
            link,
            _private: (),
        });

        // call insert_and function
        f(&mut view);
        drop(view);

        // destroy any links in the queue
        for link in destroy_queue.iter() {
            if let Some(map) = self.pearl_maps.get_data_mut(link.map_handle) {
                map.destroy(link.data_handle)
            }
        }

        link
    }

    pub fn remove<P: Pearl>(&mut self, link: Link<P>) -> Option<P> {
        let map = self.get_map_with_mut(link.map_handle)?;
        let data = map.remove(link.data_handle)?;

        // if map was emptied, remove its data from the world
        if map.is_empty() {
            self.pearl_data.remove(&TypeId::of::<P>()).unwrap();
            self.pearl_maps.remove(link.map_handle).unwrap();
        }

        Some(data)
    }

    pub fn trigger_simple<E: SimpleEvent>(&mut self, event: &E) {
        self.trigger::<E>(event)
    }

    pub fn trigger<E: Event>(&mut self, event: &E::Data<'_>) {
        let event_id = TypeId::of::<E>();
        let Some(anymap) = self.event_runners.remove(&event_id) else {
            return;
        };

        let event_map = anymap.downcast_ref::<sealed::EventMap<E>>().unwrap();
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
                    .downcast_mut::<sealed::EventMap<E>>()
                    .unwrap()
                    .extend(event_map);
            }
        }
    }

    fn get_map<P: Pearl>(&self) -> Option<&WorldMap<P>> {
        let map_handle = self.pearl_data.get(&TypeId::of::<P>())?.map_handle;
        let anymap = self.pearl_maps.get_data(map_handle).unwrap();
        Some(anymap.as_map::<P>().unwrap())
    }

    fn get_map_mut<P: Pearl>(&mut self) -> Option<&mut WorldMap<P>> {
        let map_handle = self.pearl_data.get(&TypeId::of::<P>())?.map_handle;
        let anymap = self.pearl_maps.get_data_mut(map_handle).unwrap();
        Some(anymap.as_map_mut::<P>().unwrap())
    }

    fn get_map_with<P: Pearl>(&self, handle: Handle<AnyMapBox>) -> Option<&WorldMap<P>> {
        self.pearl_maps.get_data(handle)?.as_map::<P>()
    }

    fn get_map_with_mut<P: Pearl>(
        &mut self,
        handle: Handle<AnyMapBox>,
    ) -> Option<&mut WorldMap<P>> {
        self.pearl_maps.get_data_mut(handle)?.as_map_mut::<P>()
    }
}

pub struct ViewWalker<'a, P> {
    world: &'a mut World,
    destroy_queue: IndexSet<Link<()>>,
    map_handle: Handle<AnyMapBox>,
    data_index: usize,
    data_cap: usize,

    _type: PhantomData<*const P>,
}

impl<'a, P> Drop for ViewWalker<'a, P> {
    fn drop(&mut self) {
        // destroy all pending links
        for link in self.destroy_queue.iter() {
            if let Some(map) = self.world.pearl_maps.get_data_mut(link.map_handle) {
                map.destroy(link.data_handle)
            }
        }
    }
}

impl<'a, P: Pearl> ViewWalker<'a, P> {
    pub fn new(world: &'a mut World) -> Option<Self> {
        let map_handle = world.pearl_data.get(&TypeId::of::<P>())?.map_handle;
        let data_cap = world.get_map_with::<P>(map_handle).unwrap().len();
        Some(Self {
            world,
            destroy_queue: IndexSet::new(),
            map_handle,
            data_index: 0,
            data_cap,
            _type: PhantomData,
        })
    }

    pub fn next(&mut self) -> Option<View<P>> {
        if self.data_index == self.data_cap {
            return None;
        }

        let data_index = self.data_index;
        self.data_index += 1;
        Some(View {
            world: self.world,
            destroy_queue: &mut self.destroy_queue,
            map_handle: self.map_handle,
            data_index,
            _type: PhantomData,
        })
    }
}

pub struct View<'a, P: Pearl> {
    world: &'a mut World,
    destroy_queue: &'a mut IndexSet<Link<()>>,
    map_handle: Handle<AnyMapBox>,
    data_index: usize,

    _type: PhantomData<*const P>,
}

impl<'a, P: Pearl> Drop for View<'a, P> {
    fn drop(&mut self) {
        P::on_view_drop(DropContext {
            view: self,
            _private: (),
        })
    }
}

impl<'a, P: Pearl> DerefMut for View<'a, P> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        let map = self.world.get_map_with_mut(self.map_handle).unwrap();
        map.get_index_mut(self.data_index).unwrap().1
    }
}

impl<'a, P: Pearl> Deref for View<'a, P> {
    type Target = P;

    fn deref(&self) -> &Self::Target {
        let map = self.world.get_map_with(self.map_handle).unwrap();
        map.get_index(self.data_index).unwrap().1
    }
}

impl<'a, P: Pearl> View<'a, P> {
    pub fn count<P2: Pearl>(&self) -> usize {
        self.world.count::<P2>()
    }

    pub fn has_type<P2: Pearl>(&self) -> bool {
        self.world.has_type::<P2>()
    }

    pub fn contains<P2: Pearl>(&self, link: Link<P2>) -> bool {
        self.world.contains(link)
    }

    pub fn get<P2: Pearl>(&self, link: Link<P2>) -> Option<&P2> {
        self.world.get(link)
    }

    pub fn view_other<P2: Pearl>(&mut self, link: Link<P2>) -> Option<View<P2>> {
        let map = self.world.get_map_with(link.map_handle)?;
        let data_index = map.index_of(link.data_handle)?;
        Some(View {
            world: self.world,
            destroy_queue: self.destroy_queue,
            map_handle: link.map_handle,
            data_index,
            _type: PhantomData,
        })
    }

    pub fn iter<P2: Pearl>(&self) -> Iter<P2> {
        self.world.iter::<P2>()
    }

    pub fn iter_mut<P2: Pearl>(&mut self) -> IterMut<P2> {
        // iterating mutably will not disturb any indices
        self.world.iter_mut::<P2>()
    }

    pub fn insert<P2: Pearl>(&mut self, data: P2) -> Link<P2> {
        // inserting does not disturb any indices
        self.world.insert(data)
    }

    pub fn destroy<P2: Pearl>(&mut self, link: Link<P2>) -> bool {
        // check if the link is valid
        if !self.world.contains(link) {
            return false;
        }

        // we have to queue removals because removing data disturbs indices
        self.destroy_queue.insert(link.into_type())
    }
}

mod sealed {
    use super::*;

    pub type EventFn<E> = for<'a> fn(&mut World, &<E as Event>::Data<'a>);
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
    ) {
        log::info!(
            "Running `{}` event for all '{}' pearls",
            std::any::type_name::<E>(),
            std::any::type_name::<P>()
        );

        let Some(mut view_walker) = world.view_walker::<P>() else {
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
