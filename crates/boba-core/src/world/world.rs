use std::{
    any::{Any, TypeId},
    fmt::{Debug, Display},
    hash::Hash,
    ops::{Deref, DerefMut},
};

use handle_map::{
    map::{DenseHandleMap, SparseHandleMap},
    Handle,
};
use hashbrown::HashMap;
use indexmap::IndexMap;

use crate::{pearl::Event, world::WorldQueue, Pearl};

use super::PearlView;

pub struct Link<P> {
    map_handle: Handle<Box<dyn Any>>,
    pearl_handle: Handle<P>,
}

impl<P> Copy for Link<P> {}
impl<P> Clone for Link<P> {
    fn clone(&self) -> Self {
        Self {
            map_handle: self.map_handle.clone(),
            pearl_handle: self.pearl_handle.clone(),
        }
    }
}

impl<P> Hash for Link<P> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.map_handle.hash(state);
        self.pearl_handle.hash(state);
    }
}

impl<P> Eq for Link<P> {}
impl<P> PartialEq for Link<P> {
    fn eq(&self, other: &Self) -> bool {
        self.map_handle == other.map_handle && self.pearl_handle == other.pearl_handle
    }
}

impl<P> Debug for Link<P> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Link")
            .field("map_handle", &self.map_handle)
            .field("pearl_handle", &self.pearl_handle)
            .finish()
    }
}

impl<P> Display for Link<P> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let id = self.pearl_handle.id();
        let pearl_name = core::any::type_name::<P>();
        write!(f, "Link<{pearl_name}>({id})")
    }
}

impl<P> Link<P> {
    pub fn id(&self) -> u64 {
        self.pearl_handle.id()
    }

    #[doc(hidden)]
    pub fn into_type<P2>(&self) -> Link<P2> {
        Link {
            map_handle: self.map_handle,
            pearl_handle: self.pearl_handle.into_type(),
        }
    }

    #[doc(hidden)]
    pub fn from_raw(v1: u64, v2: u64) -> Self {
        Link {
            map_handle: Handle::from_raw(v1),
            pearl_handle: Handle::from_raw(v2),
        }
    }
}

struct MapData {
    handle: Handle<Box<dyn Any>>,
    events: IndexMap<TypeId, fn(&mut World)>,
}

impl MapData {
    pub fn new(handle: Handle<Box<dyn Any>>) -> Self {
        Self {
            handle,
            events: IndexMap::new(),
        }
    }
}

type EventFn<E> = fn(&mut WorldQueue, &mut E);
type EventMap<E> = IndexMap<TypeId, EventFn<E>>;

/// A storage solution for multiple all types of [`Pearl`] structs.
#[derive(Default)]
pub struct World {
    map_data: HashMap<TypeId, MapData>,
    maps: SparseHandleMap<Box<dyn Any>>,
    events: HashMap<TypeId, Box<dyn Any>>,
}

impl World {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn types(&self) -> usize {
        self.maps.len()
    }

    pub fn is_empty(&self) -> bool {
        self.maps.is_empty()
    }

    pub fn len<P: Pearl>(&self) -> usize {
        match self.map_data.get(&TypeId::of::<P>()) {
            None => 0,
            Some(map_data) => {
                let Some(anymap) = self.maps.get(map_data.handle) else {
                    return 0;
                };
                let map = anymap.downcast_ref::<DenseHandleMap<P>>().unwrap();
                map.len()
            }
        }
    }

    pub fn has<P: Pearl>(&self) -> bool {
        self.map_data.contains_key(&TypeId::of::<P>())
    }

    pub fn contains<P: Pearl>(&self, link: Link<P>) -> bool {
        let Some(anymap) = self.maps.get(link.map_handle) else {
            return false;
        };

        let map = anymap.downcast_ref::<DenseHandleMap<P>>().unwrap();
        map.contains(link.pearl_handle)
    }

    pub fn get<P: Pearl>(&self, link: Link<P>) -> Option<&P> {
        let anymap = self.maps.get(link.map_handle)?;
        let map = anymap.downcast_ref::<DenseHandleMap<P>>().unwrap();
        map.get(link.pearl_handle)
    }

    pub fn get_mut<P: Pearl>(&mut self, link: Link<P>) -> Option<&mut P> {
        let anymap = self.maps.get_mut(link.map_handle)?;
        let map = anymap.downcast_mut::<DenseHandleMap<P>>().unwrap();
        map.get_mut(link.pearl_handle)
    }

    pub fn remove<P: Pearl>(&mut self, link: Link<P>) -> Option<P> {
        let anymap = self.maps.get_mut(link.map_handle)?;
        let map = anymap.downcast_mut::<DenseHandleMap<P>>().unwrap();
        let mut pearl = map.remove(link.pearl_handle)?;

        // remove map and its data if map was emptied
        if map.is_empty() {
            self.map_data.remove(&TypeId::of::<P>()).unwrap();
            self.maps.remove(link.map_handle).unwrap();
        }

        P::on_remove(Removed {
            world: self,
            old_link: link,
            pearl: &mut pearl,
        });

        Some(pearl)
    }

    pub fn insert<P: Pearl>(&mut self, pearl: P) -> Link<P> {
        self.insert_then(pearl, |_| {})
    }

    pub fn insert_then<P: Pearl>(&mut self, pearl: P, then: impl FnOnce(PearlView<P>)) -> Link<P> {
        use hashbrown::hash_map::Entry as E;
        let link = match self.map_data.entry(TypeId::of::<P>()) {
            E::Occupied(e) => {
                let map_data = e.into_mut();
                let anymap = self.maps.get_mut(map_data.handle).unwrap();
                let map = anymap.downcast_mut::<DenseHandleMap<P>>().unwrap();
                let pearl_handle = map.insert(pearl);
                Link {
                    map_handle: map_data.handle,
                    pearl_handle,
                }
            }
            E::Vacant(e) => {
                let mut map = DenseHandleMap::new();
                let pearl_handle = map.insert(pearl);
                let map_handle = self.maps.insert(Box::new(map));
                e.insert(MapData::new(map_handle)); // register events
                P::register(self);
                Link {
                    map_handle,
                    pearl_handle,
                }
            }
        };

        let mut queue = WorldQueue::new(self);
        P::on_insert(Inserted {
            view: PearlView::new_unchecked(link, &mut queue),
        });
        then(PearlView::new_unchecked(link, &mut queue));
        link
    }

    pub fn links<P: Pearl>(&self) -> Links<P> {
        match self.map_data.get(&TypeId::of::<P>()) {
            None => Links::empty(),
            Some(map_data) => {
                let anymap = self.maps.get(map_data.handle).unwrap();
                let map = anymap.downcast_ref::<DenseHandleMap<P>>().unwrap();
                Links {
                    inner: map.handles(),
                    map_handle: map_data.handle,
                }
            }
        }
    }

    pub fn links_copied<P: Pearl>(&self) -> LinksCopied<P> {
        match self.map_data.get(&TypeId::of::<P>()) {
            None => LinksCopied::empty(),
            Some(map_data) => {
                let anymap = self.maps.get(map_data.handle).unwrap();
                let map = anymap.downcast_ref::<DenseHandleMap<P>>().unwrap();
                LinksCopied {
                    inner: map.handles_copied().into_iter(),
                    map_handle: map_data.handle,
                }
            }
        }
    }

    pub fn pearls<P: Pearl>(&self) -> Pearls<P> {
        match self.map_data.get(&TypeId::of::<P>()) {
            None => Pearls::empty(),
            Some(map_data) => {
                let anymap = self.maps.get(map_data.handle).unwrap();
                let map = anymap.downcast_ref::<DenseHandleMap<P>>().unwrap();
                Pearls {
                    inner: map.values(),
                }
            }
        }
    }

    pub fn pearls_mut<P: Pearl>(&mut self) -> PearlsMut<P> {
        match self.map_data.get(&TypeId::of::<P>()) {
            None => PearlsMut::empty(),
            Some(map_data) => {
                let anymap = self.maps.get_mut(map_data.handle).unwrap();
                let map = anymap.downcast_mut::<DenseHandleMap<P>>().unwrap();
                PearlsMut {
                    inner: map.values_mut(),
                }
            }
        }
    }

    pub fn iter<P: Pearl>(&self) -> Iter<P> {
        match self.map_data.get(&TypeId::of::<P>()) {
            None => Iter::empty(),
            Some(map_data) => {
                let anymap = self.maps.get(map_data.handle).unwrap();
                let map = anymap.downcast_ref::<DenseHandleMap<P>>().unwrap();
                Iter {
                    inner: map.iter(),
                    map_handle: map_data.handle,
                }
            }
        }
    }

    pub fn iter_mut<P: Pearl>(&mut self) -> IterMut<P> {
        match self.map_data.get(&TypeId::of::<P>()) {
            None => IterMut::empty(),
            Some(map_data) => {
                let anymap = self.maps.get_mut(map_data.handle).unwrap();
                let map = anymap.downcast_mut::<DenseHandleMap<P>>().unwrap();
                IterMut {
                    inner: map.iter_mut(),
                    map_handle: map_data.handle,
                }
            }
        }
    }

    pub fn trigger<E: Event>(&mut self, data: &mut E) {
        let mut queue = WorldQueue::new(self);
        Self::trigger_nested::<E>(&mut queue, data);
    }

    pub(crate) fn trigger_nested<E: Event>(queue: &mut WorldQueue, data: &mut E) {
        let Some(anymap) = queue.world.events.get(&TypeId::of::<E>()) else {
            return;
        };

        let map = anymap.downcast_ref::<EventMap<E>>().unwrap();
        let runners = map.values().cloned().collect::<Vec<_>>();
        for runner in runners {
            runner(queue, data);
        }
    }
}

// seal event source impl so it cannot be called externally
mod sealed {
    use super::*;

    impl<P> crate::pearl::EventSource<P> for World {
        fn listen<E: Event>(&mut self)
        where
            P: crate::pearl::Listener<E>,
        {
            // create pearl and event ids
            let pearl_id = TypeId::of::<P>();
            let event_id = TypeId::of::<E>();

            // get the event map associated with E
            use hashbrown::hash_map::Entry;
            let map = match self.events.entry(event_id) {
                Entry::Occupied(e) => e.into_mut().downcast_mut::<EventMap<E>>().unwrap(),
                Entry::Vacant(e) => e
                    .insert(Box::new(EventMap::<E>::new()))
                    .downcast_mut::<EventMap<E>>()
                    .unwrap(),
            };

            // insert the event trigger code for P
            map.insert(pearl_id, |world, data| {
                for link in world.links_copied::<P>() {
                    let view = PearlView::new_unchecked(link, world);
                    P::trigger(view, data);
                }
            });

            // add the event id and remover to the pearls map data
            let map_data = self.map_data.get_mut(&pearl_id).unwrap();
            map_data.events.insert(event_id, |world| {
                let anymap = world.events.get_mut(&TypeId::of::<E>()).unwrap();
                let map = anymap.downcast_mut::<EventMap<E>>().unwrap();
                map.swap_remove(&TypeId::of::<P>());
            });
        }
    }
}

pub struct Links<'a, P> {
    inner: handle_map::map::dense::Handles<'a, P>,
    map_handle: Handle<Box<dyn Any>>,
}

impl<'a, P> Links<'a, P> {
    pub fn empty() -> Self {
        Self {
            inner: handle_map::map::dense::Handles::empty(),
            map_handle: Handle::from_raw(0),
        }
    }
}

impl<'a, P> Iterator for Links<'a, P> {
    type Item = Link<P>;

    fn next(&mut self) -> Option<Self::Item> {
        let pearl_handle = self.inner.next()?;
        Some(Link {
            map_handle: self.map_handle,
            pearl_handle,
        })
    }
}

pub struct LinksCopied<P> {
    inner: std::vec::IntoIter<Handle<P>>,
    map_handle: Handle<Box<dyn Any>>,
}

impl<P> LinksCopied<P> {
    pub fn empty() -> Self {
        Self {
            inner: Vec::new().into_iter(),
            map_handle: Handle::from_raw(0),
        }
    }
}

impl<P> Iterator for LinksCopied<P> {
    type Item = Link<P>;

    fn next(&mut self) -> Option<Self::Item> {
        let pearl_handle = self.inner.next()?;
        Some(Link {
            map_handle: self.map_handle,
            pearl_handle,
        })
    }
}

pub struct Pearls<'a, P> {
    inner: handle_map::map::dense::Values<'a, P>,
}

impl<'a, P> Pearls<'a, P> {
    pub fn empty() -> Self {
        Self {
            inner: handle_map::map::dense::Values::empty(),
        }
    }
}

impl<'a, P> Iterator for Pearls<'a, P> {
    type Item = &'a P;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}

pub struct PearlsMut<'a, P> {
    inner: handle_map::map::dense::ValuesMut<'a, P>,
}

impl<'a, P> PearlsMut<'a, P> {
    pub fn empty() -> Self {
        Self {
            inner: handle_map::map::dense::ValuesMut::empty(),
        }
    }
}

impl<'a, P> Iterator for PearlsMut<'a, P> {
    type Item = &'a mut P;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}

pub struct Iter<'a, P> {
    inner: handle_map::map::dense::Iter<'a, P>,
    map_handle: Handle<Box<dyn Any>>,
}

impl<'a, P> Iter<'a, P> {
    pub fn empty() -> Self {
        Self {
            inner: handle_map::map::dense::Iter::empty(),
            map_handle: Handle::from_raw(0),
        }
    }
}

impl<'a, P> Iterator for Iter<'a, P> {
    type Item = (Link<P>, &'a P);

    fn next(&mut self) -> Option<Self::Item> {
        let (pearl_handle, pearl) = self.inner.next()?;
        Some((
            Link {
                map_handle: self.map_handle,
                pearl_handle,
            },
            pearl,
        ))
    }
}

pub struct IterMut<'a, P> {
    inner: handle_map::map::dense::IterMut<'a, P>,
    map_handle: Handle<Box<dyn Any>>,
}

impl<'a, P> IterMut<'a, P> {
    pub fn empty() -> Self {
        Self {
            inner: handle_map::map::dense::IterMut::empty(),
            map_handle: Handle::from_raw(0),
        }
    }
}

impl<'a, P> Iterator for IterMut<'a, P> {
    type Item = (Link<P>, &'a mut P);

    fn next(&mut self) -> Option<Self::Item> {
        let (pearl_handle, pearl) = self.inner.next()?;
        Some((
            Link {
                map_handle: self.map_handle,
                pearl_handle,
            },
            pearl,
        ))
    }
}

pub struct Inserted<'a, 'world, P: Pearl> {
    view: PearlView<'a, 'world, P>,
}

impl<'a, 'world, P: Pearl> DerefMut for Inserted<'a, 'world, P> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.view
    }
}

impl<'a, 'world, P: Pearl> Deref for Inserted<'a, 'world, P> {
    type Target = PearlView<'a, 'world, P>;

    fn deref(&self) -> &Self::Target {
        &self.view
    }
}

pub struct Removed<'a, P: Pearl> {
    world: &'a mut World,
    old_link: Link<P>,
    pearl: &'a mut P,
}

impl<'a, P: Pearl> DerefMut for Removed<'a, P> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.pearl
    }
}

impl<'a, P: Pearl> Deref for Removed<'a, P> {
    type Target = P;

    fn deref(&self) -> &Self::Target {
        self.pearl
    }
}

impl<'a, P: Pearl> Removed<'a, P> {
    pub fn old_link(&self) -> Link<P> {
        self.old_link
    }

    pub fn world(&self) -> &World {
        &self.world
    }

    pub fn world_mut(&mut self) -> &mut World {
        &mut self.world
    }
}
