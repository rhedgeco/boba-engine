use crate::{
    pearl::{
        Handle, Iter, PearlEntry, PearlExt, PearlId, PearlMap, PearlMut, PearlRef, UntypedPearlMap,
    },
    Event, EventListener, EventRegister, Pearl,
};
use indexmap::IndexMap;
use std::{
    any::{Any, TypeId},
    cell::{Ref, RefCell, RefMut},
    collections::HashSet,
    ops::{Deref, DerefMut},
};

pub struct GlobalEntry<P: Pearl> {
    pearl: RefCell<P>,
}

impl<P: Pearl> GlobalEntry<P> {
    pub(crate) fn new(pearl: P) -> Self {
        Self {
            pearl: RefCell::new(pearl),
        }
    }
}

impl<P: Pearl> GlobalEntry<P> {
    pub fn borrow(&self) -> Option<GlobalRef<P>> {
        let pearl = self.pearl.try_borrow().ok()?;
        Some(GlobalRef { pearl })
    }

    pub fn borrow_mut(&self) -> Option<GlobalMut<P>> {
        let pearl = self.pearl.try_borrow_mut().ok()?;
        Some(GlobalMut { pearl })
    }
}

pub struct GlobalRef<'a, P: Pearl> {
    pearl: Ref<'a, P>,
}

impl<'a, P: Pearl> Deref for GlobalRef<'a, P> {
    type Target = P;

    fn deref(&self) -> &Self::Target {
        self.pearl.deref()
    }
}

pub struct GlobalMut<'a, P: Pearl> {
    pearl: RefMut<'a, P>,
}

impl<'a, P: Pearl> DerefMut for GlobalMut<'a, P> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.pearl.deref_mut()
    }
}

impl<'a, P: Pearl> Deref for GlobalMut<'a, P> {
    type Target = P;

    fn deref(&self) -> &Self::Target {
        self.pearl.deref()
    }
}

#[derive(Default)]
pub struct BobaWorld {
    global_pearls: IndexMap<PearlId, Box<dyn Any>>,
    pearl_maps: IndexMap<PearlId, Box<dyn UntypedPearlMap>>,
    event_registry: EventRegistry,
}

impl BobaWorld {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get<P: Pearl>(&self, handle: Handle<P>) -> Option<PearlRef<P>> {
        self.get_map()?.get(handle)
    }

    pub fn get_global<P: Pearl>(&self) -> Option<GlobalRef<P>> {
        let any = self.global_pearls.get(&P::id())?;
        let global = any.downcast_ref::<GlobalEntry<P>>();
        let global = global.expect("Internal Error: Faulty downcast");
        global.borrow()
    }

    pub fn get_mut<P: Pearl>(&self, handle: Handle<P>) -> Option<PearlMut<P>> {
        self.get_map()?.get_mut(handle)
    }

    pub fn get_global_mut<P: Pearl>(&self) -> Option<GlobalMut<P>> {
        let any = self.global_pearls.get(&P::id())?;
        let global = any.downcast_ref::<GlobalEntry<P>>();
        let global = global.expect("Internal Error: Faulty downcast");
        global.borrow_mut()
    }

    pub fn insert<P: Pearl>(&mut self, pearl: P) -> Handle<P> {
        let handle = self.get_or_create_map().insert(pearl);
        P::on_insert(handle, self);
        handle
    }

    pub fn insert_global<P: Pearl>(&mut self, pearl: P) -> Option<P> {
        let global = Box::new(GlobalEntry::new(pearl));
        let any = self.global_pearls.insert(P::id(), global);
        P::on_insert_global(self);
        *any?.downcast().expect("Internal Error: Faulty downcast")
    }

    pub fn insert_callback<E: Event>(&mut self, callback: EventRunner<E>) {
        self.event_registry.insert_callback(callback);
    }

    pub fn remove<P: Pearl>(&mut self, handle: Handle<P>) -> Option<P> {
        let mut pearl = self.get_map_mut()?.remove(handle)?;
        pearl.on_remove(self);
        Some(pearl)
    }

    pub fn remove_global<P: Pearl>(&mut self) -> Option<P> {
        let any = self.global_pearls.remove(&P::id())?;
        let pearl: GlobalEntry<P> = *any.downcast().expect("Internal Error: Faulty downcast");
        let mut pearl = pearl.pearl.into_inner();
        pearl.on_remove_global(self);
        Some(pearl)
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
