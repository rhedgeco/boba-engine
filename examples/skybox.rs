use boba_engine::prelude::*;

fn main() {
    env_logger::init();
    let mut milk_tea = MilkTea::new();

    // insert a global skybox as light blue
    milk_tea
        .world
        .insert_global(TaroSkybox::Color(Color::LIGHT_BLUE));

    // create 3 cameras
    // first with the default "global" skybox
    let camera1 = milk_tea.world.insert(TaroCamera::default());
    // next with an overidden local skybox set to salmon
    let camera2 = milk_tea.world.insert(TaroCamera {
        skybox: CameraSkybox::Local(TaroSkybox::Color(Color::LIGHT_SALMON)),
    });
    // last with an overidden local skybox set to green
    let camera3 = milk_tea.world.insert(TaroCamera {
        skybox: CameraSkybox::Local(TaroSkybox::Color(Color::LIGHT_GREEN)),
    });

    // create and insert 3 window builders connected to each camera
    milk_tea.world.insert(WindowBuilder::new(TaroRenderBuilder {
        render_cam: Some(camera1),
    }));
    milk_tea.world.insert(WindowBuilder::new(TaroRenderBuilder {
        render_cam: Some(camera2),
    }));
    milk_tea.world.insert(WindowBuilder::new(TaroRenderBuilder {
        render_cam: Some(camera3),
    }));

    // make sure that program shuts down when no windows are open
    milk_tea.world.insert_callback::<MilkTeaUpdate>(|_, world| {
        if world.len::<MilkTeaWindow>() == 0 {
            let mut control = world.get_static_mut::<ControlFlow>().unwrap();
            control.set_exit(true);
        }
    });

    milk_tea.run();
}
