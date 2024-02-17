use std::rc::Rc;

use extension_trait::extension_trait;
use indexmap::IndexSet;

use crate::{
    world::{Link, PearlView, WorldQueue},
    Pearl,
};

pub(crate) type Listener<T> = Rc<dyn Fn(&mut WorldQueue, &mut T)>;

pub struct Signal<T> {
    links: IndexSet<Link<()>>,
    listeners: Vec<Listener<T>>,
}

impl<T> Default for Signal<T> {
    fn default() -> Self {
        Self {
            links: Default::default(),
            listeners: Default::default(),
        }
    }
}

impl<T: 'static> Signal<T> {
    pub fn new() -> Self {
        Self::default()
    }
}

#[extension_trait]
pub impl<T> SignalSend<T> for Signal<T> {
    fn commands(&self, data: T) -> SignalCommands<T> {
        SignalCommands {
            data,
            listeners: self.listeners.clone(),
        }
    }
}

#[extension_trait]
pub impl<T: 'static> SignalRegister<T> for Signal<T> {
    fn add_listener<P: Pearl>(&mut self, link: Link<P>, listener: fn(PearlView<P>, &mut T)) {
        let (index, _) = self.links.insert_full(link.into_type());
        self.listeners.insert(
            index,
            Rc::new(move |world, data| {
                if let Some(view) = PearlView::new(link, world) {
                    listener(view, data);
                }
            }),
        );
    }

    fn remove_listener<P: Pearl>(&mut self, link: &Link<P>) -> bool {
        let Some((index, _)) = self.links.shift_remove_full(&link.into_type()) else {
            return false;
        };

        self.listeners.remove(index);
        true
    }
}

pub struct SignalCommands<T> {
    data: T,
    listeners: Vec<Rc<dyn Fn(&mut WorldQueue, &mut T)>>,
}

impl<T> SignalCommands<T> {
    pub fn send(mut self, world: &mut WorldQueue) {
        for listener in self.listeners {
            listener(world, &mut self.data);
        }
    }
}
