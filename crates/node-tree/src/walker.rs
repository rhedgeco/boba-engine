use crate::{
    tree::{GetParentError, Node},
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

    pub fn parent<'b>(&'b mut self) -> Option<TreeWalker<'b, T>> {
        let node = match self.tree.parent_of(self.node) {
            Ok(parent) => parent,
            Err(GetParentError::NoParent) => return None,
            _ => unreachable!(), // node is garunteed to be valid
        };
        Some(TreeWalker {
            tree: self.tree,
            node,
        })
    }

    pub fn children(&mut self) -> ChildWalker<T> {
        ChildWalker {
            children: self.tree.children_of(self.node).unwrap().cloned().collect(),
            tree: self.tree,
            current: 0,
        }
    }
}

pub struct ChildWalker<'a, T> {
    children: Box<[Node<T>]>,
    tree: &'a mut NodeTree<T>,
    current: usize,
}

impl<'a, T> ChildWalker<'a, T> {
    pub fn walk_next(&mut self) -> Option<TreeWalker<T>> {
        let child = *self.children.get(self.current)?;
        self.current += 1;
        match self.tree.contains(child) {
            false => self.walk_next(),
            true => Some(TreeWalker {
                tree: self.tree,
                node: child,
            }),
        }
    }
}
