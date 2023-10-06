use boba_engine::prelude::*;

#[pearl(listen(Update))]
pub struct SpawnBomb {
    index: usize,
}

impl EventListener<Update> for SpawnBomb {
    fn update(_: &mut Update, pearls: &mut PearlArenaView<Self>, _: &mut Resources) {
        let index = pearls.current().index;
        println!("I AM INFINITE - {index}");
        pearls.insert(SpawnBomb { index: index + 1 });
    }
}

fn main() {
    let mut milk_tea = MilkTeaRunner::default();
    milk_tea.pearls.insert(SpawnBomb { index: 0 });
    milk_tea.run();
}
