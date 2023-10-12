mod lifecycle;
mod window;

pub use lifecycle::*;
pub use window::*;

use boba_core::Event;
use std::{
    marker::PhantomData,
    ops::{Deref, DerefMut},
    time::Instant,
};
use winit::event_loop::{ControlFlow, EventLoopWindowTarget};

pub struct MilkTeaEvent<T: 'static> {
    _type: PhantomData<*const T>,
}

pub struct MilkTeaEventData<'a, T: 'static> {
    delta_time: f64,
    target: &'a EventLoopWindowTarget<()>,
    control: &'a mut ControlFlow,
    data: T,
}

impl<T: 'static> Event for MilkTeaEvent<T> {
    type Data<'a> = MilkTeaEventData<'a, T>;
}

impl<'a, T: 'static> DerefMut for MilkTeaEventData<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}

impl<'a, T: 'static> Deref for MilkTeaEventData<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<'a, T: 'static> MilkTeaEventData<'a, T> {
    pub fn delta_time(&self) -> f64 {
        self.delta_time
    }

    pub fn window_target(&self) -> &EventLoopWindowTarget<()> {
        self.target
    }

    pub fn control_flow(&self) -> &ControlFlow {
        &self.control
    }

    pub fn control_flow_mut(&mut self) -> &mut ControlFlow {
        self.control
    }
}

#[derive(Default)]
pub(crate) struct MilkTeaTimer {
    instant: Option<Instant>,
}

impl MilkTeaTimer {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn instant<'a>(
        &'a mut self,
        target: &'a EventLoopWindowTarget<()>,
        control: &'a mut ControlFlow,
    ) -> MilkTeaTimerInstant<'a> {
        let now = Instant::now();
        let delta_time = match self.instant {
            Some(last) => now.duration_since(last).as_secs_f64(),
            None => 0f64,
        };
        self.instant = Some(now);

        MilkTeaTimerInstant {
            delta_time,
            target,
            control,
        }
    }
}

pub struct MilkTeaTimerInstant<'a> {
    delta_time: f64,
    target: &'a EventLoopWindowTarget<()>,
    control: &'a mut ControlFlow,
}

impl<'a> MilkTeaTimerInstant<'a> {
    pub fn build<T: 'static>(&mut self, data: T) -> MilkTeaEventData<T> {
        MilkTeaEventData {
            delta_time: self.delta_time,
            target: self.target,
            control: self.control,
            data,
        }
    }
}
