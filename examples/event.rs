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

#[pearl(listen(StringEvent))]
pub struct Test1 {
    item: u32,
}

impl EventListener<StringEvent> for Test1 {
    fn update(event: &str, arena: &mut ArenaView<Self>) {
        let item = arena.current_pearl().item;
        let resource = arena.resources().get::<TestResource>().unwrap().item;
        println!("Got event: {event} on pearl Test1 {{ item: {item} }} with resource {resource}");
    }
}

struct TestResource {
    item: u32,
}

fn main() {
    let mut arena = BobaArena::new();
    arena.insert(Test1 { item: 42 });
    arena.insert(Test1 { item: 69 });
    arena.resources_mut().insert(TestResource { item: 1234 });
    arena.trigger(&mut StringEvent {
        string: format!("String Event"),
    });
}
