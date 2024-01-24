use boba_core::{
    world::{Link, Removed, View},
    Pearl, World,
};
use extension_trait::extension_trait;
use glam::{Mat4, Quat, Vec3};
use indexmap::IndexSet;

pub type Iter<'a> = indexmap::set::Iter<'a, Link<Transform>>;

pub struct Transform {
    local_matrix: Mat4,
    world_matrix: Mat4,
    local_pos: Vec3,
    world_pos: Vec3,
    local_rot: Quat,
    world_rot: Quat,
    local_scale: Vec3,
    lossy_scale: Vec3,
    pending_sync: bool,

    link: Link<Self>,
    parent: Option<Link<Self>>,
    children: IndexSet<Link<Self>>,
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
            pending_sync: false,
            link: Link::from_raw(0, 0),
            parent: None,
            children: IndexSet::new(),
        }
    }
}

impl Pearl for Transform {
    fn on_insert(view: &mut View<'_, Self>, link: Link<Self>) {
        view.link = link;
        view.pending_sync = false;
    }

    fn on_remove(mut pearl: Removed<Self>, world: &mut World) {
        // remove the parent
        let parent_option = pearl.parent.take();

        // remove the pearl from its parents children list
        if let Some(parent) = parent_option {
            world.get_mut(parent).unwrap().children.remove(&pearl.link);
        }

        // remove the pearl from its childrens parent slot
        for child in pearl.children.drain(..) {
            world.get_mut(child).unwrap().parent = parent_option;
        }
    }

    fn on_view_drop(view: &mut View<Self>) {
        if view.pending_sync {
            view.sync_transforms();
        }
    }
}

impl Transform {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from_pos(pos: Vec3) -> Self {
        Self::from_pos_rot_scale(pos, Quat::IDENTITY, Vec3::ONE)
    }

    pub fn from_rot(rot: Quat) -> Self {
        Self::from_pos_rot_scale(Vec3::ZERO, rot, Vec3::ONE)
    }

    pub fn from_scale(scale: Vec3) -> Self {
        Self::from_pos_rot_scale(Vec3::ZERO, Quat::IDENTITY, scale)
    }

    pub fn from_pos_rot(pos: Vec3, rot: Quat) -> Self {
        Self::from_pos_rot_scale(pos, rot, Vec3::ONE)
    }

    pub fn from_pos_scale(pos: Vec3, scale: Vec3) -> Self {
        Self::from_pos_rot_scale(pos, Quat::IDENTITY, scale)
    }

    pub fn from_rot_scale(rot: Quat, scale: Vec3) -> Self {
        Self::from_pos_rot_scale(Vec3::ZERO, rot, scale)
    }

    pub fn from_pos_rot_scale(pos: Vec3, rot: Quat, scale: Vec3) -> Self {
        let matrix = Mat4::from_scale_rotation_translation(scale, rot, pos);
        Self {
            local_matrix: matrix,
            world_matrix: matrix,
            local_pos: pos,
            world_pos: pos,
            local_rot: rot,
            world_rot: rot,
            local_scale: scale,
            lossy_scale: scale,
            ..Default::default()
        }
    }

    pub fn local_matrix(&self) -> Mat4 {
        self.local_matrix
    }

    pub fn world_matrix(&self) -> Mat4 {
        self.world_matrix
    }

    pub fn local_pos(&self) -> Vec3 {
        self.local_pos
    }

    pub fn world_pos(&self) -> Vec3 {
        self.world_pos
    }

    pub fn set_local_pos_no_sync(&mut self, pos: Vec3) {
        self.local_pos = pos;
        self.pending_sync = true;
    }

    pub fn local_rot(&self) -> Quat {
        self.local_rot
    }

    pub fn world_rot(&self) -> Quat {
        self.world_rot
    }

    pub fn set_local_rot_no_sync(&mut self, rot: Quat) {
        self.local_rot = rot;
        self.pending_sync = true;
    }

    pub fn local_scale(&self) -> Vec3 {
        self.local_scale
    }

    pub fn lossy_scale(&self) -> Vec3 {
        self.lossy_scale
    }

    pub fn set_local_scale_no_sync(&mut self, scale: Vec3) {
        self.local_scale = scale;
        self.pending_sync = true;
    }

    pub fn parent_link(&self) -> Option<Link<Self>> {
        self.parent
    }

    pub fn child_links(&self) -> Iter {
        self.children.iter()
    }
}

#[extension_trait]
impl<'a> PrivateTransformView<'a> for View<'a, Transform> {
    fn set_parent_no_sync(&mut self, parent_option: Option<Link<Transform>>) -> bool {
        // early return if the parent is Some and doesnt exist
        if parent_option.is_some_and(|parent| !self.world_contains(parent)) {
            return false;
        }

        // replace childs parent
        let old_parent_option = std::mem::replace(&mut self.parent, parent_option);

        // if the old parent existed, remove the child from its children list
        let current_link = self.link;
        if let Some(old_parent_link) = old_parent_option {
            let mut old_parent = self.view(old_parent_link).unwrap();
            old_parent.children.remove(&current_link);
        };

        // if the parent was set to None we are done
        let Some(parent_link) = parent_option else {
            return true;
        };

        // add the child to the new parents children list
        let mut parent = self.view(parent_link).unwrap();
        parent.children.insert(current_link);

        // resolve recusrive tree branches by walking up each parent
        resolve_recursive(&mut parent, current_link, old_parent_option);
        return true;

        fn resolve_recursive(
            parent: &mut View<Transform>,
            source: Link<Transform>,
            old_parent_option: Option<Link<Transform>>,
        ) {
            match parent.parent {
                // if there are no more parents in the chain, early return
                None => (),
                // if next parent is not the source node, recurse up and check the next parent in the chain
                Some(next_parent_link) if next_parent_link != source => {
                    let mut next_parent = parent.view(next_parent_link).unwrap();
                    resolve_recursive(&mut next_parent, source, old_parent_option)
                }
                // if a recursive loop was found, remove the recursive child from the source node
                // and set the recursive childs parent to the source nodes old parent.
                // also add the recursive child to the old parents children list
                Some(next_parent_link) => {
                    let parent_link = parent.link;
                    parent.parent = old_parent_option;
                    let mut next_parent = parent.view(next_parent_link).unwrap();
                    next_parent.children.remove(&parent_link);
                    if let Some(old_parent_link) = old_parent_option {
                        let mut old_parent = next_parent.view(old_parent_link).unwrap();
                        old_parent.children.insert(parent_link);
                    }
                }
            }
        }
    }
}

#[extension_trait]
pub impl<'a> TransformView<'a> for View<'a, Transform> {
    fn parent(&mut self) -> Option<View<Transform>> {
        Some(self.view(self.parent?).unwrap())
    }

    fn walk_children(&mut self) -> ChildWalker<'_, 'a> {
        ChildWalker::new(self)
    }

    fn set_local_pos(&mut self, pos: Vec3) {
        if self.local_pos == pos {
            return;
        }

        self.local_pos = pos;
        self.sync_transforms();
    }

    fn set_local_rot(&mut self, rot: Quat) {
        if self.local_rot == rot {
            return;
        }

        self.local_rot = rot;
        self.sync_transforms();
    }

    fn set_local_scale(&mut self, scale: Vec3) {
        if self.local_scale == scale {
            return;
        }

        self.local_scale = scale;
        self.sync_transforms();
    }

    fn sync_transforms(&mut self) {
        self.pending_sync = false;
        self.local_matrix =
            Mat4::from_scale_rotation_translation(self.local_scale, self.local_rot, self.local_pos);
        let local_matrix = self.local_matrix;
        let world_matrix = match self.parent() {
            Some(parent) => parent.world_matrix * local_matrix,
            None => local_matrix,
        };

        self.world_matrix = world_matrix;
        (self.lossy_scale, self.world_rot, self.world_pos) =
            self.world_matrix.to_scale_rotation_translation();

        let mut children = self.walk_children();
        while let Some(mut child) = children.walk_next() {
            child.sync_transforms()
        }
    }

    fn unparent(&mut self) {
        PrivateTransformView::set_parent_no_sync(self, None);
    }

    fn set_parent(&mut self, parent: Link<Transform>) -> bool {
        if !TransformView::set_parent_no_sync(self, parent) {
            return false;
        }

        self.sync_transforms();
        return true;
    }

    fn set_parent_no_sync(&mut self, parent: Link<Transform>) -> bool {
        self.pending_sync = true;
        PrivateTransformView::set_parent_no_sync(self, Some(parent))
    }
}

pub struct ChildWalker<'a, 'source> {
    view: &'a mut View<'source, Transform>,
    children: Box<[Link<Transform>]>,
    current: usize,
}

impl<'a, 'source> ChildWalker<'a, 'source> {
    pub fn new(view: &'a mut View<'source, Transform>) -> Self {
        let children = view.child_links().copied().collect();
        Self {
            view,
            children,
            current: 0,
        }
    }

    pub fn walk_next(&mut self) -> Option<View<Transform>> {
        let link = *self.children.get(self.current)?;
        self.current += 1;
        Some(self.view.view(link).unwrap())
    }
}
