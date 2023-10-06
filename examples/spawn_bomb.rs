use boba_engine::prelude::*;

#[pearl(listen(Update))]
pub struct SpawnBomb {
    index: usize,
}

impl EventListener<Update> for SpawnBomb {
    fn update(_: &Update, arena: &mut ArenaView<Self>) {
        let index = arena.current_pearl().index;
        println!("I AM INFINITE - {index}");
        arena.insert(SpawnBomb { index: index + 1 });
    }
}

fn main() {
    let mut milk_tea = MilkTeaRunner::default();
    milk_tea.arena.insert(SpawnBomb { index: 0 });
    milk_tea.run();
}
