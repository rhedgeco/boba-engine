use std::any::{Any, TypeId};

use hashbrown::HashMap;

#[derive(Default)]
pub struct Resources {
    resources: HashMap<TypeId, Box<dyn Any>>,
}

impl Resources {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get<R: 'static>(&self) -> Option<&R> {
        let any = self.resources.get(&TypeId::of::<R>())?;
        Some(any.downcast_ref::<R>().unwrap())
    }

    pub fn get_mut<R: 'static>(&mut self) -> Option<&mut R> {
        let any = self.resources.get_mut(&TypeId::of::<R>())?;
        Some(any.downcast_mut::<R>().unwrap())
    }

    pub fn insert<R: 'static>(&mut self, resource: R) -> Option<R> {
        use hashbrown::hash_map::Entry as E;
        match self.resources.entry(TypeId::of::<R>()) {
            E::Vacant(e) => {
                e.insert(Box::new(resource));
                None
            }
            E::Occupied(e) => {
                let (_, any) = e.replace_entry(Box::new(resource));
                Some(*any.downcast::<R>().unwrap())
            }
        }
    }

    pub fn remove<R: 'static>(&mut self) -> Option<R> {
        let any = self.resources.remove(&TypeId::of::<R>())?;
        Some(*any.downcast::<R>().unwrap())
    }
}
