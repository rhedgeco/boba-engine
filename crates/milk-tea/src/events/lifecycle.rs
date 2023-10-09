use std::time::Instant;

use boba_core::Event;
use winit::event_loop::{ControlFlow, EventLoopWindowTarget};

pub struct StartData<'a> {
    target: &'a EventLoopWindowTarget<()>,
    control: &'a mut ControlFlow,
}

impl<'a> StartData<'a> {
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

pub struct MilkTeaStart {
    _private: (),
}

impl Event for MilkTeaStart {
    type Data<'a> = StartData<'a>;
}

impl MilkTeaStart {
    pub(crate) fn create_data<'a>(
        target: &'a EventLoopWindowTarget<()>,
        control: &'a mut ControlFlow,
    ) -> StartData<'a> {
        StartData { target, control }
    }
}

pub struct MilkTeaExit {
    _private: (),
}

impl Event for MilkTeaExit {
    type Data<'a> = ();
}

pub struct UpdateData<'a> {
    delta_time: f64,
    target: &'a EventLoopWindowTarget<()>,
    control: &'a mut ControlFlow,
}

impl<'a> UpdateData<'a> {
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

pub struct MilkTeaUpdate {
    instant: Option<Instant>,
}

impl Event for MilkTeaUpdate {
    type Data<'a> = UpdateData<'a>;
}

impl MilkTeaUpdate {
    pub(crate) fn new() -> Self {
        Self { instant: None }
    }

    pub fn next_data<'a>(
        &mut self,
        target: &'a EventLoopWindowTarget<()>,
        control: &'a mut ControlFlow,
    ) -> UpdateData<'a> {
        let now = Instant::now();
        let delta_time = match self.instant {
            Some(last) => now.duration_since(last).as_secs_f64(),
            None => 0f64,
        };
        self.instant = Some(now);

        UpdateData {
            delta_time,
            target,
            control,
        }
    }
}
