use boba_engine::prelude::*;

struct StringEvent {
    string: String,
}

impl Event for StringEvent {
    type Data<'a> = &'a str;
    fn event_data<'a>(&'a mut self) -> Self::Data<'a> {
        &self.string
    }
}

#[pearl(listen(Update, StringEvent))]
pub struct NestTrigger;

impl EventListener<Update> for NestTrigger {
    fn update(_: &Update, arena: &mut ArenaView<Self>) {
        println!("BASE UPDATE CALL");
        arena.trigger(&mut StringEvent {
            string: format!("NESTED CALL"),
        });
    }
}

impl EventListener<StringEvent> for NestTrigger {
    fn update(string: &str, _: &mut ArenaView<Self>) {
        println!("STRING EVENT: {string}");
    }
}

fn main() {
    let mut milk_tea = MilkTeaRunner::default();
    milk_tea.arena.insert(NestTrigger);
    milk_tea.arena.insert(NestTrigger);
    milk_tea.run();
}
