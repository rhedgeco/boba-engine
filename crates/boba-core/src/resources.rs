use std::any::{Any, TypeId};

use fxhash::FxHashMap;

pub trait Resource: 'static {}
impl<T: 'static> Resource for T {}

#[derive(Default)]
pub struct Resources {
    resources: FxHashMap<TypeId, Box<dyn Any>>,
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

    pub fn get<R: Resource>(&self) -> Option<&R> {
        let any = self.resources.get(&TypeId::of::<R>())?;
        any.downcast_ref()
    }

    pub fn get_mut<R: Resource>(&mut self) -> Option<&mut R> {
        let any = self.resources.get_mut(&TypeId::of::<R>())?;
        any.downcast_mut()
    }

    pub fn insert<R: Resource>(&mut self, resource: R) -> Option<R> {
        let id = TypeId::of::<R>();
        let any = self.resources.insert(id, Box::new(resource))?;
        *any.downcast().expect("Internal Error: Faulty downcast")
    }

    pub fn remove<R: Resource>(&mut self) -> Option<R> {
        let any = self.resources.remove(&TypeId::of::<R>())?;
        *any.downcast().expect("Internal Error: Faulty downcast")
    }
}
