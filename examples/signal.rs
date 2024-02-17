use boba_engine::prelude::*;

#[derive(Default)]
pub struct SignalEmitter {
    signal: Signal<String>,
}

impl Pearl for SignalEmitter {
    fn register(source: &mut impl EventSource<Self>) {
        source.listen::<MilkTea<Update>>();
    }
}

impl Listener<MilkTea<Update>> for SignalEmitter {
    fn trigger(mut pearl: PearlView<Self>, event: &mut Data<Update>) {
        let command = pearl.signal.command(format!("Hello, World!"));
        pearl.world_mut().send_signal(command);
        event.exit_app();
    }
}

pub struct SignalReciever;
impl Pearl for SignalReciever {}

fn main() {
    env_logger::init();
    let mut world = World::new();

    // create signal reciever
    let recv = world.insert(SignalReciever);

    // create signal sender
    let mut sender = SignalEmitter::default();
    sender.signal.add_listener(recv, |pearl, data| {
        let id = pearl.link().id();
        println!("SignalReciever({id}) got signal '{data}'.");
    });
    world.insert(sender);

    // run the world using milk tea
    milk_tea::run(&mut world);
}
