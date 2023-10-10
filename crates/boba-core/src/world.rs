use crate::{
    pearl::{Handle, Iter, IterMut, PearlEntry, PearlExt, PearlId, PearlMap, UntypedPearlMap},
    Event, EventListener, EventRegister, Pearl,
};
use indexmap::IndexMap;
use std::{
    any::{Any, TypeId},
    collections::HashSet,
};

#[derive(Default)]
pub struct BobaWorld {
    resources: IndexMap<TypeId, Box<dyn Any>>,
    pearl_maps: IndexMap<PearlId, Box<dyn UntypedPearlMap>>,
    event_registry: EventRegistry,
}

impl BobaWorld {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get<P: Pearl>(&mut self, handle: Handle<P>) -> Option<&P> {
        self.get_map()?.get(handle)
    }

    pub fn get_resource<R: 'static>(&self) -> Option<&R> {
        self.resources.get(&TypeId::of::<R>())?.downcast_ref()
    }

    pub fn get_mut<P: Pearl>(&mut self, handle: Handle<P>) -> Option<&mut P> {
        self.get_map_mut()?.get_mut(handle)
    }

    pub fn get_resource_mut<R: 'static>(&mut self) -> Option<&mut R> {
        self.resources.get_mut(&TypeId::of::<R>())?.downcast_mut()
    }

    pub fn insert<P: Pearl>(&mut self, pearl: P) -> Handle<P> {
        self.get_or_create_map().insert(pearl)
    }

    pub fn insert_resource<R: 'static>(&mut self, resource: R) -> Option<R> {
        let id = TypeId::of::<R>();
        let any = self.resources.insert(id, Box::new(resource))?;
        *any.downcast().expect("Internal Error: Faulty downcast")
    }

    pub fn insert_callback<E: Event>(&mut self, callback: EventRunner<E>) {
        self.event_registry.insert_callback(callback);
    }

    pub fn remove<P: Pearl>(&mut self, handle: Handle<P>) -> Option<P> {
        self.get_map_mut()?.remove(handle)
    }

    pub fn remove_resource<R: 'static>(&mut self) -> Option<R> {
        let any = self.resources.remove(&TypeId::of::<R>())?;
        *any.downcast().expect("Internal Error: Faulty downcast")
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

    pub fn iter_mut<P: Pearl>(&mut self) -> IterMut<P> {
        match self.get_map_mut() {
            Some(map) => map.iter_mut(),
            None => {
                let empty: &mut [PearlEntry<P>] = &mut [];
                empty.iter_mut()
            }
        }
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
    pearl_types: HashSet<PearlId>,
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
        if self.pearl_types.insert(P::id()) {
            self.insert_callback(P::update);
        }
    }
}
