pub mod collections;

use std::any::TypeId;

use crate::event::EventRegister;

/// A general data type that can be inserted into a [`World`](crate::World).
pub trait Pearl: Sized + 'static {
    fn register(register: &mut impl EventRegister<Self>);
}

/// Unique identifier that can only be created by a [`Pearl`] type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PearlId(TypeId);

impl PearlId {
    /// Returns the unique id associated with `P`.
    pub fn of<P: Pearl>() -> Self {
        Self(TypeId::of::<P>())
    }

    /// Returns the underlying [`TypeId`].
    pub fn into_raw(self) -> TypeId {
        self.0
    }
}

/// A set of useful extension methods for the [`Pearl`] type.
pub trait PearlExt: Pearl {
    fn id() -> PearlId;
    fn pearl_id(&self) -> PearlId;
}

impl<T: Pearl> PearlExt for T {
    /// Returns the associated [`PearlId`].
    fn id() -> PearlId {
        PearlId::of::<T>()
    }

    /// Returns the associated [`PearlId`].
    fn pearl_id(&self) -> PearlId {
        T::id()
    }
}
