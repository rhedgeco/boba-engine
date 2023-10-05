use boba_engine::prelude::*;

pub struct Test1 {
    item: u32,
}

impl Pearl for Test1 {
    fn register(register: &mut impl EventRegister<Self>) {
        register.event::<String>()
    }
}

impl EventListener<String> for Test1 {
    fn update(event: &mut String, pearls: &mut PearlAccess<Self>) {
        let item = pearls.current().item;
        println!("Got event: {event} on pearl Test1 {{ item: {item} }}");
    }
}

fn main() {
    let mut world = World::new();
    world.insert_pearl(Test1 { item: 42 });
    world.insert_pearl(Test1 { item: 69 });
    world.trigger(&mut format!("String Event"));
}
