use std::{fmt::Debug, hash::Hash};

use handle_map::{map::DenseHandleMap, Handle};
use indexmap::IndexSet;
use thiserror::Error;

pub struct Node<T>(Handle<NodeData<T>>);

impl<T: Debug> Debug for Node<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("NodeLink").field(&self.0).finish()
    }
}

impl<T> Copy for Node<T> {}
impl<T> Clone for Node<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<T> Hash for Node<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

impl<T> Eq for Node<T> {}
impl<T> PartialEq for Node<T> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

struct NodeData<T> {
    data: T,
    parent: Option<Node<T>>,
    children: IndexSet<Node<T>>,
}

impl<T: Debug> Debug for NodeData<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("NodeData")
            .field("data", &self.data)
            .field("parent", &self.parent)
            .field("children", &self.children)
            .finish()
    }
}

impl<T> NodeData<T> {
    fn new(data: T) -> Self {
        Self {
            data,
            parent: None,
            children: IndexSet::new(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Error)]
pub enum GetParentError {
    #[error("child node is not valid")]
    InvalidNode,
    #[error("node has no parent")]
    NoParent,
}

#[derive(Debug, PartialEq, Eq, Error)]
pub enum SetParentError {
    #[error("child node is not valid")]
    InvalidChild,
    #[error("parent node is not valid")]
    InvalidParent,
}

pub struct NodeTree<T> {
    nodes: DenseHandleMap<NodeData<T>>,
}

impl<T> Default for NodeTree<T> {
    fn default() -> Self {
        Self {
            nodes: Default::default(),
        }
    }
}

impl<T> NodeTree<T> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn contains(&self, node: Node<T>) -> bool {
        self.nodes.contains(node.0)
    }

    pub fn get(&self, node: Node<T>) -> Option<&T> {
        Some(&self.nodes.get_data(node.0)?.data)
    }

    pub fn get_mut(&mut self, node: Node<T>) -> Option<&mut T> {
        Some(&mut self.nodes.get_data_mut(node.0)?.data)
    }

    pub fn parent_of(&self, node: Node<T>) -> Result<Node<T>, GetParentError> {
        self.nodes
            .get_data(node.0)
            .ok_or(GetParentError::InvalidNode)?
            .parent
            .ok_or(GetParentError::NoParent)
    }

    pub fn children_of(&self, node: Node<T>) -> Option<impl Iterator<Item = &Node<T>>> {
        let node_children = &self.nodes.get_data(node.0)?.children;
        Some(node_children.iter())
    }

    pub fn insert(&mut self, data: T) -> Node<T> {
        Node(self.nodes.insert(NodeData::new(data)))
    }

    pub fn insert_with_parent(&mut self, data: T, parent: Node<T>) -> Node<T> {
        let node = self.insert(data);
        if let Some(parent_data) = self.nodes.get_data_mut(parent.0) {
            parent_data.children.insert(node);
            self.nodes.get_data_mut(node.0).unwrap().parent = Some(parent);
        }
        node
    }

    pub fn remove(&mut self, node: Node<T>) -> Option<T> {
        // remove the node from the tree
        let mut node_data = self.nodes.remove(node.0)?;

        // add all its children to its parent node if it exists
        if let Some(parent) = node_data.parent {
            let parent_data = self.nodes.get_data_mut(parent.0).unwrap();
            parent_data.children.remove(&node);
            for child in node_data.children.iter() {
                parent_data.children.insert(*child);
            }
        }

        // set the parent of all the children to the removed nodes parent
        for child in node_data.children.drain(..) {
            let child_data = self.nodes.get_data_mut(child.0).unwrap();
            child_data.parent = node_data.parent;
        }

        // return the data held by the node
        Some(node_data.data)
    }

    pub fn set_parent(
        &mut self,
        child: Node<T>,
        parent_option: Option<Node<T>>,
    ) -> Result<(), SetParentError> {
        // early return if the parent is Some and doesnt exist
        if parent_option.is_some_and(|parent| !self.nodes.contains(parent.0)) {
            return Err(SetParentError::InvalidParent);
        }

        // if child doesnt exist, return false
        let Some(child_data) = self.nodes.get_data_mut(child.0) else {
            return Err(SetParentError::InvalidChild);
        };

        // replace childs parent
        let old_parent_option = std::mem::replace(&mut child_data.parent, parent_option);

        // if the old parent existed, remove the child from its children list
        if let Some(old_parent) = old_parent_option {
            let old_parent_data = self.nodes.get_data_mut(old_parent.0).unwrap();
            old_parent_data.children.remove(&child);
        };

        // if the parent was set to None we are done
        let Some(parent) = parent_option else {
            return Ok(());
        };

        // add the child to the new parents children list
        let parent_data = self.nodes.get_data_mut(parent.0).unwrap();
        parent_data.children.insert(child);

        // resolve recusrive tree branches by walking up each parent
        resolve_recursive(self, parent, child, old_parent_option);
        return Ok(());

        fn resolve_recursive<T>(
            tree: &mut NodeTree<T>,
            parent: Node<T>,
            source: Node<T>,
            old_parent_option: Option<Node<T>>,
        ) {
            let parent_data = tree.nodes.get_data_mut(parent.0).unwrap();
            match parent_data.parent {
                // if there are no more parents in the chain, early return
                None => (),
                // if next parent is not the source node, recurse up and check the next parent in the chain
                Some(next_parent) if next_parent != source => {
                    resolve_recursive(tree, next_parent, source, old_parent_option)
                }
                // if a recursive loop was found, remove the recursive child from the source node
                // and set the recursive childs parent to the source nodes old parent.
                // also add the recursive child to the old parents children list
                Some(next_parent) => {
                    parent_data.parent = old_parent_option;
                    let next_parent_data = tree.nodes.get_data_mut(next_parent.0).unwrap();
                    next_parent_data.children.remove(&parent);
                    if let Some(old_parent) = old_parent_option {
                        let old_parent_data = tree.nodes.get_data_mut(old_parent.0).unwrap();
                        old_parent_data.children.insert(parent);
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_node() {
        let mut tree = NodeTree::<u32>::new();
        let node1 = tree.insert(42);
        assert!(tree.contains(node1));
        assert_eq!(tree.get(node1), Some(&42));
    }

    #[test]
    fn remove_node() {
        let mut tree = NodeTree::<u32>::new();
        let node1 = tree.insert(42);
        assert_eq!(tree.remove(node1), Some(42));
    }

    #[test]
    fn tree_inserts() {
        // tree structure
        //
        //       / leaf1
        // root -
        //       \ stem - leaf2

        let mut tree = NodeTree::<&'static str>::new();
        let root = tree.insert("root");
        let leaf1 = tree.insert_with_parent("leaf1", root);
        let stem = tree.insert("stem");
        tree.set_parent(stem, Some(root)).unwrap();
        let leaf2 = tree.insert_with_parent("leaf2", stem);

        // test parent links
        assert_eq!(tree.parent_of(root), Err(GetParentError::NoParent));
        assert_eq!(tree.parent_of(leaf1), Ok(root));
        assert_eq!(tree.parent_of(stem), Ok(root));
        assert_eq!(tree.parent_of(leaf2), Ok(stem));

        // test child links
        let mut root_children = tree.children_of(root).unwrap();
        assert_eq!(root_children.next(), Some(&leaf1));
        assert_eq!(root_children.next(), Some(&stem));
        assert_eq!(root_children.next(), None);
        let mut stem_children = tree.children_of(stem).unwrap();
        assert_eq!(stem_children.next(), Some(&leaf2));
        assert_eq!(stem_children.next(), None);
        assert_eq!(tree.children_of(leaf1).unwrap().next(), None);
        assert_eq!(tree.children_of(leaf2).unwrap().next(), None);
    }

    #[test]
    fn tree_removals() {
        // tree structure
        //
        //       / leaf1
        // root -
        //       \        / leaf2
        //        \ stem -
        //                \ leaf3

        let mut tree = NodeTree::<&'static str>::new();
        let root = tree.insert("root");
        let leaf1 = tree.insert_with_parent("leaf1", root);
        let stem = tree.insert_with_parent("stem", root);
        let leaf2 = tree.insert_with_parent("leaf2", stem);
        let leaf3 = tree.insert_with_parent("leaf3", stem);

        // remove the stem
        assert_eq!(tree.remove(stem), Some("stem"));
        assert!(!tree.contains(stem));

        // tree structure
        //
        //        / leaf1
        //       /
        // root --- leaf2
        //       \
        //        \ leaf3

        // check if parents were transferred
        assert_eq!(tree.parent_of(root), Err(GetParentError::NoParent));
        assert_eq!(tree.parent_of(leaf1), Ok(root));
        assert_eq!(tree.parent_of(leaf2), Ok(root));
        assert_eq!(tree.parent_of(leaf3), Ok(root));

        // check if children were transferred
        let mut root_children = tree.children_of(root).unwrap();
        assert_eq!(root_children.next(), Some(&leaf1));
        assert_eq!(root_children.next(), Some(&leaf2));
        assert_eq!(root_children.next(), Some(&leaf3));
        assert_eq!(root_children.next(), None);
        assert_eq!(tree.children_of(leaf1).unwrap().next(), None);
        assert_eq!(tree.children_of(leaf2).unwrap().next(), None);
        assert_eq!(tree.children_of(leaf3).unwrap().next(), None);
        drop(root_children);

        // remove the root
        assert_eq!(tree.remove(root), Some("root"));
        assert!(!tree.contains(root));

        // tree structure
        //
        // - leaf1
        // - leaf2
        // - leaf3

        // check if parents were transferred
        assert_eq!(tree.parent_of(leaf1), Err(GetParentError::NoParent));
        assert_eq!(tree.parent_of(leaf2), Err(GetParentError::NoParent));
        assert_eq!(tree.parent_of(leaf3), Err(GetParentError::NoParent));

        // double check children
        assert_eq!(tree.children_of(leaf1).unwrap().next(), None);
        assert_eq!(tree.children_of(leaf2).unwrap().next(), None);
        assert_eq!(tree.children_of(leaf3).unwrap().next(), None);
    }

    #[test]
    fn re_parent() {
        // tree structure
        //
        //       / leaf1
        // root -
        //       \        / leaf2
        //        \ stem -
        //                \ leaf3

        let mut tree = NodeTree::<&'static str>::new();
        let root = tree.insert("root");
        let leaf1 = tree.insert_with_parent("leaf1", root);
        let stem = tree.insert_with_parent("stem", root);
        let leaf2 = tree.insert_with_parent("leaf2", stem);
        let leaf3 = tree.insert_with_parent("leaf3", stem);

        // reparent stem to the leaf1 node
        assert!(tree.set_parent(stem, Some(leaf1)).is_ok());

        // tree structure
        //
        //                      / leaf2
        // root - leaf1 - stem -
        //                      \ leaf3

        // check if parents were transferred
        assert_eq!(tree.parent_of(root), Err(GetParentError::NoParent));
        assert_eq!(tree.parent_of(leaf1), Ok(root));
        assert_eq!(tree.parent_of(stem), Ok(leaf1));
        assert_eq!(tree.parent_of(leaf2), Ok(stem));
        assert_eq!(tree.parent_of(leaf3), Ok(stem));

        // check if children were transferred
        let mut root_children = tree.children_of(root).unwrap();
        assert_eq!(root_children.next(), Some(&leaf1));
        assert_eq!(root_children.next(), None);
        let mut leaf1_children = tree.children_of(leaf1).unwrap();
        assert_eq!(leaf1_children.next(), Some(&stem));
        assert_eq!(leaf1_children.next(), None);
        let mut stem_children = tree.children_of(stem).unwrap();
        assert_eq!(stem_children.next(), Some(&leaf2));
        assert_eq!(stem_children.next(), Some(&leaf3));
        assert_eq!(stem_children.next(), None);
        assert_eq!(tree.children_of(leaf2).unwrap().next(), None);
        assert_eq!(tree.children_of(leaf3).unwrap().next(), None);
    }

    #[test]
    fn recursive_reparent() {
        // tree structure
        //
        //       / leaf1
        // root -
        //       \        / leaf2
        //        \ stem -
        //                \ leaf3 -
        //                         \ leaf4

        let mut tree = NodeTree::<&'static str>::new();
        let root = tree.insert("root");
        let leaf1 = tree.insert_with_parent("leaf1", root);
        let stem = tree.insert_with_parent("stem", root);
        let leaf2 = tree.insert_with_parent("leaf2", stem);
        let leaf3 = tree.insert_with_parent("leaf3", stem);
        let leaf4 = tree.insert_with_parent("leaf4", leaf3);

        // set stem parent to leaf3 making a recursive loop
        assert!(tree.set_parent(stem, Some(leaf3)).is_ok());

        // tree structure
        //
        //       / leaf1
        // root -
        //       \         / leaf4
        //        \ leaf3 -
        //                 \ stem -
        //                         \ leaf2

        // check if parents were transferred
        assert_eq!(tree.parent_of(root), Err(GetParentError::NoParent));
        assert_eq!(tree.parent_of(leaf1), Ok(root));
        assert_eq!(tree.parent_of(leaf3), Ok(root));
        assert_eq!(tree.parent_of(leaf4), Ok(leaf3));
        assert_eq!(tree.parent_of(stem), Ok(leaf3));
        assert_eq!(tree.parent_of(leaf2), Ok(stem));

        // check if children were transferred
        let mut root_children = tree.children_of(root).unwrap();
        assert_eq!(root_children.next(), Some(&leaf1));
        assert_eq!(root_children.next(), Some(&leaf3));
        assert_eq!(root_children.next(), None);
        let mut leaf3_children = tree.children_of(leaf3).unwrap();
        assert_eq!(leaf3_children.next(), Some(&leaf4));
        assert_eq!(leaf3_children.next(), Some(&stem));
        assert_eq!(leaf3_children.next(), None);
        let mut stem_children = tree.children_of(stem).unwrap();
        assert_eq!(stem_children.next(), Some(&leaf2));
        assert_eq!(stem_children.next(), None);
        assert_eq!(tree.children_of(leaf1).unwrap().next(), None);
        assert_eq!(tree.children_of(leaf4).unwrap().next(), None);
        assert_eq!(tree.children_of(leaf2).unwrap().next(), None);
    }
}
