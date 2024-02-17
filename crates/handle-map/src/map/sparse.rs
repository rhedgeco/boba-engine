use std::{
    collections::VecDeque,
    ops::{Index, IndexMut},
};

use crate::{map::HandleMapId, Handle};

#[derive(Debug)]
struct SparseEntry<T> {
    handle: Handle<T>,
    data: Option<T>,
}

impl<T> SparseEntry<T> {
    #[inline]
    pub fn new(handle: Handle<T>, data: T) -> Self {
        Self {
            handle,
            data: Some(data),
        }
    }
}

/// A storage solution that gives a [`Handle`] to the location of the data.
/// This is optimized for fast access, as the [`Handle`] ensures an array indexing operation.
///
/// This map is ***Sparse*** because when an item is removed,
/// the indices of the map are kept in place, and a hole is left where the item used to be.
/// That hole will then be filled again when new items are inserted into the map.
/// However, if many items are removed and there are no inserts afterwards,
/// the map will take up as much space as the max amount of items that used to be inside.
///
/// Since the map uses a [`Handle`] for indexing, the max length of the map is limited to `u32::MAX`.
#[derive(Debug)]
pub struct SparseHandleMap<T> {
    id: u16,
    values: Vec<SparseEntry<T>>,
    open_slots: VecDeque<usize>,
}

impl<T> Default for SparseHandleMap<T> {
    /// Returns a default handle map with a unique id.
    #[inline]
    fn default() -> Self {
        Self {
            id: HandleMapId::generate(),
            values: Default::default(),
            open_slots: Default::default(),
        }
    }
}

impl<T> IndexMut<Handle<T>> for SparseHandleMap<T> {
    fn index_mut(&mut self, handle: Handle<T>) -> &mut Self::Output {
        self.get_mut(handle).expect("invalid handle")
    }
}

impl<T> Index<Handle<T>> for SparseHandleMap<T> {
    type Output = T;

    fn index(&self, handle: Handle<T>) -> &Self::Output {
        self.get(handle).expect("invalid handle")
    }
}

impl<T> SparseHandleMap<T> {
    /// Returns a default handle map with a unique id.
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns the id for this manager.
    #[inline]
    pub fn id(&self) -> u16 {
        self.id
    }

    /// Returns the length of the map.
    #[inline]
    pub fn len(&self) -> usize {
        self.values.len() - self.open_slots.len()
    }

    /// Returns `true` if the map is empty.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.values.len() == self.open_slots.len()
    }

    /// Returns the handle that will be provided after `count` inserts.
    ///
    /// Is only true for chains of inserts.
    /// The prediction will be false if there is a removal before `count` is reached.
    #[inline]
    pub fn predict_handle(&self, count: usize) -> Handle<T> {
        match self.open_slots.get(count) {
            Some(index) => self.values[*index].handle,
            None => {
                let new_indices = count - self.open_slots.len();
                let index = self.values.len() + new_indices;
                Handle::from_raw_parts(index as u32, 0, self.id)
            }
        }
    }

    /// Inserts `data` into the map and returns a [`Handle`] to its location.
    #[inline]
    pub fn insert(&mut self, data: T) -> Handle<T> {
        match self.open_slots.pop_front() {
            Some(index) => {
                let entry = &mut self.values[index];
                entry.data = Some(data);
                entry.handle.clone()
            }
            None => {
                let index = self.values.len();
                if index > u32::MAX as usize {
                    // even though the index may be a usize, it gets stored as a u32.
                    // so if the length of the backing vec increases past u32::MAX,
                    // then a capacity overflow panic must be thrown.
                    panic!("SparseHandleMap capacity overflow");
                }

                let handle = Handle::from_raw_parts(index as u32, 0, self.id);
                self.values.push(SparseEntry::new(handle.clone(), data));
                handle
            }
        }
    }

    /// Returns true if `handle` is valid for this map.
    #[inline]
    pub fn contains(&self, handle: Handle<T>) -> bool {
        match self.values.get(handle.uindex()) {
            Some(other) => other.handle == handle,
            None => false,
        }
    }

    /// Returns a reference to the data associated with `handle`.
    ///
    /// Returns `None` if `handle` is invalid for this map.
    #[inline]
    pub fn get(&self, handle: Handle<T>) -> Option<&T> {
        match self.values.get(handle.uindex()) {
            Some(entry) if entry.handle == handle => entry.data.as_ref(),
            _ => None,
        }
    }

    /// Returns a mutable reference to the data associated with `handle`.
    ///
    /// Returns `None` if `handle` is invalid for this map.
    #[inline]
    pub fn get_mut(&mut self, handle: Handle<T>) -> Option<&mut T> {
        match self.values.get_mut(handle.uindex()) {
            Some(entry) if entry.handle == handle => entry.data.as_mut(),
            _ => None,
        }
    }

    /// Removes and returns the data for `handle`.
    ///
    /// Returns `None` if `handle` is invalid for this map.
    #[inline]
    pub fn remove(&mut self, handle: Handle<T>) -> Option<T> {
        match self.values.get_mut(handle.uindex()) {
            Some(entry) if entry.handle == handle => {
                let (index, gen, meta) = entry.handle.clone().into_raw_parts();
                entry.handle = Handle::from_raw_parts(index, gen.wrapping_add(1), meta);
                let data = std::mem::replace(&mut entry.data, None);
                self.open_slots.push_back(handle.uindex());
                data
            }
            _ => None,
        }
    }

    /// Returns an iterator over the map.
    #[inline]
    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            inner: self.values.iter(),
        }
    }

    /// Returns an iterator that allows modifying each value.
    #[inline]
    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        IterMut {
            inner: self.values.iter_mut(),
        }
    }
}

pub struct Iter<'a, T> {
    inner: std::slice::Iter<'a, SparseEntry<T>>,
}

impl<'a, T> Iter<'a, T> {
    pub fn empty() -> Self {
        Self { inner: [].iter() }
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = (Handle<T>, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let entry = self.inner.next()?;
            if let Some(data) = &entry.data {
                return Some((entry.handle, data));
            }
        }
    }
}

pub struct IterMut<'a, T> {
    inner: std::slice::IterMut<'a, SparseEntry<T>>,
}

impl<'a, T> IterMut<'a, T> {
    pub fn empty() -> Self {
        Self {
            inner: [].iter_mut(),
        }
    }
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = (Handle<T>, &'a mut T);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let entry = self.inner.next()?;
            if let Some(data) = &mut entry.data {
                return Some((entry.handle, data));
            }
        }
    }
}

pub struct IntoIter<T> {
    inner: std::vec::IntoIter<SparseEntry<T>>,
}

impl<T> IntoIterator for SparseHandleMap<T> {
    type Item = T;

    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter {
            inner: self.values.into_iter(),
        }
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(data) = self.inner.next()?.data {
                return Some(data);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn insert_remove() {
        let mut map = SparseHandleMap::<u32>::new();
        let handle = map.insert(42);
        assert!(map.len() == 1);
        assert!(map.contains(handle));
        assert!(map.get(handle).unwrap() == &42);

        let handle2 = map.insert(1234);
        assert!(map.len() == 2);
        assert!(map.contains(handle2));
        assert!(map.get(handle2).unwrap() == &1234);

        let data = map.remove(handle).unwrap();
        assert!(data == 42);
        let data2 = map.remove(handle2).unwrap();
        assert!(data2 == 1234);
    }

    #[test]
    fn predict() {
        let mut map = SparseHandleMap::<u32>::new();
        map.insert(42);

        let future_handle0 = map.predict_handle(0);
        let future_handle2 = map.predict_handle(2);
        map.insert(1234);
        map.insert(3456);
        map.insert(6789);

        assert!(map.contains(future_handle0));
        assert!(map.get(future_handle0).unwrap() == &1234);
        assert!(map.contains(future_handle2));
        assert!(map.get(future_handle2).unwrap() == &6789);
    }
}
