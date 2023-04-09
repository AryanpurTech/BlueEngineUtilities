use blue_engine::header::{Engine, WindowDescriptor};
use blue_engine_utilities::{model_load::load_gltf, FlyCamera};

fn main() -> anyhow::Result<()> {
    let mut engine = Engine::new_config(WindowDescriptor {
        width: 1280,
        height: 720,
        title: "Model test",
        ..Default::default()
    })?;

    // load the monke
    load_gltf(
        "./resources/ferris3d.glb",
        &mut engine.renderer,
        &mut engine.objects,
    )
    .expect("couldn't load the monke model");

    let radius = 10f32;
    let start = std::time::SystemTime::now();

    let fly_camera = FlyCamera::new(&mut engine.camera);
    engine.plugins.push(Box::new(fly_camera));

    engine.update_loop(move |renderer, _, objects, _, camera, _| {
        let camx = start.elapsed().unwrap().as_secs_f32().sin() * radius;
        let camy = start.elapsed().unwrap().as_secs_f32().sin() * radius;
        let camz = start.elapsed().unwrap().as_secs_f32().cos() * radius;
    })?;
    Ok(())
}