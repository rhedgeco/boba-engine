use crate::{
    pearl::{
        Handle, IntoIter, Iter, PearlEntry, PearlExt, PearlId, PearlMap, PearlMut, PearlRef,
        UntypedPearlMap,
    },
    unique::{UniqueMut, UniquePearlMap, UniqueRef},
    Event, EventListener, EventRegister, Pearl,
};
use indexmap::IndexMap;
use std::{any::TypeId, collections::HashSet};

#[derive(Default)]
pub struct BobaWorld {
    static_pearls: UniquePearlMap,
    global_pearls: UniquePearlMap,
    pearl_maps: IndexMap<PearlId, Box<dyn UntypedPearlMap>>,
    event_registry: EventRegistry,
}

impl BobaWorld {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn len<P: Pearl>(&self) -> usize {
        match self.get_map::<P>() {
            Some(map) => map.len(),
            None => 0,
        }
    }

    pub fn get<P: Pearl>(&self, handle: Handle<P>) -> Option<PearlRef<P>> {
        self.get_map()?.get(handle)
    }

    pub fn get_global<P: Pearl>(&self) -> Option<UniqueRef<P>> {
        self.global_pearls.get()
    }

    pub fn get_static<P: Pearl>(&self) -> Option<UniqueRef<P>> {
        self.static_pearls.get()
    }

    pub fn get_mut<P: Pearl>(&self, handle: Handle<P>) -> Option<PearlMut<P>> {
        self.get_map()?.get_mut(handle)
    }

    pub fn get_global_mut<P: Pearl>(&self) -> Option<UniqueMut<P>> {
        self.global_pearls.get_mut()
    }

    pub fn get_static_mut<P: Pearl>(&self) -> Option<UniqueMut<P>> {
        self.static_pearls.get_mut()
    }

    pub fn get_where<P: Pearl>(&self, f: impl Fn(&PearlRef<P>) -> bool) -> Option<PearlRef<P>> {
        self.get_map()?.get_where(f)
    }

    pub fn get_mut_where<P: Pearl>(&self, f: impl Fn(&PearlMut<P>) -> bool) -> Option<PearlMut<P>> {
        self.get_map()?.get_mut_where(f)
    }

    pub fn insert<P: Pearl>(&mut self, pearl: P) -> Handle<P> {
        self.get_or_create_map().insert(pearl)
    }

    pub fn insert_global<P: Pearl>(&mut self, pearl: P) -> Option<P> {
        let pearl = self.global_pearls.insert(pearl);
        if pearl.is_none() {
            P::register(&mut self.event_registry);
        }
        pearl
    }

    pub fn insert_static<P: Pearl>(&mut self, pearl: P) -> Option<P> {
        let pearl = self.static_pearls.insert(pearl);
        if pearl.is_none() {
            P::register(&mut self.event_registry);
        }
        pearl
    }

    pub fn insert_callback<E: Event>(&mut self, callback: EventRunner<E>) {
        self.event_registry.insert_callback(callback);
    }

    pub fn remove<P: Pearl>(&mut self, handle: Handle<P>) -> Option<P> {
        self.get_map_mut()?.remove(handle)
    }

    pub fn remove_where<P: Pearl>(&mut self, f: impl Fn(&PearlRef<P>) -> bool) -> Option<P> {
        self.get_map_mut()?.remove_where(f)
    }

    pub fn remove_global<P: Pearl>(&mut self) -> Option<P> {
        self.global_pearls.remove::<P>()
    }

    pub fn iter<P: Pearl>(&self) -> Iter<P> {
        match self.get_map() {
            Some(map) => map.iter(),
            None => {
                let empty: &[PearlEntry<P>] = &[];
                empty.iter()
            }
        }
    }

    pub fn into_iter<P: Pearl>(&mut self) -> IntoIter<P> {
        let Some(map) = self.get_map_mut::<P>() else {
            return IntoIter::empty();
        };

        core::mem::replace(map, PearlMap::new()).into_iter()
    }

    pub fn trigger<'a, E: Event>(&mut self, mut event: E::Data<'a>) {
        let Some(event_index) = self
            .event_registry
            .event_runners
            .get_index_of(&TypeId::of::<E>())
        else {
            return;
        };

        let mut runner_index = 0;
        while runner_index < self.event_registry.event_runners[event_index].len() {
            let runner = &self.event_registry.event_runners[event_index][runner_index];
            let runner = unsafe { runner.cast::<E>() };
            runner(&mut event, self);
            runner_index += 1;
        }
    }

    fn get_map<P: Pearl>(&self) -> Option<&PearlMap<P>> {
        self.pearl_maps.get(&P::id())?.as_any().downcast_ref()
    }

    fn get_map_mut<P: Pearl>(&mut self) -> Option<&mut PearlMap<P>> {
        self.pearl_maps
            .get_mut(&P::id())?
            .as_any_mut()
            .downcast_mut()
    }

    fn get_or_create_map<P: Pearl>(&mut self) -> &mut PearlMap<P> {
        use indexmap::map::Entry as E;
        let anymap = match self.pearl_maps.entry(P::id()) {
            E::Occupied(e) => e.into_mut(),
            E::Vacant(e) => {
                P::register(&mut self.event_registry);
                e.insert(Box::new(PearlMap::<P>::new()))
            }
        };

        let map = anymap.as_any_mut().downcast_mut();
        map.expect("Internal Error: Faulty downcast")
    }
}

type EventRunner<E> = for<'a> fn(&mut <E as Event>::Data<'a>, &mut BobaWorld);

struct RunnerStore {
    ptr: *const (),
}

impl RunnerStore {
    fn new<E: Event>(runner: EventRunner<E>) -> Self {
        Self {
            ptr: runner as *const (),
        }
    }

    unsafe fn cast<E: Event>(&self) -> EventRunner<E> {
        unsafe { core::mem::transmute::<*const (), EventRunner<E>>(self.ptr) }
    }
}

#[derive(Default)]
struct EventRegistry {
    event_pairs: HashSet<(TypeId, PearlId)>,
    event_runners: IndexMap<TypeId, Vec<RunnerStore>>,
}

impl EventRegistry {
    fn insert_callback<E: Event>(&mut self, callback: EventRunner<E>) {
        use indexmap::map::Entry;
        match self.event_runners.entry(TypeId::of::<E>()) {
            Entry::Occupied(e) => e.into_mut(),
            Entry::Vacant(e) => e.insert(Vec::new()),
        }
        .push(RunnerStore::new(callback));
    }
}

impl<P: Pearl> EventRegister<P> for EventRegistry {
    #[doc(hidden)]
    fn event<E: Event>(&mut self)
    where
        P: EventListener<E>,
    {
        if self.event_pairs.insert((TypeId::of::<E>(), P::id())) {
            self.insert_callback(P::update);
        }
    }
}
