use crate::{
    event::EventRegistry,
    pearl::map::{Handle, MultiPearlMap},
    resources::Resource,
    Event, Pearl, Resources,
};

/// The central data structure of `boba-core` that holds all the `Pearl` structs and resources.
#[derive(Default)]
pub struct World {
    events: EventRegistry,
    pearls: MultiPearlMap,
    resources: Resources,
}

impl World {
    /// Returns a new empty world.
    pub fn new() -> Self {
        Self::default()
    }

    /// Inserts [`Pearl`] into this world, returning a [`Handle`] to its location.
    ///
    /// The pearl will be instantly registered with the worlds event system.
    pub fn insert_pearl<P: Pearl>(&mut self, pearl: P) -> Handle<P> {
        P::register(&mut self.events);
        self.pearls.insert_now(pearl)
    }

    pub fn get_pearl<P: Pearl>(&self, handle: Handle<P>) -> Option<&P> {
        self.pearls.get(handle)
    }

    pub fn get_pearl_mut<P: Pearl>(&mut self, handle: Handle<P>) -> Option<&mut P> {
        self.pearls.get_mut(handle)
    }

    /// Removes a [`Pearl`] if it exists from the world using its [`Handle`].
    pub fn remove_pearl<P: Pearl>(&mut self, handle: Handle<P>) -> Option<P> {
        self.pearls.remove_now(handle)
    }

    /// Inserts or replaces a global [`Resource`] into this world.
    ///
    /// If a resource of this type already exists, the old one will be returned.
    pub fn insert_resource<R: Resource>(&mut self, resource: R) -> Option<R> {
        self.resources.insert(resource)
    }

    pub fn get_resource<R: Resource>(&self) -> Option<&R> {
        self.resources.get()
    }

    pub fn get_resource_mut<R: Resource>(&mut self) -> Option<&mut R> {
        self.resources.get_mut()
    }

    /// Removes and returns a global [`Resource`] into this world.
    ///
    /// Returns `None` if the resource does not exist
    pub fn remove_resource<R: Resource>(&mut self) -> Option<R> {
        self.resources.remove()
    }

    /// Triggers an [`Event`] in this world.
    /// Any [`Pearl`] that is registered to listen for these events will be updated.
    pub fn trigger<E: Event>(&mut self, event: &mut E) {
        self.events
            .trigger::<E>(event, &mut self.pearls, &mut self.resources);
    }
}
