use blue_engine::{
    header::{Engine, WindowDescriptor},
    primitive_shapes::cube,
};

use blue_engine_utilities::{raycast::Raycast, FlyCamera};

fn main() -> anyhow::Result<()> {
    let mut engine = Engine::new_config(WindowDescriptor {
        width: 1500,
        height: 1000,
        title: "Raycast",
        ..Default::default()
    })?;

    cube("cube", &mut engine.renderer, &mut engine.objects)?;

    // camera
    let fly_camera = FlyCamera::new(&mut engine.camera);

    // Add fly camera to the engine as plugin
    engine.plugins.push(Box::new(fly_camera));

    let mut raycast = Raycast::new(&engine.camera);

    engine.update_loop(move |_, window, _, input, camera, _| {
        raycast.update(camera, input, &window.inner_size());

        println!("{:?}", raycast.get_current_ray());
    })?;

    Ok(())
}
