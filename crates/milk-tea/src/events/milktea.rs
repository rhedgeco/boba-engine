use std::{
    ops::{Deref, DerefMut},
    time::Instant,
};

use boba_core::World;
use winit::event_loop::EventLoopWindowTarget;

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
            target_defer: Vec::new(),
            delta_time: self.delta_time,
            game_time: self.game_time,
            exiting: self.target.exiting(),
            event,
        };

        world.trigger(&mut milk_tea);

        // run deferred functions
        for deferred in milk_tea.target_defer {
            deferred(world, self.target);
        }

        // trigger app exit
        if milk_tea.exiting {
            self.target.exit()
        }

        milk_tea.event
    }
}

pub struct MilkTea<T> {
    target_defer: Vec<Box<dyn FnOnce(&mut World, &EventLoopWindowTarget<()>)>>,
    delta_time: f32,
    game_time: f32,
    exiting: bool,
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

    pub fn exit_app(&mut self) {
        self.exiting = true;
    }

    pub fn exiting(&self) -> bool {
        self.exiting
    }

    pub(crate) fn target_defer(
        &mut self,
        defer: impl FnOnce(&mut World, &EventLoopWindowTarget<()>) + 'static,
    ) {
        self.target_defer.push(Box::new(defer));
    }
}
