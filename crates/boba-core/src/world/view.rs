use std::ops::{Deref, DerefMut};

use crate::{Pearl, World};

use super::{Link, WorldAccess, WorldInsert};

pub struct PearlView<'a, P: Pearl> {
    world: &'a mut World,
    link: Link<P>,
}

impl<'a, P: Pearl> DerefMut for PearlView<'a, P> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.world.get_mut(self.link).expect("invalid view")
    }
}

impl<'a, P: Pearl> Deref for PearlView<'a, P> {
    type Target = P;

    fn deref(&self) -> &Self::Target {
        self.world.get(self.link).expect("invalid view")
    }
}

impl<'a, P: Pearl> PearlView<'a, P> {
    pub fn new(link: Link<P>, world: &'a mut World) -> Option<Self> {
        match world.contains(link) {
            false => None,
            true => Some(Self { world, link }),
        }
    }

    pub fn link(&self) -> Link<P> {
        self.link
    }

    pub fn world(&self) -> &(impl WorldAccess + WorldInsert) {
        self.world
    }

    pub fn world_mut(&mut self) -> &mut (impl WorldAccess + WorldInsert) {
        self.world
    }

    pub fn defer_destroy_self(&mut self) -> bool {
        self.world.defer_destroy(self.link)
    }
}
