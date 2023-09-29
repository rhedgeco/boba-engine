pub mod array;
pub mod query;

pub use array::{ComponentIdArray, ComponentIdSlice};
use imposters::{collections::vec::ImposterVec, Imposter};
pub use query::{ComponentQuery, ComponentQueryArray, ComponentQuerySlice};

use const_fnv1a_hash::fnv1a_hash_str_64;

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ComponentId {
    value: u64,
}

impl ComponentId {
    pub const fn hash_str(str: &str) -> Self {
        Self {
            value: fnv1a_hash_str_64(str),
        }
    }

    pub const fn from_raw(value: u64) -> Self {
        Self { value }
    }

    pub const fn raw_value(&self) -> u64 {
        self.value
    }
}

pub unsafe trait Component: Send + Sync + 'static {
    const COMPONENT_ID: ComponentId;
}

pub trait ComponentExt: Component {
    fn component_id(&self) -> ComponentId;
}

impl<T: Component> ComponentExt for T {
    fn component_id(&self) -> ComponentId {
        Self::COMPONENT_ID
    }
}

pub struct AnyComponent {
    id: ComponentId,
    imposter: Imposter,
}

impl<T: Component> From<Box<T>> for AnyComponent {
    fn from(value: Box<T>) -> Self {
        Self {
            id: T::COMPONENT_ID,
            imposter: value.into(),
        }
    }
}

impl AnyComponent {
    pub fn new<T: Component>(component: T) -> Self {
        Box::new(component).into()
    }

    pub fn component_id(&self) -> ComponentId {
        self.id
    }

    pub fn downcast<T: Component>(self) -> Result<T, Self> {
        self.downcast_box().map(|b| *b)
    }

    pub fn downcast_box<T: Component>(mut self) -> Result<Box<T>, Self> {
        self.imposter = match self.imposter.downcast_box() {
            Ok(component) => return Ok(component),
            Err(imposter) => imposter,
        };

        return Err(self);
    }

    pub fn downcast_ref<T: Component>(&self) -> Option<&T> {
        self.imposter.downcast_ref()
    }

    pub fn downcast_mut<T: Component>(&mut self) -> Option<&mut T> {
        self.imposter.downcast_mut()
    }
}

pub struct AnyComponentVec {
    id: ComponentId,
    vec: ImposterVec,
}

impl AnyComponentVec {
    pub fn new<T: Component>() -> Self {
        Self {
            id: T::COMPONENT_ID,
            vec: ImposterVec::new::<T>(),
        }
    }

    pub fn from_any(any: AnyComponent) -> Self {
        Self {
            id: any.id,
            vec: ImposterVec::from_imposter(any.imposter),
        }
    }

    pub fn component_id(&self) -> ComponentId {
        self.id
    }

    pub fn get<T: Component>(&self, index: usize) -> Option<&T> {
        self.vec.get(index)
    }

    pub fn get_mut<T: Component>(&mut self, index: usize) -> Option<&mut T> {
        self.vec.get_mut(index)
    }

    pub fn as_slice<T: Component>(&self) -> Option<&[T]> {
        self.vec.as_slice()
    }

    pub fn as_slice_mut<T: Component>(&mut self) -> Option<&mut [T]> {
        self.vec.as_slice_mut()
    }
}
