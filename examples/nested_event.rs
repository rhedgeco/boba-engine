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
pub struct NestTrigger {
    value: usize,
}

impl EventListener<Update> for NestTrigger {
    fn update(_: &Update, arena: &mut ArenaView<Self>) {
        let value = arena.current_pearl().value;
        println!("BASE UPDATE CALL from {value}");
        arena.trigger(&mut StringEvent {
            string: format!("NESTED CALL from {value}"),
        });
    }
}

impl EventListener<StringEvent> for NestTrigger {
    fn update(string: &str, arena: &mut ArenaView<Self>) {
        let value = arena.current_pearl().value;
        println!("STRING EVENT on {value}: {string}");
    }
}

fn main() {
    let mut milk_tea = MilkTeaRunner::default();
    milk_tea.arena.insert(NestTrigger { value: 1 });
    milk_tea.arena.insert(NestTrigger { value: 2 });
    milk_tea.run();
}
