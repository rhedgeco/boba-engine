use super::{PearlExt, PearlId};
use crate::{Event, EventListener, EventRegister, Pearl, Resources};
use fxhash::{FxBuildHasher, FxHashMap};
use handle_map::map::sparse::SparseHandleMap;
use indexmap::IndexMap;
use std::{
    any::{Any, TypeId},
    marker::PhantomData,
    ops::{Deref, DerefMut},
};

pub type RawHandle = handle_map::RawHandle;
pub type Handle<P> = handle_map::Handle<P>;
pub type Iter<'a, P> = std::slice::Iter<'a, PearlEntry<P>>;
pub type IterMut<'a, P> = std::slice::IterMut<'a, PearlEntry<P>>;

#[derive(Debug)]
pub struct PearlEntry<P: Pearl> {
    handle: Handle<P>,
    pearl: P,
}

impl<P: Pearl> PearlEntry<P> {
    fn new(handle: Handle<P>, pearl: P) -> Self {
        Self { handle, pearl }
    }

    pub fn handle(&self) -> Handle<P> {
        self.handle
    }
}

impl<P: Pearl> DerefMut for PearlEntry<P> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.pearl
    }
}

impl<P: Pearl> Deref for PearlEntry<P> {
    type Target = P;

    fn deref(&self) -> &Self::Target {
        &self.pearl
    }
}

#[derive(Debug)]
pub struct PearlMap<P: Pearl> {
    links: SparseHandleMap<usize>,
    pearls: Vec<PearlEntry<P>>,
}

impl<P: Pearl> Default for PearlMap<P> {
    fn default() -> Self {
        Self {
            links: Default::default(),
            pearls: Default::default(),
        }
    }
}

impl<P: Pearl> PearlMap<P> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn len(&self) -> usize {
        self.pearls.len()
    }

    pub fn is_empty(&self) -> bool {
        self.pearls.is_empty()
    }

    pub fn contains(&self, handle: Handle<P>) -> bool {
        self.links.contains(handle.into_type())
    }

    pub fn get(&self, handle: Handle<P>) -> Option<&P> {
        let index = *self.links.get_data(handle.into_type())?;
        Some(&self.pearls[index].pearl)
    }

    pub fn get_mut(&mut self, handle: Handle<P>) -> Option<&mut P> {
        let index = *self.links.get_data(handle.into_type())?;
        Some(&mut self.pearls[index].pearl)
    }

    pub fn iter(&self) -> Iter<P> {
        self.pearls.iter()
    }

    pub fn iter_mut(&mut self) -> IterMut<P> {
        self.pearls.iter_mut()
    }

    pub fn insert(&mut self, pearl: P) -> Handle<P> {
        let handle = self.links.insert(self.pearls.len()).into_type();
        self.pearls.push(PearlEntry::new(handle, pearl));
        handle
    }

    pub fn remove(&mut self, handle: Handle<P>) -> Option<P> {
        let index = self.links.remove(handle.into_type())?;
        let removed = self.pearls.swap_remove(index).pearl;
        if let Some(swapped) = self.pearls.get_mut(index) {
            self.links
                .get_data_mut(swapped.handle.into_type())
                .map(|i| *i = index);
        }

        Some(removed)
    }
}

pub trait UntypedPearlMap: Any {
    fn len(&self) -> usize;
    fn is_empty(&self) -> bool;
    fn contains(&self, raw: RawHandle) -> bool;
    fn pearl_id(&self) -> PearlId;
    fn destroy(&mut self, raw: RawHandle);
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

impl<P: Pearl> UntypedPearlMap for PearlMap<P> {
    fn len(&self) -> usize {
        self.len()
    }

    fn is_empty(&self) -> bool {
        self.is_empty()
    }

    fn contains(&self, raw: RawHandle) -> bool {
        self.contains(Handle::from_raw(raw))
    }

    fn pearl_id(&self) -> PearlId {
        P::id()
    }

    fn destroy(&mut self, raw: RawHandle) {
        self.remove(Handle::from_raw(raw));
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

pub enum MapStatus<T> {
    New(T),
    Old(T),
}

impl<T> MapStatus<T> {
    pub fn value(&self) -> &T {
        match self {
            Self::New(v) => v,
            Self::Old(v) => v,
        }
    }

    pub fn value_mut(&mut self) -> &mut T {
        match self {
            Self::New(v) => v,
            Self::Old(v) => v,
        }
    }

    pub fn into_value(self) -> T {
        match self {
            Self::New(v) => v,
            Self::Old(v) => v,
        }
    }
}

#[derive(Default)]
pub struct AnyPearlMap {
    map_index: FxHashMap<PearlId, usize>,
    pearl_maps: Vec<Box<dyn UntypedPearlMap>>,
}

impl AnyPearlMap {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn len(&self) -> usize {
        self.pearl_maps.len()
    }

    pub fn get_map<P: Pearl>(&self) -> Option<&PearlMap<P>> {
        let index = *self.map_index.get(&P::id())?;
        self.pearl_maps[index].as_any().downcast_ref()
    }

    pub fn get_map_mut<P: Pearl>(&mut self) -> Option<&mut PearlMap<P>> {
        let index = *self.map_index.get(&P::id())?;
        self.pearl_maps[index].as_any_mut().downcast_mut()
    }

    pub fn insert<P: Pearl>(&mut self, pearl: P) -> MapStatus<Handle<P>> {
        match self.get_or_create_map() {
            MapStatus::New(map) => MapStatus::New(map.insert(pearl)),
            MapStatus::Old(map) => MapStatus::Old(map.insert(pearl)),
        }
    }

    pub fn remove<P: Pearl>(&mut self, handle: Handle<P>) -> Option<P> {
        self.get_map_mut()?.remove(handle)
    }

    pub fn get<P: Pearl>(&self, handle: Handle<P>) -> Option<&P> {
        self.get_map()?.get(handle)
    }

    pub fn get_mut<P: Pearl>(&mut self, handle: Handle<P>) -> Option<&mut P> {
        self.get_map_mut()?.get_mut(handle)
    }

    pub fn iter<P: Pearl>(&self) -> Option<Iter<P>> {
        let pearls = self.get_map()?;
        Some(pearls.iter())
    }

    pub fn iter_mut<P: Pearl>(&mut self) -> Option<IterMut<P>> {
        let pearls = self.get_map_mut()?;
        Some(pearls.iter_mut())
    }

    fn get_or_create_map<P: Pearl>(&mut self) -> MapStatus<&mut PearlMap<P>> {
        let mut new = false;
        use std::collections::hash_map::Entry as E;
        let index = match self.map_index.entry(P::id()) {
            E::Occupied(e) => *e.get(),
            E::Vacant(e) => {
                let index = self.pearl_maps.len();
                self.pearl_maps.push(Box::new(PearlMap::<P>::new()));
                e.insert(index);
                new = true;
                index
            }
        };

        let map = self.pearl_maps[index].as_any_mut().downcast_mut();
        let map = map.expect("Internal Error: Faulty downcast");
        match new {
            true => MapStatus::New(map),
            false => MapStatus::Old(map),
        }
    }
}

#[derive(Default)]
pub struct PearlArena {
    events: ArenaEventRegistry,
    anymap: AnyPearlMap,
}

impl PearlArena {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get<P: Pearl>(&mut self, handle: Handle<P>) -> Option<&P> {
        self.anymap.get(handle)
    }

    pub fn get_mut<P: Pearl>(&mut self, handle: Handle<P>) -> Option<&P> {
        self.anymap.get(handle)
    }

    pub fn iter<P: Pearl>(&self) -> Option<Iter<P>> {
        self.anymap.iter()
    }

    pub fn iter_mut<P: Pearl>(&mut self) -> Option<IterMut<P>> {
        self.anymap.iter_mut()
    }

    pub fn remove<P: Pearl>(&mut self, handle: Handle<P>) -> Option<P> {
        self.anymap.remove(handle)
    }

    pub fn insert<P: Pearl>(&mut self, pearl: P) -> Handle<P> {
        match self.anymap.insert(pearl) {
            MapStatus::Old(handle) => handle,
            MapStatus::New(handle) => {
                P::register(&mut self.events);
                handle
            }
        }
    }

    pub fn trigger<E: Event>(&mut self, event: &mut E, resources: &mut Resources) {
        let Some(vec_index) = self.events.index_map.get(&TypeId::of::<E>()).map(|i| *i) else {
            return;
        };

        let mut runner_index = 0;
        while runner_index < self.events.runners[vec_index].len() {
            let runner = self.events.runners[vec_index]
                .as_any()
                .downcast_ref::<Vec<EventRunner<E>>>()
                .expect("Internal Error: Faulty downcast")[runner_index];

            runner(event, self, resources);
            runner_index += 1;
        }
    }
}

pub struct PearlArenaView<'a, P: Pearl> {
    map_index: usize,
    pearl_index: usize,
    source: &'a mut PearlArena,
    destroy_queue: IndexMap<PearlId, Vec<RawHandle>, FxBuildHasher>,
    _type: PhantomData<*const P>,
}

impl<'a, T: Pearl> PearlArenaView<'a, T> {
    fn create_view(map_index: usize, source: &'a mut PearlArena) -> Option<Self> {
        let pearl_map = &source.anymap.pearl_maps[map_index];
        if pearl_map.is_empty() {
            return None;
        }

        Some(Self {
            map_index,
            pearl_index: 0,
            source,
            destroy_queue: Default::default(),
            _type: PhantomData,
        })
    }

    fn next_view(&mut self) -> bool {
        self.pearl_index += 1;
        let pearl_map = &self.source.anymap.pearl_maps[self.map_index];
        if self.pearl_index >= pearl_map.len() {
            for (id, handles) in self.destroy_queue.drain(..) {
                let Some(map_index) = self.source.anymap.map_index.get(&id) else {
                    continue;
                };
                let pearl_map = &mut self.source.anymap.pearl_maps[*map_index];
                for raw_handle in handles {
                    pearl_map.destroy(raw_handle);
                }
            }

            return false;
        }

        true
    }

    pub fn get<P: Pearl>(&mut self, handle: Handle<P>) -> Option<&P> {
        self.source.get(handle)
    }

    pub fn get_mut<P: Pearl>(&mut self, handle: Handle<P>) -> Option<&P> {
        self.source.get(handle)
    }

    pub fn iter<P: Pearl>(&self) -> Option<Iter<P>> {
        self.source.iter()
    }

    pub fn iter_mut<P: Pearl>(&mut self) -> Option<IterMut<P>> {
        self.source.iter_mut()
    }

    pub fn insert<P: Pearl>(&mut self, pearl: P) -> Handle<P> {
        self.source.insert(pearl)
    }

    pub fn queue_destroy<P: Pearl>(&mut self, handle: Handle<P>) {
        let raw = handle.into_raw();
        use indexmap::map::Entry as E;
        match self.destroy_queue.entry(P::id()) {
            E::Occupied(e) => e.into_mut().push(raw),
            E::Vacant(e) => {
                e.insert(vec![raw]);
            }
        }
    }

    pub fn current(&self) -> &PearlEntry<T> {
        let anymap = self.source.anymap.pearl_maps[self.map_index].as_any();
        let pearl_map = anymap.downcast_ref::<PearlMap<T>>();
        let pearl_map = pearl_map.expect("Internal Error: Faulty downcast");
        &pearl_map.pearls[self.pearl_index]
    }

    pub fn current_mut(&mut self) -> &mut PearlEntry<T> {
        let anymap = self.source.anymap.pearl_maps[self.map_index].as_any_mut();
        let pearl_map = anymap.downcast_mut::<PearlMap<T>>();
        let pearl_map = pearl_map.expect("Internal Error: Faulty downcast");
        &mut pearl_map.pearls[self.pearl_index]
    }
}

type EventRunner<E> = fn(&mut E, &mut PearlArena, &mut Resources);

#[derive(Default)]
struct ArenaEventRegistry {
    index_map: FxHashMap<TypeId, usize>,
    runners: Vec<Box<dyn AnyRunnerVec>>,
}

impl<P: Pearl> EventRegister<P> for ArenaEventRegistry {
    fn event<E: Event>(&mut self)
    where
        P: EventListener<E>,
    {
        use std::collections::hash_map::Entry;
        match self.index_map.entry(TypeId::of::<E>()) {
            Entry::Occupied(e) => {
                let anyvec = self.runners[*e.get()].as_any_mut();
                let runnervec = anyvec.downcast_mut::<Vec<EventRunner<E>>>();
                let runnervec = runnervec.expect("Internal Error: Faulty downcast");
                runnervec.push(Self::runner::<E, P>);
            }
            Entry::Vacant(e) => {
                e.insert(self.runners.len());
                let runnervec: Vec<EventRunner<E>> = vec![Self::runner::<E, P>];
                self.runners.push(Box::new(runnervec));
            }
        }
    }
}

impl ArenaEventRegistry {
    fn runner<E: Event, P: EventListener<E>>(
        event: &mut E,
        pearls: &mut PearlArena,
        resources: &mut Resources,
    ) {
        let Some(map_index) = pearls.anymap.map_index.get(&P::id()) else {
            return;
        };

        let Some(mut arena_view) = PearlArenaView::<P>::create_view(*map_index, pearls) else {
            return;
        };

        P::update(event, &mut arena_view, resources);
        while arena_view.next_view() {
            P::update(event, &mut arena_view, resources);
        }
    }
}

trait AnyRunnerVec: Any {
    fn len(&self) -> usize;
    fn is_empty(&self) -> bool;
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

impl<E: Event> AnyRunnerVec for Vec<EventRunner<E>> {
    fn len(&self) -> usize {
        self.len()
    }

    fn is_empty(&self) -> bool {
        self.is_empty()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}
