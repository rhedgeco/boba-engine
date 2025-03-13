use std::{
    marker::PhantomData,
    ops::{Deref, DerefMut},
};

use derivative::Derivative;
use derive_more::Display;

use crate::UniqueId;

#[repr(transparent)]
#[derive(Display, Derivative)]
#[derivative(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[display("{raw}")]
pub struct ComponentId<T> {
    _type: PhantomData<fn(T)>,
    raw: UniqueId,
}

impl<T> ComponentId<T> {
    pub fn unique() -> Self {
        Self {
            _type: PhantomData,
            raw: UniqueId::new(),
        }
    }

    pub fn from_raw(raw: UniqueId) -> Self {
        Self {
            _type: PhantomData,
            raw,
        }
    }

    pub fn into_raw(self) -> UniqueId {
        self.raw
    }
}

pub struct Component<T> {
    id: ComponentId<T>,
    data: T,
}

impl<T> Component<T> {
    pub fn id(&self) -> ComponentId<T> {
        self.id
    }

    pub fn into_inner(self) -> T {
        self.data
    }
}

impl<T> Deref for Component<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<T> DerefMut for Component<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}

pub enum ComponentStore {
    Sparse,
}

pub trait ComponentDef: 'static {
    const STORE: ComponentStore;
}

pub trait IntoComponent: ComponentDef {
    fn into_component(self) -> Component<Self>
    where
        Self: Sized,
    {
        Component {
            id: ComponentId::unique(),
            data: self,
        }
    }
}
impl<T: ComponentDef> IntoComponent for T {}
