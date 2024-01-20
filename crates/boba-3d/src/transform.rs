use std::{
    any::TypeId,
    ops::{Deref, DerefMut},
};

use boba_core::{world, Pearl};
use glam::{Mat4, Quat, Vec3};
use hashbrown::HashMap;
use indexmap::IndexSet;
use node_tree::{tree::Node, walker::ChildWalker, NodeTree, TreeWalker};

pub struct Transform {
    pub local_matrix: Mat4,
    pub world_matrix: Mat4,
    pub local_pos: Vec3,
    pub world_pos: Vec3,
    pub local_rot: Quat,
    pub world_rot: Quat,
    pub local_scale: Vec3,
    pub lossy_scale: Vec3,

    linked_pearls: HashMap<TypeId, IndexSet<world::Link<()>>>,
}

impl Default for Transform {
    fn default() -> Self {
        Self {
            local_matrix: Mat4::IDENTITY,
            world_matrix: Mat4::IDENTITY,
            local_pos: Vec3::ZERO,
            world_pos: Vec3::ZERO,
            local_rot: Quat::IDENTITY,
            world_rot: Quat::IDENTITY,
            local_scale: Vec3::ONE,
            lossy_scale: Vec3::ONE,
            linked_pearls: Default::default(),
        }
    }
}

impl Transform {
    pub fn linked_pearls<P: Pearl>(&self) -> Box<[world::Link<P>]> {
        match self.linked_pearls.get(&TypeId::of::<P>()) {
            Some(set) => set.iter().map(|link| link.into_type()).collect(),
            None => Box::new([]),
        }
    }
}

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct Link(Node<Transform>);

pub struct TransformTree {
    tree: NodeTree<Transform>,
    root: Node<Transform>,
}

impl Default for TransformTree {
    fn default() -> Self {
        let mut node_tree = NodeTree::new();
        let root = node_tree.insert(Transform::default());
        Self {
            tree: node_tree,
            root,
        }
    }
}

impl TransformTree {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn root(&self) -> &Transform {
        self.tree.get(self.root).unwrap()
    }

    pub fn root_view(&mut self) -> View {
        let walker = TreeWalker::new(&mut self.tree, self.root).unwrap();
        View(walker)
    }

    pub fn spawn(&mut self) -> Link {
        let root_matrix = self.tree.get(self.root).unwrap().world_matrix;
        let transform = Transform {
            world_matrix: root_matrix,
            ..Default::default()
        };
        Link(self.tree.insert_with_parent(transform, self.root))
    }

    pub fn spawn_with_parent(&mut self, parent: Link) -> Link {
        let transform = match self.tree.get(parent.0) {
            None => Transform::default(),
            Some(data) => Transform {
                world_matrix: data.world_matrix,
                ..Default::default()
            },
        };

        Link(self.tree.insert_with_parent(transform, parent.0))
    }

    pub fn get(&self, link: Link) -> Option<&Transform> {
        self.tree.get(link.0)
    }

    pub fn view(&mut self, link: Link) -> Option<View> {
        let walker = TreeWalker::new(&mut self.tree, link.0)?;
        Some(View(walker))
    }
}

pub struct View<'a>(TreeWalker<'a, Transform>);

impl<'a> DerefMut for View<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.0.data_mut()
    }
}

impl<'a> Deref for View<'a> {
    type Target = Transform;

    fn deref(&self) -> &Self::Target {
        self.0.data()
    }
}

impl<'a> View<'a> {
    pub fn parent(&mut self) -> Option<View> {
        self.0.parent().map(|walker| View(walker))
    }

    pub fn children(&mut self) -> ViewChildren {
        ViewChildren(self.0.children())
    }

    pub fn set_local_pos(&mut self, pos: Vec3) {
        if self.local_pos == pos {
            return;
        }

        self.0.data_mut().local_pos = pos;
        self.recalculate_local_matrix();
    }

    pub fn set_local_rot(&mut self, rot: Quat) {
        if self.local_rot == rot {
            return;
        }

        self.0.data_mut().local_rot = rot;
        self.recalculate_local_matrix();
    }

    pub fn set_local_scale(&mut self, scale: Vec3) {
        if self.local_scale == scale {
            return;
        }

        self.0.data_mut().local_scale = scale;
        self.recalculate_local_matrix();
    }

    pub fn set_local_pos_rot(&mut self, pos: Vec3, rot: Quat) {
        if self.local_pos == pos && self.local_rot == rot {
            return;
        }

        let data = self.0.data_mut();
        data.local_pos = pos;
        data.local_rot = rot;
        self.recalculate_local_matrix();
    }

    pub fn set_local_pos_scale(&mut self, pos: Vec3, scale: Vec3) {
        if self.local_pos == pos && self.local_scale == scale {
            return;
        }

        let data = self.0.data_mut();
        data.local_pos = pos;
        data.local_scale = scale;
        self.recalculate_local_matrix();
    }

    pub fn set_local_rot_scale(&mut self, rot: Quat, scale: Vec3) {
        if self.local_rot == rot && self.local_scale == scale {
            return;
        }

        let data = self.0.data_mut();
        data.local_rot = rot;
        data.local_scale = scale;
        self.recalculate_local_matrix();
    }

    pub fn set_local_pos_rot_scale(&mut self, pos: Vec3, rot: Quat, scale: Vec3) {
        if self.local_pos == pos && self.local_rot == rot && self.local_scale == scale {
            return;
        }

        let data = self.0.data_mut();
        data.local_pos = pos;
        data.local_rot = rot;
        data.local_scale = scale;
        self.recalculate_local_matrix();
    }

    pub fn link_pearl<P: Pearl>(&mut self, link: world::Link<P>) {
        use hashbrown::hash_map::Entry as E;
        match self.0.data_mut().linked_pearls.entry(TypeId::of::<P>()) {
            E::Vacant(e) => {
                e.insert(IndexSet::new()).insert(link.into_type());
            }
            E::Occupied(e) => {
                e.into_mut().insert(link.into_type());
            }
        }
    }

    pub fn unlink_pearl<P: Pearl>(&mut self, link: world::Link<P>) {
        let Some(set) = self.0.data_mut().linked_pearls.get_mut(&TypeId::of::<P>()) else {
            return;
        };

        set.remove(&link.into_type());
    }

    fn recalculate_local_matrix(&mut self) {
        let data = self.0.data_mut();
        self.0.data_mut().local_matrix =
            Mat4::from_scale_rotation_translation(data.local_scale, data.local_rot, data.local_pos);
        self.sync_transforms();
    }

    fn sync_transforms(&mut self) {
        self.world_matrix = match self.0.parent() {
            Some(parent) => parent.data().world_matrix * self.0.data().local_matrix,
            None => self.0.data().local_matrix,
        };

        (self.lossy_scale, self.world_rot, self.world_pos) =
            self.world_matrix.to_scale_rotation_translation();

        let mut children = self.0.children();
        while let Some(child) = children.walk_next() {
            View(child).sync_transforms()
        }
    }
}

pub struct ViewChildren<'a>(ChildWalker<'a, Transform>);

impl<'a> ViewChildren<'a> {
    pub fn walk_next(&mut self) -> Option<View> {
        self.0.walk_next().map(|walker| View(walker))
    }
}
