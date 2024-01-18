use std::{
    any::{Any, TypeId},
    hash::Hash,
    ops::{Deref, DerefMut},
};

use handle_map::{map::DenseHandleMap, Handle};
use hashbrown::HashMap;
use node_tree::{NodeLink, NodeTree};

use crate::{Event, Pearl};

pub(super) type PearlMap<P> = DenseHandleMap<P>;
pub type EventListener<E> = for<'a> fn(&mut World, &mut <E as Event>::Data<'a>);

pub struct Link<P> {
    pub(super) map_handle: Handle<Box<dyn Any>>,
    pub(super) pearl_handle: Handle<P>,
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
    pub fn into_type<T>(self) -> Link<T> {
        Link {
            map_handle: self.map_handle,
            pearl_handle: self.pearl_handle.into_type(),
        }
    }
}

/// A struct used for calling the `on_remove` method of pearls.
/// This struct cannot be created manually so it prevents the function from being called.
pub struct Removed<'a, P>(&'a mut P);

impl<P> DerefMut for Removed<'_, P> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<P> Deref for Removed<'_, P> {
    type Target = P;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

type NodeData = HashMap<TypeId, Link<()>>;

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct Node(pub(super) NodeLink<NodeData>);

#[derive(Default)]
pub struct World {
    nodes: NodeTree<NodeData>,
    types: HashMap<TypeId, Handle<Box<dyn Any>>>,
    pearls: DenseHandleMap<Box<dyn Any>>,
    events: HashMap<TypeId, Box<dyn Any>>,
}

impl World {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn spawn(&mut self) -> Node {
        Node(self.nodes.insert(Default::default()))
    }

    pub fn spawn_with_parent(&mut self, parent: Node) -> Node {
        Node(self.nodes.insert_with_parent(Default::default(), parent.0))
    }

    pub fn contains_node(self, node: Node) -> bool {
        self.nodes.contains(node.0)
    }

    pub fn contains_pearl<P: Pearl>(&self, link: Link<P>) -> bool {
        self.get_pearl(link).is_some()
    }

    pub fn get_parent(&self, node: Node) -> Option<Node> {
        Some(Node(self.nodes.parent_of(node.0)?))
    }

    pub fn set_parent(&mut self, child: Node, parent: Option<Node>) -> bool {
        self.nodes.set_parent(child.0, parent.map(|n| n.0))
    }

    pub fn get_children(&self, node: Node) -> Option<Box<[Node]>> {
        let node_children = self.nodes.children_of(node.0)?;
        Some(node_children.map(|n| Node(*n)).collect())
    }

    pub fn get_pearl<P: Pearl>(&self, link: Link<P>) -> Option<&P> {
        let any = self.pearls.get_data(link.map_handle)?;
        let map = any.downcast_ref::<PearlMap<P>>().unwrap();
        map.get_data(link.pearl_handle)
    }

    pub fn find_pearl<P: Pearl>(&self, node: Node) -> Option<Link<P>> {
        let node_data = self.nodes.get(node.0)?;
        Some(node_data.get(&TypeId::of::<P>())?.into_type())
    }

    pub fn get_pearl_mut<P: Pearl>(&mut self, link: Link<P>) -> Option<&mut P> {
        let any = self.pearls.get_data_mut(link.map_handle)?;
        let map = any.downcast_mut::<PearlMap<P>>().unwrap();
        map.get_data_mut(link.pearl_handle)
    }

    pub fn remove_pearl<P: Pearl>(&mut self, link: Link<P>) -> Option<P> {
        let any = self.pearls.get_data_mut(link.map_handle)?;
        let map = any.downcast_mut::<PearlMap<P>>().unwrap();
        let mut pearl = map.remove(link.pearl_handle)?;
        P::on_remove(Removed(&mut pearl), self);
        Some(pearl)
    }

    pub fn iter<P: Pearl>(&self) -> impl Iterator<Item = &P> {
        let Some(map_handle) = self.types.get(&TypeId::of::<P>()) else {
            return [].iter();
        };

        let any = self.pearls.get_data(*map_handle).unwrap();
        let map = any.downcast_ref::<PearlMap<P>>().unwrap();
        map.iter()
    }

    pub fn iter_mut<P: Pearl>(&mut self) -> impl Iterator<Item = &mut P> {
        let Some(map_handle) = self.types.get(&TypeId::of::<P>()) else {
            return [].iter_mut();
        };

        let any = self.pearls.get_data_mut(*map_handle).unwrap();
        let map = any.downcast_mut::<PearlMap<P>>().unwrap();
        map.iter_mut()
    }

    pub fn add_listener<E: Event>(&mut self, listener: EventListener<E>) {
        use hashbrown::hash_map::Entry;
        match self.events.entry(TypeId::of::<E>()) {
            Entry::Occupied(e) => {
                let any = e.into_mut();
                let vec = any.downcast_mut::<Vec<EventListener<E>>>().unwrap();
                vec.push(listener);
            }
            Entry::Vacant(e) => {
                let mut vec = Vec::new();
                vec.push(listener);
                e.insert(Box::new(vec));
            }
        }
    }

    pub fn add_pearl<P: Pearl>(&mut self, node: Node, pearl: P) -> Result<Link<P>, P> {
        if self.nodes.get(node.0).is_none() {
            return Err(pearl);
        }

        let pearl_id = TypeId::of::<P>();
        use hashbrown::hash_map::Entry as E;
        let (map_handle, new_type) = match self.types.entry(pearl_id) {
            E::Occupied(e) => (*e.get(), false),
            E::Vacant(e) => {
                let map = Box::new(PearlMap::<P>::new());
                let handle = *e.insert(self.pearls.insert(map));
                (handle, true)
            }
        };

        let any = self.pearls.get_data_mut(map_handle).unwrap();
        let map = any.downcast_mut::<PearlMap<P>>().unwrap();
        let pearl_handle = map.insert(pearl);
        let link = Link {
            map_handle,
            pearl_handle,
        };

        let node_data = self.nodes.get_mut(node.0).unwrap();
        node_data.insert(pearl_id, link.into_type());
        P::on_insert(link, self);
        new_type.then(|| P::init_type(self));
        Ok(link)
    }

    pub fn trigger<'a, E: Event>(&mut self, event: &mut E::Data<'a>) {
        let event_id = TypeId::of::<E>();
        let Some(any) = self.events.remove(&event_id) else {
            return;
        };

        // remove the runners for this iteration so that they are not changed
        let mut runners = any.downcast::<Vec<EventListener<E>>>().unwrap();
        for runner in runners.iter() {
            runner(self, event);
        }

        // put the executed runners back after event is complete
        use hashbrown::hash_map::Entry;
        match self.events.entry(event_id) {
            Entry::Vacant(e) => {
                e.insert(runners);
            }
            Entry::Occupied(e) => {
                e.into_mut()
                    .downcast_mut::<Vec<EventListener<E>>>()
                    .unwrap()
                    .append(&mut runners);
            }
        }
    }
}
