use std::{
    marker::PhantomData,
    ops::{Deref, DerefMut},
    time::Instant,
};

use boba_core::pearl::Event;
use winit::event_loop::EventLoopWindowTarget;

pub trait MilkTeaEvent: 'static {
    type Data<'a>;
}

pub trait SimpleMilkTeaEvent: 'static {}
impl<T: SimpleMilkTeaEvent> MilkTeaEvent for T {
    type Data<'a> = Self;
}

pub struct Update;
impl SimpleMilkTeaEvent for Update {}

pub(crate) struct EventTimer {
    start_time: Option<Instant>,
    update_time: Instant,
    delta_time: f32,
    game_time: f32,
}

impl EventTimer {
    pub fn new() -> Self {
        Self {
            start_time: None,
            update_time: Instant::now(),
            delta_time: 0.,
            game_time: 0.,
        }
    }

    pub fn update_timer(&mut self) {
        let now = Instant::now();
        self.game_time = match self.start_time {
            Some(start_time) => now.duration_since(start_time).as_secs_f32(),
            None => {
                self.start_time = Some(now);
                0.
            }
        };

        self.delta_time = now.duration_since(self.update_time).as_secs_f32();
        self.update_time = now;
    }

    // pub fn build<'a, T: MilkTeaEvent>(
    //     &self,
    //     data: T::Data<'a>,
    //     target: &'a EventLoopWindowTarget<()>,
    // ) -> Data<'a, T> {
    //     Data {
    //         game_time: self.game_time,
    //         delta_time: self.delta_time,
    //         target,
    //         data,
    //     }
    // }

    pub fn build_simple<'a, T: SimpleMilkTeaEvent>(
        &self,
        data: T,
        target: &'a EventLoopWindowTarget<()>,
    ) -> Data<'a, T> {
        Data {
            game_time: self.game_time,
            delta_time: self.delta_time,
            target,
            data,
        }
    }
}

pub struct MilkTea<T: MilkTeaEvent> {
    _type: PhantomData<T>,
}

impl<T: MilkTeaEvent> Event for MilkTea<T> {
    type Data<'a> = Data<'a, T>;
}

pub struct Data<'a, T: MilkTeaEvent> {
    game_time: f32,
    delta_time: f32,
    target: &'a EventLoopWindowTarget<()>,
    data: T::Data<'a>,
}

impl<'a, T: MilkTeaEvent> DerefMut for Data<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}

impl<'a, T: MilkTeaEvent> Deref for Data<'a, T> {
    type Target = T::Data<'a>;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<'a, T: MilkTeaEvent> Data<'a, T> {
    pub fn game_time(&self) -> f32 {
        self.game_time
    }

    pub fn delta_time(&self) -> f32 {
        self.delta_time
    }

    pub fn exit_app(&self) {
        self.target.exit()
    }

    pub fn is_exiting(&self) -> bool {
        self.target.exiting()
    }

    pub(crate) fn window_target(&self) -> &EventLoopWindowTarget<()> {
        self.target
    }
}
