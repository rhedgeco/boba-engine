use boba_engine::prelude::*;
use milk_tea::{window::WindowRenderer, winit::window::Window};

fn main() {
    let mut milk_tea = MilkTea::new();

    milk_tea
        .world
        .insert(MilkTeaWindow::<TestRenderer>::new(WindowConfig::default()));

    milk_tea
        .world
        .insert(MilkTeaWindow::<TestRenderer>::new(WindowConfig::default()));

    milk_tea.run();
}

struct TestRenderer {
    window: Window,
}

impl WindowRenderer for TestRenderer {
    fn init(
        _: pearl::Handle<MilkTeaWindow<Self>>,
        window: milk_tea::winit::window::Window,
    ) -> Self {
        Self { window }
    }

    fn id(&self) -> milk_tea::winit::window::WindowId {
        self.window.id()
    }

    fn render(&mut self, _: &BobaWorld) {
        println!("TestRendering window {:?}", self.id());
    }
}
