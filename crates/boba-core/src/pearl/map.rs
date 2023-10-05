use std::{
    any::Any,
    marker::PhantomData,
    ops::{Deref, DerefMut},
};

use fxhash::FxHashMap;
use handle_map::map::sparse::SparseHandleMap;

use crate::Pearl;

use super::{PearlExt, PearlId};

pub type Handle<P> = handle_map::Handle<P>;

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

pub type Iter<'a, P> = std::slice::Iter<'a, PearlEntry<P>>;
pub type IterMut<'a, P> = std::slice::IterMut<'a, PearlEntry<P>>;

#[derive(Debug)]
pub struct PearlMap<P: Pearl> {
    links: SparseHandleMap<usize>,
    pearls: Vec<PearlEntry<P>>,
    insert_queue: Vec<PearlEntry<P>>,
    destroy_queue: Vec<Handle<P>>,
}

impl<P: Pearl> Default for PearlMap<P> {
    fn default() -> Self {
        Self {
            links: Default::default(),
            pearls: Default::default(),
            insert_queue: Default::default(),
            destroy_queue: Default::default(),
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

    pub fn queue_insert(&mut self, pearl: P) -> Handle<P> {
        let insert_count = self.insert_queue.len();
        let handle = self.links.predict_handle(insert_count).into_type();
        self.insert_queue.push(PearlEntry::new(handle, pearl));
        handle
    }

    pub fn queue_destroy(&mut self, handle: Handle<P>) -> bool {
        if !self.links.contains(handle.into_type()) {
            return false;
        }

        self.destroy_queue.push(handle);
        true
    }

    pub fn flush_queue(&mut self) {
        for entry in self.insert_queue.drain(..) {
            self.pearls.push(entry);
        }

        let mut destroy_queue = core::mem::replace(&mut self.destroy_queue, Vec::new());
        for destroy in destroy_queue.drain(..) {
            self.remove(destroy);
        }
    }

    pub fn insert_now(&mut self, pearl: P) -> Handle<P> {
        self.flush_queue();
        self.insert(pearl)
    }

    pub fn remove_now(&mut self, handle: Handle<P>) -> Option<P> {
        self.flush_queue();
        self.remove(handle)
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

    fn insert(&mut self, pearl: P) -> Handle<P> {
        let handle = self.links.insert(self.pearls.len()).into_type();
        self.pearls.push(PearlEntry::new(handle, pearl));
        handle
    }

    fn remove(&mut self, handle: Handle<P>) -> Option<P> {
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

pub trait AnyPearlMap: Any {
    fn len(&self) -> usize;
    fn pearl_id(&self) -> PearlId;
    fn flush_queue(&mut self);
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

impl<P: Pearl> AnyPearlMap for PearlMap<P> {
    fn len(&self) -> usize {
        self.len()
    }

    fn pearl_id(&self) -> PearlId {
        P::id()
    }

    fn flush_queue(&mut self) {
        self.flush_queue()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

#[derive(Default)]
pub struct MultiPearlMap {
    map_index: FxHashMap<PearlId, usize>,
    pearl_maps: Vec<Box<dyn AnyPearlMap>>,
}

impl MultiPearlMap {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn len(&self) -> usize {
        self.pearl_maps.len()
    }

    pub fn get_map<P: Pearl>(&self) -> Option<&PearlMap<P>> {
        let index = *self.map_index.get(&P::id())?;
        self.pearl_maps[index].as_any().downcast_ref()
    }

    pub fn get_map_mut<P: Pearl>(&mut self) -> Option<&mut PearlMap<P>> {
        let index = *self.map_index.get(&P::id())?;
        self.pearl_maps[index].as_any_mut().downcast_mut()
    }

    pub fn queue_insert<P: Pearl>(&mut self, pearl: P) -> Handle<P> {
        self.get_or_create_map().queue_insert(pearl)
    }

    pub fn queue_destroy<P: Pearl>(&mut self, handle: Handle<P>) -> bool {
        match self.get_map_mut() {
            Some(map) => map.queue_destroy(handle),
            None => return false,
        }
    }

    pub fn flush_queue<P: Pearl>(&mut self) {
        self.get_map_mut::<P>().map(|map| map.flush_queue());
    }

    pub fn flush_all<P: Pearl>(&mut self) {
        for anymap in &mut self.pearl_maps {
            anymap.flush_queue()
        }
    }

    pub fn insert_now<P: Pearl>(&mut self, pearl: P) -> Handle<P> {
        self.get_or_create_map().insert_now(pearl)
    }

    pub fn remove_now<P: Pearl>(&mut self, handle: Handle<P>) -> Option<P> {
        self.get_map_mut()?.remove_now(handle)
    }

    pub fn get<P: Pearl>(&self, handle: Handle<P>) -> Option<&P> {
        self.get_map()?.get(handle)
    }

    pub fn get_mut<P: Pearl>(&mut self, handle: Handle<P>) -> Option<&mut P> {
        self.get_map_mut()?.get_mut(handle)
    }

    pub fn iter<P: Pearl>(&self) -> Option<Iter<P>> {
        let pearls = self.get_map()?;
        Some(pearls.iter())
    }

    pub fn iter_mut<P: Pearl>(&mut self) -> Option<IterMut<P>> {
        let pearls = self.get_map_mut()?;
        Some(pearls.iter_mut())
    }

    pub fn stream<P: Pearl>(&mut self) -> Option<PearlStream<P>> {
        let map_index = *self.map_index.get(&P::id())?;
        Some(PearlStream::new(map_index, self))
    }

    fn get_or_create_map<P: Pearl>(&mut self) -> &mut PearlMap<P> {
        use std::collections::hash_map::Entry as E;
        let index = match self.map_index.entry(P::id()) {
            E::Occupied(e) => *e.get(),
            E::Vacant(e) => {
                let index = self.pearl_maps.len();
                self.pearl_maps.push(Box::new(PearlMap::<P>::new()));
                e.insert(index);
                index
            }
        };

        let map = self.pearl_maps[index].as_any_mut().downcast_mut();
        map.expect("Internal Error: Faulty downcast")
    }
}

pub struct PearlStream<'a, P: Pearl> {
    map_index: usize,
    pearl_index: usize,
    source: &'a mut MultiPearlMap,
    _type: PhantomData<*const P>,
}

impl<'a, P: Pearl> PearlStream<'a, P> {
    fn new(map_index: usize, source: &'a mut MultiPearlMap) -> Self {
        Self {
            map_index,
            pearl_index: 0,
            source,
            _type: PhantomData,
        }
    }

    pub fn next_access(&mut self) -> Option<PearlAccess<P>> {
        let anymap = &self.source.pearl_maps[self.map_index];
        if self.pearl_index >= anymap.len() {
            return None;
        }

        let access = PearlAccess::new(self.map_index, self.pearl_index, self.source);
        self.pearl_index += 1;
        Some(access)
    }
}

pub struct PearlAccess<'a, P: Pearl> {
    map_index: usize,
    pearl_index: usize,
    source: &'a mut MultiPearlMap,
    _type: PhantomData<*const P>,
}

impl<'a, T: Pearl> PearlAccess<'a, T> {
    fn new(map_index: usize, pearl_index: usize, source: &'a mut MultiPearlMap) -> Self {
        Self {
            map_index,
            pearl_index,
            source,
            _type: PhantomData,
        }
    }

    pub fn current(&self) -> &PearlEntry<T> {
        let any = &self.source.pearl_maps[self.map_index];
        let map = any.as_any().downcast_ref::<PearlMap<T>>();
        let map = map.expect("Internal Error: Faulty downcast");
        &map.pearls[self.pearl_index]
    }

    pub fn current_mut(&mut self) -> &mut PearlEntry<T> {
        let any = &mut self.source.pearl_maps[self.map_index];
        let map = any.as_any_mut().downcast_mut::<PearlMap<T>>();
        let map = map.expect("Internal Error: Faulty downcast");
        &mut map.pearls[self.pearl_index]
    }

    pub fn get<P: Pearl>(&self, handle: Handle<P>) -> Option<&P> {
        self.source.get(handle)
    }

    pub fn get_mut<P: Pearl>(&mut self, handle: Handle<P>) -> Option<&mut P> {
        self.source.get_mut(handle)
    }

    pub fn queue_insert<P: Pearl>(&mut self, pearl: P) -> Handle<P> {
        self.source.queue_insert(pearl)
    }

    pub fn queue_destroy<P: Pearl>(&mut self, handle: Handle<P>) -> bool {
        self.source.queue_destroy(handle)
    }
}
