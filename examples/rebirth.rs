use boba_engine::prelude::*;

struct Phoenix(u64);

impl Pearl for Phoenix {
    fn register(source: &mut impl EventSource<Self>) {
        source.listen::<MilkTeaUpdate>();
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

impl Listener<MilkTeaUpdate> for Phoenix {
    fn trigger(mut view: PearlView<Self>, _: &mut MilkTeaUpdate) {
        println!(
            "A NEW LIFE BURSTS FORTH FROM THE ASHES: GENERATION {}!",
            view.0
        );
        view.queue_destroy(view.link());
        let next_phoenix = Phoenix(view.0 + 1);
        view.world_mut().insert(next_phoenix);
    }
}

fn main() {
    env_logger::init();
    let mut world = World::new();
    world.insert(Phoenix(0));
    run_headless(&mut world);
}
