use std::collections::HashMap;

use derive_more::Display;
use fxhash::{FxBuildHasher, FxHashMap};

use crate::{
    Component, UniqueId,
    component::{ComponentDef, ComponentId, ComponentStore},
    store::SparseStore,
};

#[repr(transparent)]
#[derive(Debug, Display, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EntityId(UniqueId);

#[derive(Debug, Clone, Copy)]
struct EntityLink {
    sparse_index: usize,
}

#[derive(Debug, Clone, Copy)]
enum ComponentLink {
    Sparse(usize),
}

pub struct World {
    entity_links: FxHashMap<EntityId, EntityLink>,
    component_links: FxHashMap<UniqueId, ComponentLink>,
    sparse_store: SparseStore,
}

impl World {
    pub const fn new() -> Self {
        Self {
            entity_links: HashMap::with_hasher(FxBuildHasher::new()),
            component_links: HashMap::with_hasher(FxBuildHasher::new()),
            sparse_store: SparseStore::new(),
        }
    }

    pub fn entity(&self, id: EntityId) -> Option<EntityRef> {
        Some(EntityRef {
            link: *self.entity_links.get(&id)?,
            world: self,
        })
    }

    pub fn entity_mut(&mut self, id: EntityId) -> Option<EntityMut> {
        Some(EntityMut {
            link: *self.entity_links.get(&id)?,
            world: self,
        })
    }

    pub fn component<T: 'static>(&self, id: ComponentId<T>) -> Option<&Component<T>> {
        match self.component_links.get(&id.into_raw())? {
            ComponentLink::Sparse(index) => self.sparse_store.get(*index),
        }
    }

    pub fn component_mut<T: 'static>(&mut self, id: ComponentId<T>) -> Option<&mut Component<T>> {
        match self.component_links.get(&id.into_raw())? {
            ComponentLink::Sparse(index) => self.sparse_store.get_mut(*index),
        }
    }

    pub fn spawn(&mut self, f: impl FnOnce(EntityMut)) -> EntityId {
        let link = EntityLink {
            sparse_index: self.sparse_store.spawn(),
        };

        f(EntityMut { world: self, link });

        let id = EntityId(UniqueId::new());
        self.entity_links.insert(id, link);
        id
    }
}

pub struct EntityRef<'a> {
    world: &'a World,
    link: EntityLink,
}

impl<'a> EntityRef<'a> {
    pub fn component<T: 'static>(&self) -> Option<&Component<T>> {
        self.world.sparse_store.get(self.link.sparse_index)
    }
}

pub struct EntityMut<'a> {
    world: &'a mut World,
    link: EntityLink,
}

impl<'a> EntityMut<'a> {
    pub fn component<T: 'static>(&self) -> Option<&Component<T>> {
        self.world.sparse_store.get(self.link.sparse_index)
    }

    pub fn component_mut<T: 'static>(&mut self) -> Option<&mut Component<T>> {
        self.world.sparse_store.get_mut(self.link.sparse_index)
    }

    pub fn attach<T: ComponentDef>(&mut self, component: Component<T>) -> Option<Component<T>> {
        match T::STORE {
            ComponentStore::Sparse => {
                self.world.component_links.insert(
                    component.id().into_raw(),
                    ComponentLink::Sparse(self.link.sparse_index),
                );

                let removed = self
                    .world
                    .sparse_store
                    .insert(self.link.sparse_index, component)?;

                self.world.component_links.remove(&removed.id().into_raw());
                Some(removed)
            }
        }
    }
}
