use boba_engine::prelude::*;

#[pearl(listen(Update))]
pub struct FpsPrinter;

impl EventListener<Update> for FpsPrinter {
    fn update(event: &mut Update, _: &mut PearlAccess<Self>, _: &mut Resources) {
        let fps = 1f64 / event.delta_time();
        println!("FPS: {fps}");
    }
}

fn main() {
    let mut milk_tea = MilkTeaRunner::default();
    milk_tea.world.insert_pearl(FpsPrinter);
    milk_tea.run();
}
