use std::{any::TypeId, hash::Hash, marker::PhantomData};

use handle_map::{map::SparseHandleMap, Handle};
use indexmap::{IndexMap, IndexSet};

use crate::arena::map::ArenaMap;

use super::map::{self, AnyMap};

type AnyMapBox = Box<dyn AnyMap>;

pub struct Link<T> {
    map_handle: Handle<AnyMapBox>,
    data_handle: Handle<T>,
}

impl<P> Copy for Link<P> {}
impl<P> Clone for Link<P> {
    fn clone(&self) -> Self {
        Self {
            map_handle: self.map_handle.clone(),
            data_handle: self.data_handle.clone(),
        }
    }
}

impl<P> Hash for Link<P> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.map_handle.hash(state);
        self.data_handle.hash(state);
    }
}

impl<P> Eq for Link<P> {}
impl<P> PartialEq for Link<P> {
    fn eq(&self, other: &Self) -> bool {
        self.map_handle == other.map_handle && self.data_handle == other.data_handle
    }
}

impl<P> Link<P> {
    #[doc(hidden)]
    pub fn from_raw(map: u64, data: u64) -> Self {
        Self {
            map_handle: Handle::from_raw(map),
            data_handle: Handle::from_raw(data),
        }
    }

    #[doc(hidden)]
    pub fn into_type<T>(self) -> Link<T> {
        Link {
            map_handle: self.map_handle,
            data_handle: self.data_handle.into_type(),
        }
    }
}

pub type Iter<'a, T> = map::Iter<'a, T>;
pub type IterMut<'a, T> = map::IterMut<'a, T>;

#[derive(Default)]
pub struct Arena {
    map_handles: IndexMap<TypeId, Handle<AnyMapBox>>,
    pearl_maps: SparseHandleMap<AnyMapBox>,
}

impl Arena {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn count<T: 'static>(&self) -> usize {
        match self.get_map::<T>() {
            Some(map) => map.len(),
            None => 0,
        }
    }

    pub fn has_type<T: 'static>(&self) -> bool {
        self.map_handles.get(&TypeId::of::<T>()).is_some()
    }

    pub fn contains<T: 'static>(&self, link: Link<T>) -> bool {
        let Some(map) = self.get_map_with(link.map_handle) else {
            return false;
        };

        map.contains(link.data_handle)
    }

    pub fn get<T: 'static>(&self, link: Link<T>) -> Option<&T> {
        self.get_map_with(link.map_handle)?.get(link.data_handle)
    }

    pub fn get_mut<T: 'static>(&mut self, link: Link<T>) -> Option<&mut T> {
        self.get_map_with_mut(link.map_handle)?
            .get_mut(link.data_handle)
    }

    pub fn iter<T: 'static>(&self) -> Iter<T> {
        match self.get_map::<T>() {
            Some(map) => map.iter(),
            None => Iter::empty(),
        }
    }

    pub fn iter_mut<T: 'static>(&mut self) -> IterMut<T> {
        match self.get_map_mut::<T>() {
            Some(map) => map.iter_mut(),
            None => IterMut::empty(),
        }
    }

    pub fn view_walker<T: 'static>(&mut self) -> Option<ViewWalker<T>> {
        ViewWalker::new(self)
    }

    pub fn insert<T: 'static>(&mut self, data: T, on_init: impl FnOnce()) -> Link<T> {
        use indexmap::map::Entry as E;
        match self.map_handles.entry(TypeId::of::<T>()) {
            E::Occupied(e) => {
                let map_handle = *e.get();
                let map = self.pearl_maps[map_handle].as_map_mut::<T>().unwrap();
                let data_handle = map.insert(data);
                Link {
                    map_handle,
                    data_handle,
                }
            }
            E::Vacant(e) => {
                let mut map = ArenaMap::new();
                let data_handle = map.insert(data);
                let map_handle = self.pearl_maps.insert(Box::new(map));
                e.insert(map_handle);
                on_init();
                Link {
                    map_handle,
                    data_handle,
                }
            }
        }
    }

    pub fn remove<T: 'static>(&mut self, link: Link<T>) -> Option<T> {
        let map = self.get_map_with_mut(link.map_handle)?;
        let data = map.remove(link.data_handle)?;

        // if map was emptied, remove its data from the arena
        if map.is_empty() {
            self.map_handles.remove(&TypeId::of::<T>()).unwrap();
            self.pearl_maps.remove(link.map_handle).unwrap();
        }

        Some(data)
    }

    fn get_map<T: 'static>(&self) -> Option<&ArenaMap<T>> {
        let map_handle = self.map_handles.get(&TypeId::of::<T>())?;
        let anymap = self.pearl_maps.get_data(*map_handle).unwrap();
        Some(anymap.as_map::<T>().unwrap())
    }

    fn get_map_mut<T: 'static>(&mut self) -> Option<&mut ArenaMap<T>> {
        let map_handle = self.map_handles.get(&TypeId::of::<T>())?;
        let anymap = self.pearl_maps.get_data_mut(*map_handle).unwrap();
        Some(anymap.as_map_mut::<T>().unwrap())
    }

    fn get_map_with<T: 'static>(&self, handle: Handle<AnyMapBox>) -> Option<&ArenaMap<T>> {
        self.pearl_maps.get_data(handle)?.as_map::<T>()
    }

    fn get_map_with_mut<T: 'static>(
        &mut self,
        handle: Handle<AnyMapBox>,
    ) -> Option<&mut ArenaMap<T>> {
        self.pearl_maps.get_data_mut(handle)?.as_map_mut::<T>()
    }
}

pub struct ViewWalker<'a, T> {
    arena: &'a mut Arena,
    destroy_queue: IndexSet<Link<()>>,
    map_handle: Handle<AnyMapBox>,
    data_index: usize,
    data_cap: usize,

    _type: PhantomData<*const T>,
}

impl<'a, T> Drop for ViewWalker<'a, T> {
    fn drop(&mut self) {
        // destroy all pending links
        for link in self.destroy_queue.iter() {
            if let Some(map) = self.arena.pearl_maps.get_data_mut(link.map_handle) {
                map.destroy(link.data_handle)
            }
        }
    }
}

impl<'a, T: 'static> ViewWalker<'a, T> {
    pub fn new(arena: &'a mut Arena) -> Option<Self> {
        let map_handle = *arena.map_handles.get(&TypeId::of::<T>())?;
        let data_cap = arena.get_map_with::<T>(map_handle).unwrap().len();
        Some(Self {
            arena,
            destroy_queue: IndexSet::new(),
            map_handle,
            data_index: 0,
            data_cap,
            _type: PhantomData,
        })
    }

    pub fn next(&mut self) -> Option<View<T>> {
        if self.data_index == self.data_cap {
            return None;
        }

        let data_index = self.data_index;
        self.data_index += 1;
        Some(View {
            arena: self.arena,
            destroy_queue: &mut self.destroy_queue,
            map_handle: self.map_handle,
            data_index,
            _type: PhantomData,
        })
    }
}

pub struct View<'a, T> {
    arena: &'a mut Arena,
    destroy_queue: &'a mut IndexSet<Link<()>>,
    map_handle: Handle<AnyMapBox>,
    data_index: usize,

    _type: PhantomData<*const T>,
}

impl<'a, T: 'static> View<'a, T> {
    pub fn count<T2: 'static>(&self) -> usize {
        self.arena.count::<T2>()
    }

    pub fn has_type<T2: 'static>(&self) -> bool {
        self.arena.has_type::<T2>()
    }

    pub fn contains<T2: 'static>(&self, link: Link<T2>) -> bool {
        self.arena.contains(link)
    }

    pub fn get<T2: 'static>(&self, link: Link<T2>) -> Option<&T2> {
        self.arena.get(link)
    }

    pub fn get_mut<T2: 'static>(&mut self, link: Link<T2>) -> Option<&mut T2> {
        // accessing mutably does not disturb any indices
        self.arena.get_mut(link)
    }

    pub fn current(&self) -> &T {
        let map = self.arena.get_map_with(self.map_handle).unwrap();
        map.get_index(self.data_index).unwrap().1
    }

    pub fn current_mut(&mut self) -> &mut T {
        let map = self.arena.get_map_with_mut(self.map_handle).unwrap();
        map.get_index_mut(self.data_index).unwrap().1
    }

    pub fn view_other<T2: 'static>(&mut self, link: Link<T2>) -> Option<View<T2>> {
        let map = self.arena.get_map_with(link.map_handle)?;
        let data_index = map.index_of(link.data_handle)?;
        Some(View {
            arena: self.arena,
            destroy_queue: self.destroy_queue,
            map_handle: link.map_handle,
            data_index,
            _type: PhantomData,
        })
    }

    pub fn iter<T2: 'static>(&self) -> Iter<T2> {
        self.arena.iter::<T2>()
    }

    pub fn iter_mut<T2: 'static>(&mut self) -> IterMut<T2> {
        // iterating mutably will not disturb any indices
        self.arena.iter_mut::<T2>()
    }

    pub fn insert<T2: 'static>(&mut self, data: T2, on_init: impl FnOnce()) -> Link<T2> {
        // inserting does not disturb any indices
        self.arena.insert(data, on_init)
    }

    pub fn destroy<T2: 'static>(&mut self, link: Link<T2>) -> bool {
        // check if the link is valid
        if !self.arena.contains(link) {
            return false;
        }

        // we have to queue removals because removing data disturbs indices
        self.destroy_queue.insert(link.into_type())
    }
}
