use crate::{
    pearl::{
        Handle, Iter, IterMut, PearlEntry, PearlExt, PearlId, PearlMap, RawHandle, UntypedPearlMap,
    },
    Event, EventListener, EventRegister, Pearl, Resources,
};
use fxhash::{FxBuildHasher, FxHashMap};
use indexmap::IndexMap;
use std::{
    any::{Any, TypeId},
    marker::PhantomData,
};

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
pub struct ArenaPearls {
    map_index: FxHashMap<PearlId, usize>,
    pearl_maps: Vec<Box<dyn UntypedPearlMap>>,
}

impl ArenaPearls {
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
pub struct BobaArena {
    events: ArenaEventRegistry,
    anymap: ArenaPearls,
    resources: Resources,
}

impl BobaArena {
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

    pub fn resources(&self) -> &Resources {
        &self.resources
    }

    pub fn resources_mut(&mut self) -> &mut Resources {
        &mut self.resources
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

    pub fn trigger<E: Event>(&mut self, event: &mut E) {
        let Some(vec_index) = self.events.index_map.get(&TypeId::of::<E>()).map(|i| *i) else {
            return;
        };

        let mut runner_index = 0;
        while runner_index < self.events.runners[vec_index].len() {
            let runner = self.events.runners[vec_index]
                .as_any()
                .downcast_ref::<Vec<EventRunner<E>>>()
                .expect("Internal Error: Faulty downcast")[runner_index];

            runner(event, self);
            runner_index += 1;
        }
    }
}

pub struct ArenaView<'a, P: Pearl> {
    map_index: usize,
    pearl_index: usize,
    source: &'a mut BobaArena,
    destroy_queue: IndexMap<PearlId, Vec<RawHandle>, FxBuildHasher>,
    _type: PhantomData<*const P>,
}

impl<'a, T: Pearl> ArenaView<'a, T> {
    fn create_view(map_index: usize, source: &'a mut BobaArena) -> Option<Self> {
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

    fn next_view(mut self) -> Option<Self> {
        self.pearl_index += 1;
        let pearl_map = &self.source.anymap.pearl_maps[self.map_index];
        if self.pearl_index >= pearl_map.len() {
            for (id, handles) in self.destroy_queue {
                let Some(map_index) = self.source.anymap.map_index.get(&id) else {
                    continue;
                };
                let pearl_map = &mut self.source.anymap.pearl_maps[*map_index];
                for raw_handle in handles {
                    pearl_map.destroy(raw_handle);
                }
            }

            return None;
        }

        Some(self)
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

    pub fn trigger<E: Event>(&mut self, event: &mut E) {
        self.source.trigger(event);
    }

    pub fn resources(&self) -> &Resources {
        self.source.resources()
    }

    pub fn resources_mut(&mut self) -> &mut Resources {
        self.source.resources_mut()
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

    pub fn current_pearl(&self) -> &PearlEntry<T> {
        let anymap = self.source.anymap.pearl_maps[self.map_index].as_any();
        let pearl_map = anymap.downcast_ref::<PearlMap<T>>();
        let pearl_map = pearl_map.expect("Internal Error: Faulty downcast");
        &pearl_map.pearls[self.pearl_index]
    }

    pub fn current_pearl_mut(&mut self) -> &mut PearlEntry<T> {
        let anymap = self.source.anymap.pearl_maps[self.map_index].as_any_mut();
        let pearl_map = anymap.downcast_mut::<PearlMap<T>>();
        let pearl_map = pearl_map.expect("Internal Error: Faulty downcast");
        &mut pearl_map.pearls[self.pearl_index]
    }
}

type EventRunner<E> = fn(&mut E, &mut BobaArena);

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
    fn runner<E: Event, P: EventListener<E>>(event: &mut E, pearls: &mut BobaArena) {
        let Some(map_index) = pearls.anymap.map_index.get(&P::id()) else {
            return;
        };

        let Some(mut arena_view) = ArenaView::<P>::create_view(*map_index, pearls) else {
            return;
        };

        loop {
            P::update(event.event_data(), &mut arena_view);
            match arena_view.next_view() {
                Some(next) => arena_view = next,
                None => break,
            }
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
