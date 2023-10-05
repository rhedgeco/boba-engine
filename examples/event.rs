use boba_engine::prelude::*;

#[pearl(listen(String))]
pub struct Test1 {
    item: u32,
}

impl EventListener<String> for Test1 {
    fn update(event: &mut String, pearls: &mut PearlAccess<Self>, resources: &mut Resources) {
        let item = pearls.current().item;
        let resource = resources.get::<TestResource>().unwrap().item;
        println!("Got event: {event} on pearl Test1 {{ item: {item} }} with resource {resource}");
    }
}

struct TestResource {
    item: u32,
}

fn main() {
    let mut world = World::new();
    world.insert_pearl(Test1 { item: 42 });
    world.insert_pearl(Test1 { item: 69 });
    world.insert_resource(TestResource { item: 1234 });
    world.trigger(&mut format!("String Event"));
}
