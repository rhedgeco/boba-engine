use boba_core::{
    world::{Link, PearlView, WorldQueue},
    Pearl, World,
};
use extension_trait::extension_trait;
use indexmap::IndexSet;

pub type Listener<P, T> = fn(PearlView<P>, &mut T);

struct ListenerStore<T> {
    link: Link<()>,
    listener_ptr: *const (),
    runner: fn(Link<()>, &mut WorldQueue, *const (), &mut T),
}

impl<T> Copy for ListenerStore<T> {}
impl<T> Clone for ListenerStore<T> {
    fn clone(&self) -> Self {
        Self {
            link: self.link.clone(),
            listener_ptr: self.listener_ptr.clone(),
            runner: self.runner.clone(),
        }
    }
}

impl<T> ListenerStore<T> {
    pub fn new<P: Pearl>(link: Link<P>, listener: Listener<P, T>) -> Self {
        Self {
            link: link.into_type(),
            // WARNING: dangerous type erasure goes on here to lift 'static restriction on T
            listener_ptr: listener as *const (),
            runner: |link, queue, listener_ptr, message| {
                let link = link.into_type::<P>();
                if let Some(view) = PearlView::new(link, queue) {
                    // SAFETY: listener_ptr was created from an identical fn pointer
                    let listener: Listener<P, T> = unsafe { core::mem::transmute(listener_ptr) };
                    listener(view, message);
                }
            },
        }
    }

    pub fn send_to(self, world: &mut WorldQueue, message: &mut T) {
        (self.runner)(self.link, world, self.listener_ptr, message);
    }
}

pub struct SignalBuilder<T> {
    links: IndexSet<Link<()>>,
    listeners: Vec<ListenerStore<T>>,
}

impl<T> Default for SignalBuilder<T> {
    fn default() -> Self {
        Self {
            links: IndexSet::new(),
            listeners: Vec::new(),
        }
    }
}

impl<T> SignalBuilder<T> {
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
pub impl<T> SignalRegister<T> for SignalBuilder<T> {
    fn add_listener<P: Pearl>(&mut self, link: Link<P>, listener: Listener<P, T>) {
        let (index, _) = self.links.insert_full(link.into_type());
        self.listeners
            .insert(index, ListenerStore::new(link, listener));
    }

    fn remove_listener<P: Pearl>(&mut self, link: &Link<P>) -> bool {
        let Some((index, _)) = self.links.shift_remove_full(&link.into_type()) else {
            return false;
        };

        self.listeners.remove(index);
        true
    }
}

pub struct Signal<T> {
    message: T,
    listeners: Vec<ListenerStore<T>>,
}

impl<T> Signal<T> {
    pub fn send(mut self, world: &mut WorldQueue) {
        for listener in self.listeners {
            listener.send_to(world, &mut self.message);
        }
    }
}

#[extension_trait]
pub impl<T> WorldSignalExt<T> for WorldQueue<'_> {
    fn send_signal(&mut self, signal: Signal<T>) {
        signal.send(self)
    }
}

impl<T> WorldSignalExt<T> for World {
    fn send_signal(&mut self, signal: Signal<T>) {
        let mut queue = WorldQueue::new(self);
        queue.send_signal(signal);
    }
}
