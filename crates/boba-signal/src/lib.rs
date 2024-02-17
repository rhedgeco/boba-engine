use std::rc::Rc;

use boba_core::{
    world::{Link, PearlView, WorldQueue},
    Pearl, World,
};
use extension_trait::extension_trait;
use indexmap::IndexSet;

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

    pub fn command(&self, data: T) -> SignalCommand<T> {
        SignalCommand {
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

pub struct SignalCommand<T> {
    data: T,
    listeners: Vec<Rc<dyn Fn(&mut WorldQueue, &mut T)>>,
}

impl<T> SignalCommand<T> {
    pub fn send(mut self, world: &mut WorldQueue) {
        for listener in self.listeners {
            listener(world, &mut self.data);
        }
    }
}

#[extension_trait]
pub impl<T> SignalSender<T> for WorldQueue<'_> {
    fn send_signal(&mut self, command: SignalCommand<T>) {
        command.send(self)
    }
}

impl<T> SignalSender<T> for World {
    fn send_signal(&mut self, command: SignalCommand<T>) {
        let mut queue = WorldQueue::new(self);
        queue.send_signal(command);
    }
}
