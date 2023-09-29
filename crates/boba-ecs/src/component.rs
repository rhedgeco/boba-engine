use const_identify::{ConstId, ConstIdentify};
use imposters::{collections::vec::ImposterVec, Imposter};

/// Marker trait for any struct that is a valid `boba-ecs` component
pub trait Component: ConstIdentify + Send + Sync + 'static {
    fn const_id(&self) -> ConstId;
}

impl<T: ConstIdentify + Send + Sync + 'static> Component for T {
    fn const_id(&self) -> ConstId {
        Self::CONST_ID
    }
}

pub struct AnyComponent {
    id: ConstId,
    imposter: Imposter,
}

impl<T: Component> From<Box<T>> for AnyComponent {
    fn from(value: Box<T>) -> Self {
        Self {
            id: T::CONST_ID,
            imposter: value.into(),
        }
    }
}

impl AnyComponent {
    pub fn new<T: Component>(component: T) -> Self {
        Box::new(component).into()
    }

    pub fn const_id(&self) -> ConstId {
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
    id: ConstId,
    vec: ImposterVec,
}

impl AnyComponentVec {
    pub fn new<T: Component>() -> Self {
        Self {
            id: T::CONST_ID,
            vec: ImposterVec::new::<T>(),
        }
    }

    pub fn from_any(any: AnyComponent) -> Self {
        Self {
            id: any.id,
            vec: ImposterVec::from_imposter(any.imposter),
        }
    }

    pub fn const_id(&self) -> ConstId {
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
