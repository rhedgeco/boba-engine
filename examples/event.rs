use boba_engine::prelude::*;

struct StringEvent;
impl Event for StringEvent {
    type Data<'a> = &'a str;
}

pub struct Test1 {
    item: u32,
}

impl Pearl for Test1 {
    fn register(register: &mut impl EventRegister<Self>) {
        register.event::<StringEvent>();
    }
}

impl EventListener<StringEvent> for Test1 {
    fn update(event: &mut &str, world: &mut BobaWorld) {
        let global = world.get_global::<TestResource>().unwrap().item;
        for test in world.iter::<Test1>().filter_map(|e| e.borrow()) {
            let item = test.item;
            println!("Got event: {event} on pearl Test1 {{ item: {item} }} with global {global}");
        }
    }
}

impl Pearl for TestResource {}
struct TestResource {
    item: u32,
}

fn main() {
    let mut world = BobaWorld::new();
    world.insert(Test1 { item: 42 });
    world.insert(Test1 { item: 69 });
    world.insert_global(TestResource { item: 1234 });
    world.trigger::<StringEvent>("String Event");
}
