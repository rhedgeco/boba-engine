use std::ops::{Deref, DerefMut};

use crate::Pearl;

use super::{Link, WorldQueue};

pub struct PearlView<'a, 'world, P: Pearl> {
    world: &'a mut WorldQueue<'world>,
    link: Link<P>,
}

impl<'a, 'world, P: Pearl> DerefMut for PearlView<'a, 'world, P> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.world.get_mut(self.link).expect("invalid view link")
    }
}

impl<'a, 'world, P: Pearl> Deref for PearlView<'a, 'world, P> {
    type Target = P;

    fn deref(&self) -> &Self::Target {
        self.world.get(self.link).expect("invalid view link")
    }
}

impl<'a, 'world, P: Pearl> PearlView<'a, 'world, P> {
    pub fn new(link: Link<P>, world: &'a mut WorldQueue<'world>) -> Option<Self> {
        match world.contains(link) {
            true => Some(Self { world, link }),
            false => None,
        }
    }

    pub fn new_unchecked(link: Link<P>, world: &'a mut WorldQueue<'world>) -> Self {
        debug_assert!(world.contains(link));
        Self { world, link }
    }

    pub fn link(&self) -> Link<P> {
        self.link
    }

    pub fn world(&self) -> &WorldQueue<'world> {
        &self.world
    }

    pub fn world_mut(&mut self) -> &mut WorldQueue<'world> {
        &mut self.world
    }

    pub fn get_view<'b, P2: Pearl>(
        &'b mut self,
        link: Link<P2>,
    ) -> Option<PearlView<'b, 'world, P2>> {
        PearlView::new(link, self.world)
    }

    pub fn destroy_self(&mut self) -> bool {
        self.world.destroy(self.link)
    }
}
