use std::ops::{Index, IndexMut};

use crate::Handle;

use super::{sparse::SparseHandleMap, HandleMapId};

/// A storage solution that gives a [`Handle`] to the location of the data.
/// This is optimized for fast access, as the [`Handle`] ensures an array indexing operation.
///
/// This map is ***Dense*** because all the data is stored right next to eachother in memory.
/// This means that when accessing with a handle, there needs to be one more level of indirection under the covers.
/// However, iteration over the map will always be maximally efficient,
/// as the whole map can be used as a tightly packed array slice.
#[derive(Debug)]
pub struct DenseHandleMap<T> {
    id: u16,
    link_map: SparseHandleMap<usize>,
    back_link: Vec<Handle<T>>,
    values: Vec<T>,
}

impl<T> Default for DenseHandleMap<T> {
    /// Returns a default handle map with a unique id.
    #[inline]
    fn default() -> Self {
        Self {
            id: HandleMapId::generate(),
            link_map: Default::default(),
            back_link: Default::default(),
            values: Default::default(),
        }
    }
}

impl<T> IndexMut<Handle<T>> for DenseHandleMap<T> {
    fn index_mut(&mut self, handle: Handle<T>) -> &mut Self::Output {
        self.get_mut(handle).expect("invalid handle")
    }
}

impl<T> Index<Handle<T>> for DenseHandleMap<T> {
    type Output = T;

    fn index(&self, handle: Handle<T>) -> &Self::Output {
        self.get(handle).expect("invalid handle")
    }
}

impl<T> DenseHandleMap<T> {
    /// Returns a new handle map with a unique id.
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns the undelying id for this map.
    #[inline]
    pub fn id(&self) -> u16 {
        self.id
    }

    /// Returns the number of items in the map.
    #[inline]
    pub fn len(&self) -> usize {
        self.values.len()
    }

    /// Returns `true` if the map is empty.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }

    /// Returns the handle that will be provided after `count` inserts.
    ///
    /// Is only true for chains of inserts.
    /// The prediction will be false if there is a removal before `count` is reached.
    #[inline]
    pub fn predict_handle(&self, count: usize) -> Handle<T> {
        self.link_map.predict_handle(count).into_type()
    }

    /// Inserts `data` into the map, and returns a [`Handle`] to its location.
    #[inline]
    pub fn insert(&mut self, value: T) -> Handle<T> {
        let handle = self.link_map.insert(self.values.len());
        self.back_link.push(handle.into_type());
        self.values.push(value);
        handle.into_type::<T>()
    }

    /// Returns true if `handle` is valid for this map.
    #[inline]
    pub fn contains(&self, handle: Handle<T>) -> bool {
        self.link_map.contains(handle.into_type())
    }

    /// Returns a reference to the data associated with `handle`.
    ///
    /// Returns `None` if the handle is invalid.
    #[inline]
    pub fn get(&self, handle: Handle<T>) -> Option<&T> {
        let index = self.link_map.get(handle.into_type())?;
        Some(&self.values[*index])
    }

    /// Returns a mutable reference to the data associated with `handle`.
    ///
    /// Returns `None` if the handle is invalid.
    #[inline]
    pub fn get_mut(&mut self, handle: Handle<T>) -> Option<&mut T> {
        let index = self.link_map.get(handle.into_type())?;
        Some(&mut self.values[*index])
    }

    /// Removes and returns the data associated with `handle` from this map.
    ///
    /// Returns `None` if the handle is invalid.
    #[inline]
    pub fn remove(&mut self, handle: Handle<T>) -> Option<T> {
        // get the index for the handle
        let index = self.link_map.remove(handle.into_type())?;

        // The data will be swap removed from its vec,
        // so the back link should also be swap_removed.
        // If other data would be swapped into a new location,
        // we need to reflect that back in the link map
        // so that handles will still be valid.
        self.back_link.swap_remove(index);
        if let Some(handle) = self.back_link.get(index) {
            *self.link_map.get_mut(handle.into_type()).unwrap() = index;
        }

        // finally, swap remove the data and return it
        Some(self.values.swap_remove(index))
    }

    /// Returns an iterator over the handles of the map.
    #[inline]
    pub fn handles(&self) -> Handles<'_, T> {
        Handles {
            inner: self.back_link.iter(),
        }
    }

    /// Returns a copy of all the handles in this map
    #[inline]
    pub fn handles_copied(&self) -> Vec<Handle<T>> {
        self.back_link.clone()
    }

    /// Returns an iterator over the reference values of a map.
    #[inline]
    pub fn values(&self) -> Values<'_, T> {
        Values {
            inner: self.values.iter(),
        }
    }

    /// Returns an iterator over the mutable values of a map.
    #[inline]
    pub fn values_mut(&mut self) -> ValuesMut<'_, T> {
        ValuesMut {
            inner: self.values.iter_mut(),
        }
    }

    /// Returns an iterator over the handles and reference values of the map.
    #[inline]
    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            handles: Handles {
                inner: self.back_link.iter(),
            },
            values: Values {
                inner: self.values.iter(),
            },
        }
    }

    /// Returns an iterator over the handles and mutable values of the map.
    #[inline]
    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        IterMut {
            handles: Handles {
                inner: self.back_link.iter(),
            },
            values: ValuesMut {
                inner: self.values.iter_mut(),
            },
        }
    }
}

pub struct Handles<'a, T> {
    inner: core::slice::Iter<'a, Handle<T>>,
}

impl<'a, T> Handles<'a, T> {
    pub fn empty() -> Self {
        Self { inner: [].iter() }
    }
}

impl<'a, T> Iterator for Handles<'a, T> {
    type Item = Handle<T>;

    fn next(&mut self) -> Option<Self::Item> {
        Some(self.inner.next()?.into_type())
    }
}

pub struct Values<'a, T> {
    inner: core::slice::Iter<'a, T>,
}

impl<'a, T> Values<'a, T> {
    pub fn empty() -> Self {
        Self { inner: [].iter() }
    }
}

impl<'a, T> Iterator for Values<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}

pub struct ValuesMut<'a, T> {
    inner: core::slice::IterMut<'a, T>,
}

impl<'a, T> ValuesMut<'a, T> {
    pub fn empty() -> Self {
        Self {
            inner: [].iter_mut(),
        }
    }
}

impl<'a, T> Iterator for ValuesMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}

pub struct Iter<'a, T> {
    handles: Handles<'a, T>,
    values: Values<'a, T>,
}

impl<'a, T> Iter<'a, T> {
    pub fn empty() -> Self {
        Self {
            handles: Handles::empty(),
            values: Values::empty(),
        }
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = (Handle<T>, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        Some((
            self.handles.next()?.into_type(),
            self.values.next().unwrap(),
        ))
    }
}

pub struct IterMut<'a, T> {
    handles: Handles<'a, T>,
    values: ValuesMut<'a, T>,
}

impl<'a, T> IterMut<'a, T> {
    pub fn empty() -> Self {
        Self {
            handles: Handles::empty(),
            values: ValuesMut::empty(),
        }
    }
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = (Handle<T>, &'a mut T);

    fn next(&mut self) -> Option<Self::Item> {
        Some((
            self.handles.next()?.into_type(),
            self.values.next().unwrap(),
        ))
    }
}

pub struct IntoIter<T> {
    handles: std::vec::IntoIter<Handle<T>>,
    values: std::vec::IntoIter<T>,
}

impl<T> IntoIterator for DenseHandleMap<T> {
    type Item = (Handle<T>, T);

    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter {
            handles: self.back_link.into_iter(),
            values: self.values.into_iter(),
        }
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = (Handle<T>, T);

    fn next(&mut self) -> Option<Self::Item> {
        Some((
            self.handles.next()?.into_type(),
            self.values.next().unwrap(),
        ))
    }
}
