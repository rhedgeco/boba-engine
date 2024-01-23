use std::ops::{Deref, DerefMut};

use crate::{Pearl, World};

use super::Link;

/// A view into a single [`Pearl`](crate::Pearl) in a [`World`]
pub struct View<'a, P: Pearl> {
    world: &'a World,
    pearl_ptr: *mut P,
}

impl<P: Pearl> DerefMut for View<'_, P> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.pearl_ptr }
    }
}

impl<P: Pearl> Deref for View<'_, P> {
    type Target = P;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.pearl_ptr }
    }
}

impl<P: Pearl> Drop for View<'_, P> {
    fn drop(&mut self) {
        P::on_view_drop(self);
    }
}

impl<'a, P: Pearl> View<'a, P> {
    pub(super) unsafe fn new(world: &'a World, pearl_ptr: *mut P) -> Self {
        Self { world, pearl_ptr }
    }

    pub fn world(&self) -> &World {
        self.world
    }

    pub fn to_pearl<P2: Pearl>(&mut self, link: Link<P2>) -> Option<View<P2>> {
        let pearl_ptr = self.world.get(link)? as *const P2 as *mut P2;
        Some(unsafe { View::new(self.world, pearl_ptr) })
    }
}
