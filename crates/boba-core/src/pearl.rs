use crate::{event::EventRegister, BobaWorld};
use handle_map::map::sparse::SparseHandleMap;
use std::{
    any::{Any, TypeId},
    cell::{Ref, RefCell, RefMut},
    ops::{Deref, DerefMut},
};

pub type RawHandle = handle_map::RawHandle;
pub type Handle<P> = handle_map::Handle<P>;
pub type Iter<'a, P> = std::slice::Iter<'a, PearlEntry<P>>;

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
    pearl: RefCell<P>,
}

impl<P: Pearl> PearlEntry<P> {
    fn new(handle: Handle<P>, pearl: P) -> Self {
        Self {
            handle,
            pearl: RefCell::new(pearl),
        }
    }

    pub fn borrow(&self) -> Option<PearlRef<P>> {
        let pearl = self.pearl.try_borrow().ok()?;
        Some(PearlRef {
            handle: self.handle,
            pearl,
        })
    }

    pub fn borrow_mut(&self) -> Option<PearlMut<P>> {
        let pearl = self.pearl.try_borrow_mut().ok()?;
        Some(PearlMut {
            handle: self.handle,
            pearl,
        })
    }

    pub fn handle(&self) -> Handle<P> {
        self.handle
    }
}

pub struct PearlRef<'a, P: Pearl> {
    handle: Handle<P>,
    pearl: Ref<'a, P>,
}

impl<'a, P: Pearl> PearlRef<'a, P> {
    pub fn handle(&self) -> Handle<P> {
        self.handle
    }
}

impl<'a, P: Pearl> Deref for PearlRef<'a, P> {
    type Target = P;

    fn deref(&self) -> &Self::Target {
        self.pearl.deref()
    }
}

pub struct PearlMut<'a, P: Pearl> {
    handle: Handle<P>,
    pearl: RefMut<'a, P>,
}

impl<'a, P: Pearl> PearlMut<'a, P> {
    pub fn handle(&self) -> Handle<P> {
        self.handle
    }
}

impl<'a, P: Pearl> DerefMut for PearlMut<'a, P> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.pearl.deref_mut()
    }
}

impl<'a, P: Pearl> Deref for PearlMut<'a, P> {
    type Target = P;

    fn deref(&self) -> &Self::Target {
        self.pearl.deref()
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

    pub fn get(&self, handle: Handle<P>) -> Option<PearlRef<P>> {
        let index = *self.links.get_data(handle.into_type())?;
        self.pearls[index].borrow()
    }

    pub fn get_where(&self, f: impl Fn(&PearlRef<P>) -> bool) -> Option<PearlRef<P>> {
        for pearl in self.pearls.iter().filter_map(|e| e.borrow()) {
            if f(&pearl) {
                return Some(pearl);
            }
        }

        None
    }

    pub fn get_mut(&self, handle: Handle<P>) -> Option<PearlMut<P>> {
        let index = *self.links.get_data(handle.into_type())?;
        self.pearls[index].borrow_mut()
    }

    pub fn get_mut_where(&self, f: impl Fn(&PearlMut<P>) -> bool) -> Option<PearlMut<P>> {
        for pearl in self.pearls.iter().filter_map(|e| e.borrow_mut()) {
            if f(&pearl) {
                return Some(pearl);
            }
        }

        None
    }

    pub fn iter(&self) -> Iter<P> {
        self.pearls.iter()
    }

    pub fn into_iter(self) -> IntoIter<P> {
        IntoIter {
            iter: self.pearls.into_iter(),
        }
    }

    pub fn insert(&mut self, pearl: P) -> Handle<P> {
        let handle = self.links.insert(self.pearls.len()).into_type();
        self.pearls.push(PearlEntry::new(handle, pearl));
        handle
    }

    pub fn remove(&mut self, handle: Handle<P>) -> Option<P> {
        let index = self.links.remove(handle.into_type())?;
        Some(self.remove_at_index(index))
    }

    pub fn remove_where(&mut self, f: impl Fn(&PearlRef<P>) -> bool) -> Option<P> {
        let mut remove_index = None;
        for (index, pearl) in self.pearls.iter().filter_map(|e| e.borrow()).enumerate() {
            if f(&pearl) {
                remove_index = Some(index);
            }
        }

        Some(self.remove_at_index(remove_index?))
    }

    fn remove_at_index(&mut self, index: usize) -> P {
        let removed = self.pearls.swap_remove(index).pearl;
        if let Some(swapped) = self.pearls.get_mut(index) {
            self.links
                .get_data_mut(swapped.handle.into_type())
                .map(|i| *i = index);
        }

        removed.into_inner()
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

pub struct IntoIter<P: Pearl> {
    iter: std::vec::IntoIter<PearlEntry<P>>,
}

impl<P: Pearl> IntoIter<P> {
    pub fn empty() -> Self {
        Self {
            iter: Vec::new().into_iter(),
        }
    }
}

impl<P: Pearl> Iterator for IntoIter<P> {
    type Item = P;

    fn next(&mut self) -> Option<Self::Item> {
        Some(self.iter.next()?.pearl.into_inner())
    }
}
