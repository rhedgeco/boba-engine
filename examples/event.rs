use boba_engine::prelude::*;

#[pearl(listen(String))]
pub struct Test1 {
    item: u32,
}

impl EventListener<String> for Test1 {
    fn update(event: &mut String, arena: &mut ArenaView<Self>) {
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
    arena.trigger(&mut format!("String Event"));
}
