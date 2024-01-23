use std::ops::{Deref, DerefMut};

use crate::World;

use super::Link;

/// A view into a single [`Pearl`](crate::Pearl) in a [`World`]
pub struct View<'a, P> {
    world: &'a World,
    pearl: *mut P,
}

impl<P> DerefMut for View<'_, P> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.pearl }
    }
}

impl<P> Deref for View<'_, P> {
    type Target = P;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.pearl }
    }
}

impl<'a, P> View<'a, P> {
    pub(super) unsafe fn new(world: &'a World, pearl: *mut P) -> Self {
        Self { world, pearl }
    }

    pub fn world(&self) -> &World {
        self.world
    }

    pub fn to<P2: 'static>(&mut self, link: Link<P2>) -> Option<View<P2>> {
        let pearl_ptr = self.world.get(link)? as *const P2 as *mut P2;
        Some(unsafe { View::new(self.world, pearl_ptr) })
    }
}
