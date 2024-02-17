use std::any::TypeId;

use extension_trait::extension_trait;
use handle_map::{map::SparseHandleMap, Handle};

#[extension_trait]
pub impl<P: 'static> AnyPearlMap for PearlMap<P> {
    fn pearl_id(&self) -> TypeId {
        TypeId::of::<P>()
    }

    fn len(&self) -> usize {
        self.len()
    }

    fn is_empty(&self) -> bool {
        self.is_empty()
    }

    fn contains(&self, handle: Handle<()>) -> bool {
        self.contains(handle.into_type())
    }

    fn get_index(&self, handle: Handle<()>) -> Option<usize> {
        self.get_index(handle.into_type())
    }

    fn destroy(&mut self, handle: Handle<()>) -> bool {
        self.remove(handle.into_type()).is_some()
    }
}

impl dyn AnyPearlMap {
    pub fn has_type<P: 'static>(&self) -> bool {
        TypeId::of::<P>() == self.pearl_id()
    }

    pub fn as_map_mut<P: 'static>(&mut self) -> Option<&mut PearlMap<P>> {
        match self.has_type::<P>() {
            true => Some(unsafe { self.as_map_mut_unchecked() }),
            false => None,
        }
    }

    /// Casts this to a [`PearlMap`] reference with type `P`
    ///
    /// SAFETY: map type must be correct
    pub unsafe fn as_map_ref_unchecked<P: 'static>(&self) -> &PearlMap<P> {
        debug_assert!(self.has_type::<P>());
        &*(self as *const dyn AnyPearlMap as *const PearlMap<P>)
    }

    /// Casts this to a mutable [`PearlMap`] reference with type `P`
    ///
    /// SAFETY: map type must be correct
    pub unsafe fn as_map_mut_unchecked<P: 'static>(&mut self) -> &mut PearlMap<P> {
        debug_assert!(self.has_type::<P>());
        &mut *(self as *mut dyn AnyPearlMap as *mut PearlMap<P>)
    }
}

pub struct PearlEntry<P> {
    pub handle: Handle<usize>,
    pub pearl: P,
}

pub struct PearlMap<P> {
    indices: SparseHandleMap<usize>,
    pearls: Vec<PearlEntry<P>>,
}

impl<P> Default for PearlMap<P> {
    fn default() -> Self {
        Self {
            indices: Default::default(),
            pearls: Default::default(),
        }
    }
}

impl<P> PearlMap<P> {
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
        self.indices.contains(handle.into_type())
    }

    pub fn get(&self, handle: Handle<P>) -> Option<&P> {
        self.get_index(handle)
            .map(|index| &self.pearls[index].pearl)
    }

    pub fn get_index(&self, handle: Handle<P>) -> Option<usize> {
        self.indices.get(handle.into_type()).copied()
    }

    pub fn get_mut(&mut self, handle: Handle<P>) -> Option<&mut P> {
        self.get_index(handle)
            .map(|index| &mut self.pearls[index].pearl)
    }

    pub fn insert(&mut self, pearl: P) -> Handle<P> {
        let handle = self.indices.insert(self.pearls.len());
        self.pearls.push(PearlEntry { handle, pearl });
        handle.into_type()
    }

    pub fn last(&self) -> Option<&PearlEntry<P>> {
        self.pearls.last()
    }

    pub fn remove(&mut self, handle: Handle<P>) -> Option<P> {
        // swap remove the pearl from its place in the vec
        let index = self.indices.remove(handle.into_type())?;
        let pearl = self.pearls.swap_remove(index).pearl;

        // if another pearl was swapped there, correct its index
        if let Some(entry) = self.pearls.get_mut(index) {
            let swapped_index = self.indices.get_mut(entry.handle).unwrap();
            *swapped_index = index;
        };

        Some(pearl)
    }

    pub fn iter(&self) -> core::slice::Iter<PearlEntry<P>> {
        self.pearls.iter()
    }

    pub fn iter_mut(&mut self) -> core::slice::IterMut<PearlEntry<P>> {
        self.pearls.iter_mut()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn remove() {
        let mut map = PearlMap::<u32>::new();
        let h1 = map.insert(1);
        let h2 = map.insert(2);
        let h3 = map.insert(3);
        assert_eq!(map.get(h1), Some(&1));
        assert_eq!(map.get(h2), Some(&2));
        assert_eq!(map.get(h3), Some(&3));

        map.remove(h1);
        assert_eq!(map.get(h1), None);
        assert_eq!(map.get(h2), Some(&2));
        assert_eq!(map.get(h3), Some(&3));

        map.remove(h2);
        assert_eq!(map.get(h1), None);
        assert_eq!(map.get(h2), None);
        assert_eq!(map.get(h3), Some(&3));

        map.remove(h3);
        assert_eq!(map.get(h1), None);
        assert_eq!(map.get(h2), None);
        assert_eq!(map.get(h3), None);
    }
}
