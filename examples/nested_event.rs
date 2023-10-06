use boba_engine::prelude::*;

#[pearl(listen(Update, String))]
pub struct NestTrigger;

impl EventListener<Update> for NestTrigger {
    fn update(_: &mut Update, pearls: &mut PearlArenaView<Self>, resources: &mut Resources) {
        println!("BASE UPDATE CALL");
        pearls.trigger(&mut format!("NESTED CALL"), resources);
    }
}

impl EventListener<String> for NestTrigger {
    fn update(string: &mut String, _: &mut PearlArenaView<Self>, _: &mut Resources) {
        println!("STRING EVENT: {string}");
    }
}

fn main() {
    let mut milk_tea = MilkTeaRunner::default();
    milk_tea.pearls.insert(NestTrigger);
    milk_tea.pearls.insert(NestTrigger);
    milk_tea.run();
}
