use boba_engine::prelude::*;

struct Update;
impl SimpleEvent for Update {}

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
    fn update(view: &mut View<'_, Self>, _: &Update) {
        println!(
            "A NEW LIFE BURSTS FORTH FROM THE ASHES: GENERATION {}!",
            view.0
        );
        view.destroy(view.current_link());
        view.insert(Phoenix(view.0 + 1));
    }
}

fn main() {
    let mut world = World::new();
    world.insert(Phoenix(0));

    loop {
        world.trigger_simple(&Update);
    }
}
