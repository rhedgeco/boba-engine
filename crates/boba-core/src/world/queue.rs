use indexmap::IndexSet;

use crate::{pearl::Event, Pearl, World};

use super::{Iter, IterMut, Link, Links, LinksCopied, PearlView, Pearls, PearlsMut};

/// A wrapper around a [`World`] that preserves all current links connections.
///
/// This provides the ability to queue operations that will be done when dropped.
/// This means that any queued operations (such as removals) will be deferred.
pub struct WorldQueue<'a> {
    pub(crate) world: &'a mut World,
    queue: Vec<Box<dyn FnOnce(&mut World)>>,
    destroy: IndexSet<Link<()>>,
}

impl<'a> Drop for WorldQueue<'a> {
    fn drop(&mut self) {
        // execute all operations in the queue
        for action in self.queue.drain(..) {
            action(self.world);
        }
    }
}

impl<'a> WorldQueue<'a> {
    pub fn new(world: &'a mut World) -> Self {
        Self {
            world,
            queue: Vec::new(),
            destroy: IndexSet::new(),
        }
    }

    pub fn types(&self) -> usize {
        self.world.types()
    }

    pub fn is_empty(&self) -> bool {
        self.world.is_empty()
    }

    pub fn len<P: Pearl>(&self) -> usize {
        self.world.len::<P>()
    }

    pub fn has<P: Pearl>(&self) -> bool {
        self.world.has::<P>()
    }

    pub fn contains<P: Pearl>(&self, link: Link<P>) -> bool {
        self.world.contains(link)
    }

    pub fn get<P: Pearl>(&self, link: Link<P>) -> Option<&P> {
        self.world.get(link)
    }

    pub fn get_mut<P: Pearl>(&mut self, link: Link<P>) -> Option<&mut P> {
        self.world.get_mut(link)
    }

    pub fn insert<P: Pearl>(&mut self, pearl: P) -> Link<P> {
        self.world.insert(pearl)
    }

    pub fn insert_then<P: Pearl>(&mut self, pearl: P, then: impl FnOnce(PearlView<P>)) -> Link<P> {
        self.world.insert_then(pearl, then)
    }

    pub fn links<P: Pearl>(&self) -> Links<P> {
        self.world.links()
    }

    pub fn links_copied<P: Pearl>(&self) -> LinksCopied<P> {
        self.world.links_copied()
    }

    pub fn pearls<P: Pearl>(&self) -> Pearls<P> {
        self.world.pearls()
    }

    pub fn pearls_mut<P: Pearl>(&mut self) -> PearlsMut<P> {
        self.world.pearls_mut()
    }

    pub fn iter<P: Pearl>(&self) -> Iter<P> {
        self.world.iter()
    }

    pub fn iter_mut<P: Pearl>(&mut self) -> IterMut<P> {
        self.world.iter_mut()
    }

    pub fn trigger<E: Event>(&mut self, data: &mut E) {
        World::trigger_nested::<E>(self, data);
    }

    pub fn defer(&mut self, f: impl FnOnce(&mut World) + 'static) {
        self.queue.push(Box::new(f));
    }

    pub fn destroy<P: Pearl>(&mut self, link: Link<P>) -> bool {
        // fail if the pearl does not exist
        if !self.world.contains(link) {
            return false;
        }

        // fail if the pearls is already queued
        if !self.destroy.insert(link.into_type()) {
            return false;
        }

        // defer the removal
        self.defer(move |world| {
            world.remove(link);
        });

        true
    }
}
