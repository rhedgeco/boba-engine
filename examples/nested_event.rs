use boba_engine::prelude::*;

#[pearl(listen(Update, String))]
pub struct NestTrigger;

impl EventListener<Update> for NestTrigger {
    fn update(_: &mut Update, arena: &mut ArenaView<Self>) {
        println!("BASE UPDATE CALL");
        arena.trigger(&mut format!("NESTED CALL"));
    }
}

impl EventListener<String> for NestTrigger {
    fn update(string: &mut String, _: &mut ArenaView<Self>) {
        println!("STRING EVENT: {string}");
    }
}

fn main() {
    let mut milk_tea = MilkTeaRunner::default();
    milk_tea.arena.insert(NestTrigger);
    milk_tea.arena.insert(NestTrigger);
    milk_tea.run();
}
