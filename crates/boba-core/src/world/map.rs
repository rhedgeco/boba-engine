use std::any::Any;

use handle_map::{map::SparseHandleMap, Handle};

pub trait AnyMap: Any {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

impl dyn AnyMap {
    pub fn as_map<T: 'static>(&self) -> Option<&WorldMap<T>> {
        self.as_any().downcast_ref::<WorldMap<T>>()
    }

    pub fn as_map_mut<T: 'static>(&mut self) -> Option<&mut WorldMap<T>> {
        self.as_any_mut().downcast_mut::<WorldMap<T>>()
    }
}

struct DataEntry<T> {
    handle: Handle<usize>,
    data: T,
}

pub struct WorldMap<T> {
    indexer: SparseHandleMap<usize>,
    data: Vec<DataEntry<T>>,
}

impl<T: 'static> AnyMap for WorldMap<T> {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

impl<T> Default for WorldMap<T> {
    fn default() -> Self {
        Self {
            indexer: Default::default(),
            data: Default::default(),
        }
    }
}

impl<T> WorldMap<T> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn contains(&self, handle: Handle<T>) -> bool {
        self.indexer.get_data(handle.into_type()).is_some()
    }

    pub fn get(&self, handle: Handle<T>) -> Option<&T> {
        Some(&self.data[*self.indexer.get_data(handle.into_type())?].data)
    }

    pub fn get_mut(&mut self, handle: Handle<T>) -> Option<&mut T> {
        Some(&mut self.data[*self.indexer.get_data(handle.into_type())?].data)
    }

    pub fn index_of(&self, handle: Handle<T>) -> Option<usize> {
        self.indexer.get_data(handle.into_type()).cloned()
    }

    pub fn get_index(&self, index: usize) -> Option<(Handle<T>, &T)> {
        let entry = self.data.get(index)?;
        Some((entry.handle.into_type(), &entry.data))
    }

    pub fn get_index_mut(&mut self, index: usize) -> Option<(Handle<T>, &mut T)> {
        let entry = self.data.get_mut(index)?;
        Some((entry.handle.into_type(), &mut entry.data))
    }

    pub fn iter(&self) -> Iter<T> {
        Iter {
            entry_iter: self.data.iter(),
        }
    }

    pub fn iter_mut(&mut self) -> IterMut<T> {
        IterMut {
            entry_iter: self.data.iter_mut(),
        }
    }

    pub fn insert(&mut self, data: T) -> Handle<T> {
        let handle = self.indexer.insert(self.data.len());
        self.data.push(DataEntry { handle, data });
        handle.into_type()
    }

    pub fn remove(&mut self, handle: Handle<T>) -> Option<T> {
        // swap remove the entry to keep it packed
        let index = self.indexer.remove(handle.into_type())?;
        let data_entry = self.data.swap_remove(index);

        // correct the index of the swapped item
        if let Some(swapped) = self.data.get(index) {
            *self.indexer.get_data_mut(swapped.handle).unwrap() = index;
        }

        Some(data_entry.data)
    }
}

pub struct Iter<'a, T> {
    entry_iter: core::slice::Iter<'a, DataEntry<T>>,
}

impl<'a, T> Iter<'a, T> {
    pub fn empty() -> Self {
        Self {
            entry_iter: [].iter(),
        }
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = (Handle<T>, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        let entry = self.entry_iter.next()?;
        Some((entry.handle.into_type(), &entry.data))
    }
}

pub struct IterMut<'a, T> {
    entry_iter: core::slice::IterMut<'a, DataEntry<T>>,
}

impl<'a, T> IterMut<'a, T> {
    pub fn empty() -> Self {
        Self {
            entry_iter: [].iter_mut(),
        }
    }
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = (Handle<T>, &'a mut T);

    fn next(&mut self) -> Option<Self::Item> {
        let entry = self.entry_iter.next()?;
        Some((entry.handle.into_type(), &mut entry.data))
    }
}
