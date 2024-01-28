use std::{
    marker::PhantomData,
    ops::{Deref, DerefMut},
};

use handle_map::Handle;
use indexmap::IndexMap;

use crate::{
    pearl::{Event, SimpleEvent},
    Pearl, World,
};

use super::{AnyMapBox, Iter, IterMut, Link};

struct DestroyCommand {
    link: Link<()>,
    command: fn(Link<()>, &mut World),
}

impl DestroyCommand {
    pub fn new<P: Pearl>(link: Link<P>) -> Self {
        fn destroy<P: Pearl>(link: Link<()>, world: &mut World) {
            world.remove(link.into_type::<P>());
        }

        Self {
            link: link.into_type(),
            command: destroy::<P>,
        }
    }

    pub fn execute_on(&self, world: &mut World) {
        (self.command)(self.link, world);
    }
}

#[derive(Default)]
pub(super) struct DestroyQueue {
    commands: IndexMap<Link<()>, DestroyCommand>,
}

impl DestroyQueue {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert<P: Pearl>(&mut self, link: Link<P>) -> bool {
        use indexmap::map::Entry as E;
        match self.commands.entry(link.into_type()) {
            E::Occupied(_) => return false,
            E::Vacant(e) => {
                e.insert(DestroyCommand::new(link));
                return true;
            }
        }
    }

    pub fn execute_on(&self, world: &mut World) {
        for command in self.commands.values() {
            command.execute_on(world);
        }
    }
}

pub struct DropContext<'a, 'view, P: Pearl> {
    pub view: &'a mut View<'view, P>,
    _private: (),
}

pub struct Walker<'a, P> {
    world: &'a mut World,
    destroy_queue: &'a mut DestroyQueue,
    map_handle: Handle<AnyMapBox>,
    data_index: usize,
    data_cap: usize,

    _type: PhantomData<*const P>,
}

impl<'a, P: Pearl> Walker<'a, P> {
    pub(super) fn new(world: &'a mut World, destroy_queue: &'a mut DestroyQueue) -> Option<Self> {
        let map_handle = world.get_map_handle::<P>()?;
        let data_cap = world.get_map_with::<P>(map_handle).unwrap().len();
        Some(Self {
            world,
            destroy_queue,
            map_handle,
            data_index: 0,
            data_cap,
            _type: PhantomData,
        })
    }

    pub fn next(&mut self) -> Option<View<P>> {
        if self.data_index == self.data_cap {
            return None;
        }

        let map = self.world.get_map_with::<P>(self.map_handle).unwrap();
        let data_handle = map.get_index(self.data_index)?.0;
        self.data_index += 1;

        Some(View {
            link: Link {
                map_handle: self.map_handle,
                data_handle,
            },
            world: self.world,
            destroy_queue: self.destroy_queue,
            _type: PhantomData,
        })
    }
}

pub struct View<'a, P: Pearl> {
    link: Link<P>,
    world: &'a mut World,
    destroy_queue: &'a mut DestroyQueue,
    _type: PhantomData<*const P>,
}

impl<'a, P: Pearl> Drop for View<'a, P> {
    fn drop(&mut self) {
        P::on_view_drop(DropContext {
            view: self,
            _private: (),
        })
    }
}

impl<'a, P: Pearl> DerefMut for View<'a, P> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.world.get_mut(self.link).unwrap()
    }
}

impl<'a, P: Pearl> Deref for View<'a, P> {
    type Target = P;

    fn deref(&self) -> &Self::Target {
        self.world.get(self.link).unwrap()
    }
}

impl<'a, P: Pearl> View<'a, P> {
    pub(super) fn new(
        link: Link<P>,
        world: &'a mut World,
        destroy_queue: &'a mut DestroyQueue,
    ) -> Option<Self> {
        if !world.contains(link) {
            return None;
        }

        Some(Self {
            link,
            world,
            destroy_queue,
            _type: PhantomData,
        })
    }

    pub fn current_link(&self) -> Link<P> {
        self.link
    }

    pub fn count<P2: Pearl>(&self) -> usize {
        self.world.count::<P2>()
    }

    pub fn has_type<P2: Pearl>(&self) -> bool {
        self.world.has_type::<P2>()
    }

    pub fn contains<P2: Pearl>(&self, link: Link<P2>) -> bool {
        self.world.contains(link)
    }

    pub fn contains_global<P2: Pearl>(&self) -> bool {
        self.world.contains_global::<P2>()
    }

    pub fn get<P2: Pearl>(&self, link: Link<P2>) -> Option<&P2> {
        self.world.get(link)
    }

    pub fn get_global<P2: Pearl>(&self) -> Option<&P2> {
        self.world.get_global::<P2>()
    }

    pub fn view<P2: Pearl>(&mut self, link: Link<P2>) -> Option<View<P2>> {
        View::new(link, self.world, self.destroy_queue)
    }

    pub fn view_global<P2: Pearl>(&mut self) -> Option<View<P2>> {
        let link = self.world.get_global_link::<P2>()?;
        View::new(link, self.world, self.destroy_queue)
    }

    pub fn iter<P2: Pearl>(&self) -> Iter<P2> {
        self.world.iter::<P2>()
    }

    pub fn iter_mut<P2: Pearl>(&mut self) -> IterMut<P2> {
        // iterating mutably will not disturb any indices
        self.world.iter_mut::<P2>()
    }

    pub fn insert<P2: Pearl>(&mut self, data: P2) -> Link<P2> {
        // inserting does not disturb any indices
        self.world.insert(data)
    }

    pub fn trigger<E: Event>(&mut self, event: &E::Data<'a>) -> bool {
        self.world.nested_trigger::<E>(event, self.destroy_queue)
    }

    pub fn trigger_simple<E: SimpleEvent>(&mut self, event: &E) -> bool {
        self.world.nested_trigger::<E>(event, self.destroy_queue)
    }

    pub fn destroy<P2: Pearl>(&mut self, link: Link<P2>) -> bool {
        // check if the link is valid
        if !self.world.contains(link) {
            return false;
        }

        // we have to queue removals because removing data disturbs indices
        self.destroy_queue.insert(link)
    }

    pub fn destroy_global<P2: Pearl>(&mut self) -> bool {
        let Some(link) = self.world.get_global_link::<P2>() else {
            return false;
        };

        self.destroy_queue.insert(link)
    }
}
