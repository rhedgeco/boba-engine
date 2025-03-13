use std::{
    any::{Any, TypeId},
    collections::{HashMap, hash_map::Entry},
    mem::replace,
};

use fxhash::{FxBuildHasher, FxHashMap};

use crate::Component;

struct ComponentVec {
    push_none: fn(&mut Box<dyn Any>),
    any_vec: Box<dyn Any>,
}

impl ComponentVec {
    pub fn new<T: 'static>(size: usize) -> Self {
        let mut vec = Vec::<Option<Component<T>>>::with_capacity(size);
        vec.resize_with(size, || None);

        Self {
            any_vec: Box::new(vec),
            push_none: |any| {
                any.downcast_mut::<Vec<Option<Component<T>>>>()
                    .expect("valid vec")
                    .push(None);
            },
        }
    }

    pub fn push_none(&mut self) {
        (self.push_none)(&mut self.any_vec)
    }
}

pub struct SparseStore {
    type_index: FxHashMap<TypeId, usize>,
    components: Vec<ComponentVec>,
    entities: usize,
}

impl SparseStore {
    pub const fn new() -> Self {
        Self {
            type_index: HashMap::with_hasher(FxBuildHasher::new()),
            components: Vec::new(),
            entities: 0,
        }
    }

    pub fn len(&self) -> usize {
        self.entities
    }

    pub fn is_empty(&self) -> bool {
        self.entities == 0
    }

    pub fn spawn(&mut self) -> usize {
        for components in &mut self.components {
            components.push_none();
        }

        let index = self.entities;
        self.entities += 1;
        index
    }

    pub fn get<T: 'static>(&self, entity: usize) -> Option<&Component<T>> {
        debug_assert!(entity < self.entities);
        let type_index = *self.type_index.get(&TypeId::of::<T>())?;
        let any_vec = &self.components[type_index].any_vec;
        let vec = any_vec
            .downcast_ref::<Vec<Option<Component<T>>>>()
            .expect("valid vec");
        vec[entity].as_ref()
    }

    pub fn get_mut<T: 'static>(&mut self, entity: usize) -> Option<&mut Component<T>> {
        debug_assert!(entity < self.entities);
        let type_index = *self.type_index.get(&TypeId::of::<T>())?;
        let any_vec = &mut self.components[type_index].any_vec;
        let vec = any_vec
            .downcast_mut::<Vec<Option<Component<T>>>>()
            .expect("valid vec");
        vec[entity].as_mut()
    }

    pub fn remove<T: 'static>(&mut self, entity: usize) -> Option<Component<T>> {
        debug_assert!(entity < self.entities);
        let type_index = *self.type_index.get(&TypeId::of::<T>())?;
        let any_vec = &mut self.components[type_index].any_vec;
        let vec = any_vec
            .downcast_mut::<Vec<Option<Component<T>>>>()
            .expect("valid vec");
        vec[entity].take()
    }

    pub fn insert<T: 'static>(
        &mut self,
        entity: usize,
        component: Component<T>,
    ) -> Option<Component<T>> {
        debug_assert!(entity < self.entities);
        let type_index = self.get_or_init_type::<T>();
        let any_vec = &mut self.components[type_index].any_vec;
        let vec = any_vec
            .downcast_mut::<Vec<Option<Component<T>>>>()
            .expect("valid vec");
        replace(&mut vec[entity], Some(component))
    }

    pub fn init<T: 'static>(&mut self) {
        self.get_or_init_type::<T>();
    }

    fn get_or_init_type<T: 'static>(&mut self) -> usize {
        match self.type_index.entry(TypeId::of::<T>()) {
            Entry::Occupied(entry) => *entry.get(),
            Entry::Vacant(entry) => {
                let next_component_index = self.components.len();
                self.components.push(ComponentVec::new::<T>(self.entities));
                entry.insert(next_component_index);
                next_component_index
            }
        }
    }
}
