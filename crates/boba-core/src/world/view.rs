use std::ops::{Deref, DerefMut};

use indexmap::IndexMap;

use crate::{Pearl, World};

use super::{Link, WorldAccess, WorldInsert, WorldRemove};

pub struct PearlView<'a, P: Pearl> {
    destroy: IndexMap<Link<()>, fn(Link<()>, &mut World)>,
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

impl<'a, P: Pearl> Drop for PearlView<'a, P> {
    fn drop(&mut self) {
        for (link, func) in self.destroy.iter() {
            func(*link, self.world);
        }
    }
}

impl<'a, P: Pearl> PearlView<'a, P> {
    pub fn new(link: Link<P>, world: &'a mut World) -> Option<Self> {
        match world.contains(link) {
            false => None,
            true => Some(Self {
                world,
                link,
                destroy: IndexMap::new(),
            }),
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

    pub fn queue_destroy(&mut self, link: Link<P>) -> bool {
        match self.world.contains(link) {
            false => false,
            true => self
                .destroy
                .insert(link.into_type(), |link, world| {
                    world.remove(link.into_type::<P>());
                })
                .is_none(),
        }
    }
}
