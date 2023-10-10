use boba_engine::prelude::*;

struct StringEvent;
impl Event for StringEvent {
    type Data<'a> = &'a str;
}

#[pearl(listen(StringEvent))]
pub struct Test1 {
    item: u32,
}

impl EventListener<StringEvent> for Test1 {
    fn update(event: &mut &str, world: &mut BobaWorld) {
        let global = world.get_global::<TestResource>().unwrap().item;
        for test in world.iter::<Test1>() {
            let item = test.item;
            println!("Got event: {event} on pearl Test1 {{ item: {item} }} with global {global}");
        }
    }
}

#[pearl]
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
