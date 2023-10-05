use std::any::{Any, TypeId};

use fxhash::{FxHashMap, FxHashSet};

use crate::{
    pearl::{
        map::{MultiPearlMap, PearlAccess},
        PearlExt, PearlId,
    },
    Pearl,
};

/// Marker trait that designates a struct can be used to trigger a [`World`](crate::World) event.
pub trait Event: 'static {}
impl<T: 'static> Event for T {}

pub trait EventListener<E: Event>: Pearl {
    fn update(event: &mut E, pearls: &mut PearlAccess<Self>);
}

pub trait EventRegister<P: Pearl> {
    fn event<E: Event>(&mut self)
    where
        P: EventListener<E>;
}

type EventRunner<E> = fn(&mut E, &mut MultiPearlMap);

#[derive(Default)]
pub struct EventRegistry {
    pearls: FxHashSet<PearlId>,
    runners: FxHashMap<TypeId, Vec<Box<dyn Any>>>,
}

impl<P: Pearl> EventRegister<P> for EventRegistry {
    fn event<E: Event>(&mut self)
    where
        P: EventListener<E>,
    {
        if !self.pearls.insert(P::id()) {
            return;
        }

        use std::collections::hash_map::Entry;
        let runners = match self.runners.entry(TypeId::of::<E>()) {
            Entry::Occupied(e) => e.into_mut(),
            Entry::Vacant(e) => e.insert(Vec::new()),
        };

        // assert that the function is the correct type using explicit typing
        let runner: EventRunner<E> = EventRegistry::runner::<E, P>;
        runners.push(Box::new(runner))
    }
}

impl EventRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn trigger<E: Event>(&mut self, event: &mut E, map: &mut MultiPearlMap) {
        let Some(runners) = self.runners.get(&TypeId::of::<E>()) else {
            return;
        };

        for any in runners {
            let runner = any.downcast_ref::<EventRunner<E>>();
            runner.expect("Internal Error: Faulty downcast")(event, map);
        }
    }

    fn runner<E: Event, P: Pearl + EventListener<E>>(data: &mut E, map: &mut MultiPearlMap) {
        let Some(mut stream) = map.stream::<P>() else {
            return;
        };

        while let Some(mut access) = stream.next_access() {
            P::update(data, &mut access)
        }
    }
}
