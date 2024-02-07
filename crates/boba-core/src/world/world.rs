use std::{
    any::{Any, TypeId},
    hash::Hash,
    ops::{Index, IndexMut},
};

use extension_trait::extension_trait;
use handle_map::{map::SparseHandleMap, Handle};
use hashbrown::HashMap;
use indexmap::IndexMap;

use crate::{
    pearl::{Event, SimpleEvent},
    Pearl,
};

use self::sealed::{EventMap, EventRemover};

use super::{
    maps::{AnyPearlMap, PearlEntry, PearlMap},
    view::PearlView,
};

type MapBox = Box<dyn AnyPearlMap>;

pub struct Link<P> {
    map_handle: Handle<MapBox>,
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

impl<P> Link<P> {
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
    handle: Handle<MapBox>,
    events: IndexMap<TypeId, EventRemover>,
}

impl MapData {
    pub fn new<P: 'static>(handle: Handle<MapBox>) -> Self {
        Self {
            handle,
            events: IndexMap::new(),
        }
    }
}

pub struct InsertContext<'a, P: Pearl> {
    pub view: PearlView<'a, P>,
    _private: (),
}

pub struct RemoveContext<'a, P> {
    pub world: &'a mut World,
    pub old_link: Link<P>,
    pub pearl: &'a mut P,
    _private: (),
}

pub(super) type DestroyQueue = IndexMap<Link<()>, fn(Link<()>, &mut World)>;

#[derive(Default)]
pub struct World {
    events: HashMap<TypeId, Box<dyn Any>>,
    map_data: IndexMap<TypeId, MapData>,
    maps: SparseHandleMap<MapBox>,
    queue: DestroyQueue,
}

impl<P: 'static> Index<Link<P>> for World {
    type Output = P;

    fn index(&self, link: Link<P>) -> &Self::Output {
        self.get(link).expect("invalid link")
    }
}

impl<P: 'static> IndexMut<Link<P>> for World {
    fn index_mut(&mut self, link: Link<P>) -> &mut Self::Output {
        self.get_mut(link).expect("invalid link")
    }
}

impl World {
    pub fn new() -> Self {
        Self::default()
    }

    fn get_map<P: Pearl>(&self) -> Option<(Handle<MapBox>, &PearlMap<P>)> {
        let handle = self.map_data.get(&TypeId::of::<P>())?.handle;
        let map = self.maps.get_data(handle).unwrap();
        Some((handle, unsafe { map.as_map_ref_unchecked::<P>() }))
    }

    fn get_map_mut<P: Pearl>(&mut self) -> Option<(Handle<MapBox>, &mut PearlMap<P>)> {
        let handle = self.map_data.get(&TypeId::of::<P>())?.handle;
        let map = self.maps.get_data_mut(handle).unwrap();
        Some((handle, unsafe { map.as_map_mut_unchecked::<P>() }))
    }
}

#[extension_trait]
pub impl WorldAccess for World {
    fn is_empty(&self) -> bool {
        self.maps.is_empty()
    }

    fn type_count(&self) -> usize {
        self.maps.len()
    }

    fn len_of<P: 'static>(&self) -> usize {
        match self.map_data.get(&TypeId::of::<P>()) {
            Some(data) => self.maps.get_data(data.handle).unwrap().len(),
            None => 0,
        }
    }

    fn has<P: 'static>(&self) -> bool {
        self.map_data.contains_key(&TypeId::of::<P>())
    }

    fn contains<P: 'static>(&self, link: Link<P>) -> bool {
        match self.maps.get_data(link.map_handle) {
            Some(map) => {
                if map.pearl_id() != TypeId::of::<P>() {
                    return false;
                }
                map.contains(link.pearl_handle.into_type())
            }
            None => false,
        }
    }

    fn get<P: 'static>(&self, link: Link<P>) -> Option<&P> {
        let anymap = self.maps.get_data(link.map_handle)?;
        // SAFETY: map type is garunteed to be valid if the sparse handle is valid
        unsafe { anymap.as_map_ref_unchecked::<P>() }.get(link.pearl_handle)
    }

    fn get_mut<P: 'static>(&mut self, link: Link<P>) -> Option<&mut P> {
        let anymap = self.maps.get_data_mut(link.map_handle)?;
        // SAFETY: map type is garunteed to be valid if the sparse handle is valid
        unsafe { anymap.as_map_mut_unchecked::<P>() }.get_mut(link.pearl_handle)
    }

    fn get_view<P: Pearl>(&mut self, link: Link<P>) -> Option<PearlView<P>> {
        PearlView::new(link, self)
    }

    fn find_view<P: Pearl>(&mut self, predicate: impl Fn(&P) -> bool) -> Option<PearlView<P>> {
        let (link, _) = self.iter::<P>().find(|(_, p)| predicate(p))?;
        PearlView::new(link, self)
    }

    fn defer_destroy<P: Pearl>(&mut self, link: Link<P>) -> bool {
        match self.contains(link) {
            true => self
                .queue
                .insert(link.into_type(), |link, world| {
                    world.remove(link.into_type::<P>());
                })
                .is_none(),
            false => false,
        }
    }

    fn iter<P: Pearl>(&self) -> Iter<P> {
        match self.get_map::<P>() {
            None => Iter::empty(),
            Some((map_handle, map)) => Iter {
                map_handle,
                iter: map.iter(),
            },
        }
    }

    fn iter_mut<P: Pearl>(&mut self) -> IterMut<P> {
        match self.get_map_mut::<P>() {
            None => IterMut::empty(),
            Some((map_handle, map)) => IterMut {
                map_handle,
                iter: map.iter_mut(),
            },
        }
    }

    fn trigger_simple<E: SimpleEvent>(&mut self, event: &mut E) {
        self.trigger::<E>(event);
    }

    fn trigger<E: Event>(&mut self, data: &mut E::Data<'_>) {
        let Some(anymap) = self.events.get(&TypeId::of::<E>()) else {
            return;
        };

        let map = anymap.downcast_ref::<EventMap<E>>().unwrap();
        for runner in map.values().cloned().collect::<Vec<_>>() {
            runner(self, data);
        }
    }
}

#[extension_trait]
pub impl WorldInsert for World {
    fn insert<P: Pearl>(&mut self, pearl: P) -> Link<P> {
        self.insert_then(pearl, |_| {})
    }

    fn insert_then<P: Pearl>(&mut self, pearl: P, then: impl FnOnce(PearlView<P>)) -> Link<P> {
        use indexmap::map::Entry as E;
        let link = match self.map_data.entry(TypeId::of::<P>()) {
            E::Occupied(e) => {
                let map_handle = e.get().handle;
                let anymap = self.maps.get_data_mut(map_handle).unwrap();
                let map = anymap.as_map_mut().unwrap();
                let pearl_handle = map.insert(pearl);
                Link {
                    map_handle,
                    pearl_handle,
                }
            }
            E::Vacant(e) => {
                let mut map = PearlMap::new();
                let pearl_handle = map.insert(pearl);
                let map_handle = self.maps.insert(Box::new(map));
                e.insert(MapData::new::<P>(map_handle));
                P::register(self);
                Link {
                    map_handle,
                    pearl_handle,
                }
            }
        };

        P::on_insert(InsertContext {
            view: self.get_view(link).unwrap(),
            _private: (),
        });

        then(self.get_view(link).unwrap());

        link
    }
}

#[extension_trait]
pub impl WorldRemove for World {
    fn flush_destroy_queue(&mut self) {
        for (link, func) in core::mem::replace(&mut self.queue, DestroyQueue::new()) {
            func(link, self);
        }
    }

    fn pop<P: Pearl>(&mut self) -> Option<(Link<P>, P)> {
        let (map_handle, map) = self.get_map_mut::<P>()?;
        let entry = map.pop()?;
        Some((
            Link {
                map_handle,
                pearl_handle: entry.handle.into_type(),
            },
            entry.pearl,
        ))
    }

    fn remove<P: Pearl>(&mut self, link: Link<P>) -> Option<P> {
        let anymap = self.maps.get_data_mut(link.map_handle)?;
        // SAFETY: map type is garunteed to be valid if the sparse handle passed
        let map = unsafe { anymap.as_map_mut_unchecked::<P>() };
        let mut pearl = map.remove(link.pearl_handle)?;

        // id the map is empty remove it and its the event triggers
        if map.is_empty() {
            let pearl_id = TypeId::of::<P>();
            self.maps.remove(link.map_handle).unwrap();
            let map_data = self.map_data.swap_remove(&pearl_id).unwrap();
            for (_, remover) in map_data.events {
                remover(self);
            }
        }

        // run the on remove callback for this pearl
        P::on_remove(RemoveContext {
            world: self,
            old_link: link,
            pearl: &mut pearl,
            _private: (),
        });

        Some(pearl)
    }
}

mod sealed {
    use super::*;
    use crate::pearl::Event;

    pub type EventRemover = fn(&mut World);
    pub type EventFn<E> = fn(&mut World, &mut <E as Event>::Data<'_>);
    pub type EventMap<E> = IndexMap<TypeId, EventFn<E>>;

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
                let Some(map_data) = world.map_data.get(&TypeId::of::<P>()) else {
                    return;
                };

                let map_handle = map_data.handle;
                let anymap = world.maps.get_data(map_handle).unwrap();
                let map = unsafe { anymap.as_map_ref_unchecked::<P>() };

                for handle in map.iter().map(|e| e.handle).collect::<Vec<_>>().iter() {
                    let link = Link {
                        map_handle,
                        pearl_handle: handle.into_type(),
                    };

                    if let Some(view) = world.get_view(link) {
                        P::trigger(view, data);
                    }
                }
            });

            // add the event id and remover to the pearls map data
            let map_data = self.map_data.get_mut(&pearl_id).unwrap();
            map_data.events.insert(event_id, |world| {
                let event_map = world.events.get_mut(&TypeId::of::<E>()).unwrap();
                let event_map = event_map.downcast_mut::<EventMap<E>>().unwrap();
                event_map.swap_remove(&TypeId::of::<P>());
            });
        }
    }
}

pub struct Iter<'a, P: 'static> {
    map_handle: Handle<MapBox>,
    iter: core::slice::Iter<'a, PearlEntry<P>>,
}

impl<'a, P: 'static> Iter<'a, P> {
    pub fn empty() -> Self {
        Self {
            map_handle: Handle::from_raw(0),
            iter: [].iter(),
        }
    }
}

impl<'a, P: 'static> Iterator for Iter<'a, P> {
    type Item = (Link<P>, &'a P);

    fn next(&mut self) -> Option<Self::Item> {
        let entry = self.iter.next()?;
        Some((
            Link {
                map_handle: self.map_handle,
                pearl_handle: entry.handle.into_type(),
            },
            &entry.pearl,
        ))
    }
}

pub struct IterMut<'a, P: 'static> {
    map_handle: Handle<MapBox>,
    iter: core::slice::IterMut<'a, PearlEntry<P>>,
}

impl<'a, P: 'static> IterMut<'a, P> {
    pub fn empty() -> Self {
        Self {
            map_handle: Handle::from_raw(0),
            iter: [].iter_mut(),
        }
    }
}

impl<'a, P: 'static> Iterator for IterMut<'a, P> {
    type Item = (Link<P>, &'a mut P);

    fn next(&mut self) -> Option<Self::Item> {
        let entry = self.iter.next()?;
        Some((
            Link {
                map_handle: self.map_handle,
                pearl_handle: entry.handle.into_type(),
            },
            &mut entry.pearl,
        ))
    }
}
