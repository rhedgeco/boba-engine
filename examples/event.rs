use boba_engine::prelude::*;

#[pearl(listen(String))]
pub struct Test1 {
    item: u32,
}

impl EventListener<String> for Test1 {
    fn update(event: &mut String, pearls: &mut PearlArenaView<Self>, resources: &mut Resources) {
        let item = pearls.current().item;
        let resource = resources.get::<TestResource>().unwrap().item;
        println!("Got event: {event} on pearl Test1 {{ item: {item} }} with resource {resource}");
    }
}

struct TestResource {
    item: u32,
}

use boba_core::Resources;
use boba_engine::prelude::PearlArena;

fn main() {
    let mut pearls = PearlArena::new();
    let mut resources = Resources::new();
    pearls.insert(Test1 { item: 42 });
    pearls.insert(Test1 { item: 69 });
    resources.insert(TestResource { item: 1234 });
    pearls.trigger(&mut format!("String Event"), &mut resources);
}
