use std::{
    ops::{Deref, DerefMut},
    slice::IterMut,
};

use crate::{Pearl, World};

use super::Link;

/// Walks through, in order, each [`Pearl`] of type `P` that exists in a [`World`].
pub struct ViewWalker<'a, P> {
    pearl_iter: IterMut<'a, P>,
    world: *mut World,
}

impl<'a, P: Pearl> ViewWalker<'a, P> {
    /// Returns a new view walker over the pearls in `world`.
    ///
    /// Returns `None` if there are no pearls of that type in the world.
    pub fn new(world: &'a mut World) -> Option<Self> {
        Some(Self {
            world: world as *mut World,
            pearl_iter: world.iter_mut()?,
        })
    }

    /// Returns the next pearl in the world.
    ///
    /// This is explicitly different than an iterator as items may only be accessed
    /// one at a time, and the previous one must be dropped before the next one can be provided
    pub fn walk_next(&mut self) -> Option<View<P>> {
        let pearl = self.pearl_iter.next()?;
        Some(View {
            pearl,
            world: self.world,
        })
    }
}

/// A view into a single [`Pearl`](crate::Pearl) in a [`World`]
pub struct View<'a, P: Pearl> {
    pearl: &'a mut P,
    world: *mut World,
}

impl<P: Pearl> DerefMut for View<'_, P> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.pearl
    }
}

impl<P: Pearl> Deref for View<'_, P> {
    type Target = P;

    fn deref(&self) -> &Self::Target {
        self.pearl
    }
}

impl<P: Pearl> Drop for View<'_, P> {
    fn drop(&mut self) {
        P::on_view_drop(self);
    }
}

impl<'a, P: Pearl> View<'a, P> {
    /// Returns a view of the [`Pearl`] associated with `link` within the `world`.
    ///
    /// Returns `None` if the link is invalid for the given world.
    pub fn new(world: &'a mut World, link: Link<P>) -> Option<Self> {
        Some(Self {
            world: world as *mut World,
            pearl: world.get_mut(link)?,
        })
    }

    /// Returns `true` if `link` is valid for the source world of this view.
    pub fn world_contains<P2: Pearl>(&self, link: Link<P2>) -> bool {
        // SAFETY: the `contains`` method does not alias any pearls
        unsafe { &*self.world }.contains(link)
    }

    /// Returns a new view of the [`Pearl`] associated with `link` in the source world.
    ///
    /// Returns `None` if the link is invalid for the source world.
    pub fn view<P2: Pearl>(&mut self, link: Link<P2>) -> Option<View<P2>> {
        Some(View {
            pearl: unsafe { &mut *self.world }.get_mut(link)?,
            world: self.world,
        })
    }
}
