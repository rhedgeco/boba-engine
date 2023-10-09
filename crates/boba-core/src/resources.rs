use std::any::{Any, TypeId};

use indexmap::IndexMap;

#[derive(Default)]
pub struct Resources {
    resources: IndexMap<TypeId, Box<dyn Any>>,
}

impl Resources {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn len(&self) -> usize {
        self.resources.len()
    }

    pub fn is_empty(&self) -> bool {
        self.resources.is_empty()
    }

    pub fn get<T: 'static>(&self) -> Option<&T> {
        self.resources.get(&TypeId::of::<T>())?.downcast_ref()
    }

    pub fn get_mut<T: 'static>(&mut self) -> Option<&mut T> {
        self.resources.get_mut(&TypeId::of::<T>())?.downcast_mut()
    }

    pub fn insert<T: 'static>(&mut self, resource: T) -> Option<T> {
        let id = TypeId::of::<T>();
        let any = self.resources.insert(id, Box::new(resource))?;
        *any.downcast().expect("Internal Error: Faulty downcast")
    }

    pub fn remove_resource<T: 'static>(&mut self) -> Option<T> {
        let any = self.resources.remove(&TypeId::of::<T>())?;
        *any.downcast().expect("Internal Error: Faulty downcast")
    }
}
