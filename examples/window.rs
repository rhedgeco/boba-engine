use boba_engine::prelude::*;

fn main() {
    let mut milk_tea = MilkTea::new();
    milk_tea.world.insert(WindowBuilder {
        title: format!("Boba Engine"),
    });

    milk_tea.run();
}
