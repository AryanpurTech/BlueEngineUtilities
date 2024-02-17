use blue_engine::{
    header::{Engine, WindowDescriptor},
    primitive_shapes::uv_sphere,
};
use blue_engine_utilities::{model_load::load_gltf, LightManager};

fn main() -> color_eyre::Result<()> {
    let mut engine = Engine::new_config(WindowDescriptor {
        width: 1280,
        height: 720,
        title: "Light test",
        ..Default::default()
    })?;

    // make a light sphere
    uv_sphere(
        "light sphere",
        (18, 36, 1f32),
        &mut engine.renderer,
        &mut engine.objects,
    )?;
    engine
        .objects
        .get_mut("light sphere")
        .unwrap()
        .set_color(1f32, 0f32, 0f32, 1f32)
        .expect("color couldn't change");

    // load the monke
    load_gltf(
        Some("Suzanne"),
        std::path::Path::new("./resources/monkey.glb"),
        &mut engine.renderer,
        &mut engine.objects,
    )
    .expect("couldn't load the monke model");
    engine
        .objects
        .get_mut("Suzanne")
        .unwrap()
        .set_color(0.051f32, 0.533f32, 0.898f32, 1f32)
        .expect("color couldn't change");

    let mut light_manager = LightManager::new();
    light_manager.set_object_as_light("light sphere".to_string());

    let radius = 10f32;
    let start = std::time::SystemTime::now();

    engine.update_loop(move |renderer, _, objects, _, camera, _| {
        light_manager
            .update(objects, renderer, camera)
            .expect("couldn't update the light manager");

        let camx = start.elapsed().unwrap().as_secs_f32().sin() * radius;
        let camy = start.elapsed().unwrap().as_secs_f32().sin() * radius;
        let camz = start.elapsed().unwrap().as_secs_f32().cos() * radius;

        objects
            .get_mut("light sphere")
            .unwrap()
            .set_position(camx, camy, camz);
    })?;

    Ok(())
}
