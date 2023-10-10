use crate::{event::EventRegister, BobaWorld};
use handle_map::map::sparse::SparseHandleMap;
use std::{
    any::{Any, TypeId},
    ops::{Deref, DerefMut},
};

pub type RawHandle = handle_map::RawHandle;
pub type Handle<P> = handle_map::Handle<P>;
pub type Iter<'a, P> = std::slice::Iter<'a, PearlEntry<P>>;
pub type IterMut<'a, P> = std::slice::IterMut<'a, PearlEntry<P>>;

/// A general data type that can be inserted into a [`World`](crate::World).
#[allow(unused_variables)]
pub trait Pearl: Sized + 'static {
    fn register(register: &mut impl EventRegister<Self>) {}
    fn on_insert(handle: Handle<Self>, world: &mut BobaWorld) {}
    fn on_insert_global(world: &mut BobaWorld) {}
    fn on_remove(&mut self, world: &mut BobaWorld) {}
    fn on_remove_global(&mut self, world: &mut BobaWorld) {}
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

#[derive(Debug)]
pub struct PearlEntry<P: Pearl> {
    handle: Handle<P>,
    pearl: P,
}

impl<P: Pearl> PearlEntry<P> {
    fn new(handle: Handle<P>, pearl: P) -> Self {
        Self { handle, pearl }
    }

    pub fn handle(&self) -> Handle<P> {
        self.handle
    }
}

impl<P: Pearl> DerefMut for PearlEntry<P> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.pearl
    }
}

impl<P: Pearl> Deref for PearlEntry<P> {
    type Target = P;

    fn deref(&self) -> &Self::Target {
        &self.pearl
    }
}

#[derive(Debug)]
pub struct PearlMap<P: Pearl> {
    pub(crate) links: SparseHandleMap<usize>,
    pub(crate) pearls: Vec<PearlEntry<P>>,
}

impl<P: Pearl> Default for PearlMap<P> {
    fn default() -> Self {
        Self {
            links: Default::default(),
            pearls: Default::default(),
        }
    }
}

impl<P: Pearl> PearlMap<P> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn len(&self) -> usize {
        self.pearls.len()
    }

    pub fn is_empty(&self) -> bool {
        self.pearls.is_empty()
    }

    pub fn contains(&self, handle: Handle<P>) -> bool {
        self.links.contains(handle.into_type())
    }

    pub fn get(&self, handle: Handle<P>) -> Option<&P> {
        let index = *self.links.get_data(handle.into_type())?;
        Some(&self.pearls[index].pearl)
    }

    pub fn get_mut(&mut self, handle: Handle<P>) -> Option<&mut P> {
        let index = *self.links.get_data(handle.into_type())?;
        Some(&mut self.pearls[index].pearl)
    }

    pub fn iter(&self) -> Iter<P> {
        self.pearls.iter()
    }

    pub fn iter_mut(&mut self) -> IterMut<P> {
        self.pearls.iter_mut()
    }

    pub fn insert(&mut self, pearl: P) -> Handle<P> {
        let handle = self.links.insert(self.pearls.len()).into_type();
        self.pearls.push(PearlEntry::new(handle, pearl));
        handle
    }

    pub fn remove(&mut self, handle: Handle<P>) -> Option<P> {
        let index = self.links.remove(handle.into_type())?;
        let removed = self.pearls.swap_remove(index).pearl;
        if let Some(swapped) = self.pearls.get_mut(index) {
            self.links
                .get_data_mut(swapped.handle.into_type())
                .map(|i| *i = index);
        }

        Some(removed)
    }
}

pub trait UntypedPearlMap: Any {
    fn len(&self) -> usize;
    fn is_empty(&self) -> bool;
    fn contains(&self, raw: RawHandle) -> bool;
    fn pearl_id(&self) -> PearlId;
    fn destroy(&mut self, raw: RawHandle);
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

impl<P: Pearl> UntypedPearlMap for PearlMap<P> {
    fn len(&self) -> usize {
        self.len()
    }

    fn is_empty(&self) -> bool {
        self.is_empty()
    }

    fn contains(&self, raw: RawHandle) -> bool {
        self.contains(Handle::from_raw(raw))
    }

    fn pearl_id(&self) -> PearlId {
        P::id()
    }

    fn destroy(&mut self, raw: RawHandle) {
        self.remove(Handle::from_raw(raw));
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}
