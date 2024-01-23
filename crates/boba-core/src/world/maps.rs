use std::any::Any;

use handle_map::{map::DenseHandleMap, Handle};

use crate::Pearl;

pub type PearlMap<P> = DenseHandleMap<P>;

pub trait AnyPearlMap: Any {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
    fn destroy(&mut self, handle: Handle<()>) -> bool;
}

impl<P: Pearl> AnyPearlMap for PearlMap<P> {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn destroy(&mut self, handle: Handle<()>) -> bool {
        self.remove(handle.into_type()).is_some()
    }
}

impl dyn AnyPearlMap {
    pub fn as_map<P: 'static>(&self) -> &PearlMap<P> {
        self.as_any()
            .downcast_ref::<PearlMap<P>>()
            .expect("invalid map cast")
    }

    pub fn as_map_mut<P: 'static>(&mut self) -> &mut PearlMap<P> {
        self.as_any_mut()
            .downcast_mut::<PearlMap<P>>()
            .expect("invalid map cast")
    }
}
