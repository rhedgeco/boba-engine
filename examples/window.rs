use boba_engine::prelude::*;
use taro_renderer::TaroCamera;

fn main() {
    env_logger::init();
    let mut milk_tea = MilkTea::new();

    milk_tea.world.insert(WindowBuilder::new(TaroRenderBuilder {
        ..Default::default()
    }));

    milk_tea.world.insert_global(TaroSkybox::Color {
        r: 0.57,
        g: 0.78,
        b: 0.89,
        a: 1.0,
    });
    let camera = milk_tea.world.insert(TaroCamera::new());

    milk_tea.world.insert(WindowBuilder::new(TaroRenderBuilder {
        render_cam: Some(camera),
    }));

    milk_tea.world.insert_callback::<MilkTeaUpdate>(|_, world| {
        if world.len::<MilkTeaWindow>() == 0 {
            let mut control = world.get_static_mut::<ControlFlow>().unwrap();
            control.set_exit(true);
        }
    });

    milk_tea.run();
}
