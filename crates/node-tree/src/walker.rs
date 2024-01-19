use crate::{
    tree::{Node, ParentError},
    NodeTree,
};

pub struct TreeWalker<'a, T> {
    tree: &'a mut NodeTree<T>,
    node: Node<T>,
}

impl<'a, T> TreeWalker<'a, T> {
    pub fn new(tree: &'a mut NodeTree<T>, start: Node<T>) -> Option<Self> {
        if !tree.contains(start) {
            return None;
        }

        Some(Self { tree, node: start })
    }

    pub fn node(&self) -> Node<T> {
        self.node
    }

    pub fn data(&self) -> &T {
        self.tree.get(self.node).unwrap()
    }

    pub fn data_mut(&mut self) -> &mut T {
        self.tree.get_mut(self.node).unwrap()
    }

    pub fn get_parent(&self) -> Option<Node<T>> {
        match self.tree.parent_of(self.node) {
            Ok(parent) => Some(parent),
            Err(ParentError::NoParent) => None,
            _ => unreachable!(), // nodes are garunteed to be valid
        }
    }

    pub fn get_children(&self) -> Box<[Node<T>]> {
        self.tree.children_of(self.node).unwrap().copied().collect()
    }

    pub fn jump_to(&mut self, node: Node<T>) -> bool {
        if !self.tree.contains(node) {
            return false;
        }

        self.node = node;
        true
    }

    pub fn jump_to_parent(&mut self) -> bool {
        let Some(parent) = self.get_parent() else {
            return false;
        };

        self.node = parent;
        true
    }
}
