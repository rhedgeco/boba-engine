use std::{
    ops::{Deref, DerefMut},
    time::Instant,
};

use boba_core::World;
use winit::event_loop::EventLoopWindowTarget;

pub struct Update {
    _private: (),
}

impl Update {
    pub(crate) fn new() -> Self {
        Self { _private: () }
    }
}

struct TimerData {
    start: Instant,
    update: Instant,
}

pub(crate) struct EventTimer {
    timer_data: Option<TimerData>,
}

impl EventTimer {
    pub fn new() -> Self {
        Self { timer_data: None }
    }

    pub fn next<'a>(&mut self, target: &'a EventLoopWindowTarget<()>) -> MilkTeaExecutor<'a> {
        let now = Instant::now();
        let Some(timer_data) = &mut self.timer_data else {
            self.timer_data = Some(TimerData {
                start: now,
                update: now,
            });
            return MilkTeaExecutor {
                target,
                delta_time: 0.,
                game_time: 0.,
            };
        };

        let builder = MilkTeaExecutor {
            target,
            delta_time: now.duration_since(timer_data.update).as_secs_f32(),
            game_time: now.duration_since(timer_data.start).as_secs_f32(),
        };
        timer_data.update = now;
        builder
    }
}

pub(crate) struct MilkTeaExecutor<'a> {
    target: &'a EventLoopWindowTarget<()>,
    delta_time: f32,
    game_time: f32,
}

impl<'a> Drop for MilkTeaExecutor<'a> {
    fn drop(&mut self) {}
}

impl<'a> MilkTeaExecutor<'a> {
    pub fn trigger<T: 'static>(&self, world: &mut World, event: T) -> T {
        let mut milk_tea = MilkTea {
            target: self.target,
            delta_time: self.delta_time,
            game_time: self.game_time,
            event,
        };

        world.trigger(&mut milk_tea);
        milk_tea.event
    }
}

pub struct MilkTea<T> {
    target: *const EventLoopWindowTarget<()>,
    delta_time: f32,
    game_time: f32,
    event: T,
}

impl<'a, T> DerefMut for MilkTea<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.event
    }
}

impl<'a, T> Deref for MilkTea<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.event
    }
}

impl<T> MilkTea<T> {
    pub fn game_time(&self) -> f32 {
        self.game_time
    }

    pub fn delta_time(&self) -> f32 {
        self.delta_time
    }

    pub fn exit_app(&self) {
        self.window_target().exit()
    }

    pub fn exiting(&self) -> bool {
        self.window_target().exiting()
    }

    pub(crate) fn window_target(&self) -> &EventLoopWindowTarget<()> {
        // SAFETY: MilkTea cannot be constructed externally
        // and is only valid within the scope of MilkTeaExecutor.
        // Since the target reference is valid during the length of MilkTeaExecutor,
        // this will always be safe. This is needed to lift the liftime bound from MilkTea.
        unsafe { &*self.target }
    }
}
