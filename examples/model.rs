use blue_engine::header::{Engine, WindowDescriptor};
use blue_engine_utilities::{model_load::load_gltf, FlyCamera};

fn main() -> eyre::Result<()> {
    let mut engine = Engine::new_config(WindowDescriptor {
        width: 1280,
        height: 720,
        title: "Model test",
        ..Default::default()
    })?;

    // load the monke
    load_gltf(
        Some("ferris"),
        std::path::Path::new("./resources/ferris3d.glb"),
        &mut engine.renderer,
        &mut engine.objects,
    )
    .expect("couldn't load the monke model");

    let radius = 10f32;
    let start = std::time::SystemTime::now();

    let fly_camera = FlyCamera::new(&mut engine.camera);
    engine.signals.add_signal("fly", Box::new(fly_camera));

    engine.update_loop(move |_renderer, _, _objects, _, _camera, _| {
        let _camx = start.elapsed().unwrap().as_secs_f32().sin() * radius;
        let _camy = start.elapsed().unwrap().as_secs_f32().sin() * radius;
        let _camz = start.elapsed().unwrap().as_secs_f32().cos() * radius;
    })?;
    Ok(())
}
