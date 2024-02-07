use boba_engine::prelude::*;

struct Phoenix(u64);

impl Pearl for Phoenix {
    fn register(source: &mut impl EventSource<Self>) {
        source.listen::<Update>();
    }

    fn on_insert(_: InsertContext<Self>) {
        println!("A FIRE IS STARTED!");
    }

    fn on_remove(context: RemoveContext<Self>) {
        println!(
            "THE DEATH OF GENERATION {} WILL BRING LIFE TO ANOTHER!",
            context.pearl.0
        );
    }
}

impl Listener<Update> for Phoenix {
    fn trigger(mut view: PearlView<Self>, _: &mut Update) {
        println!(
            "A NEW LIFE BURSTS FORTH FROM THE ASHES: GENERATION {}!",
            view.0
        );
        let next_phoenix = Phoenix(view.0 + 1);
        view.world_mut().insert(next_phoenix);
        view.destroy_self();
    }
}

fn main() {
    env_logger::init();
    let mut world = World::new();
    world.insert(Phoenix(0));
    milk_tea::run(&mut world);
}
