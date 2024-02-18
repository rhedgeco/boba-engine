use std::rc::Rc;

use boba_core::{
    world::{Link, PearlView, WorldQueue},
    Pearl, World,
};
use extension_trait::extension_trait;
use indexmap::IndexSet;

pub type Listener<P, T> = fn(PearlView<P>, &mut T);
pub(crate) type Sender<T> = Rc<dyn Fn(&mut WorldQueue, &mut T)>;

pub struct SignalBuilder<T: 'static> {
    links: IndexSet<Link<()>>,
    listeners: Vec<Sender<T>>,
}

impl<T: 'static> Default for SignalBuilder<T> {
    fn default() -> Self {
        Self {
            links: IndexSet::new(),
            listeners: Vec::new(),
        }
    }
}

impl<T: 'static> SignalBuilder<T> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn build(&self, message: T) -> Signal<T> {
        Signal {
            message,
            listeners: self.listeners.clone(),
        }
    }
}

#[extension_trait]
pub impl<T: 'static> SignalRegister<T> for SignalBuilder<T> {
    fn add_listener<P: Pearl>(&mut self, link: Link<P>, listener: Listener<P, T>) {
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

pub struct Signal<T: 'static> {
    message: T,
    listeners: Vec<Sender<T>>,
}

impl<T: 'static> Signal<T> {
    pub fn send(mut self, world: &mut WorldQueue) {
        for listener in self.listeners {
            listener(world, &mut self.message);
        }
    }
}

#[extension_trait]
pub impl<T: 'static> WorldSignalExt<T> for WorldQueue<'_> {
    fn send_signal(&mut self, signal: Signal<T>) {
        signal.send(self)
    }
}

impl<T: 'static> WorldSignalExt<T> for World {
    fn send_signal(&mut self, signal: Signal<T>) {
        let mut queue = WorldQueue::new(self);
        queue.send_signal(signal);
    }
}
