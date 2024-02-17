use blue_engine::{
    header::{Engine, WindowDescriptor},
    imports::glm,
    primitive_shapes::cube,
};

use blue_engine_utilities::{raycast::Raycast, FlyCamera};

fn main() -> color_eyre::Result<()> {
    let mut engine = Engine::new_config(WindowDescriptor {
        width: 1000,
        height: 1000,
        title: "Raycast",
        ..Default::default()
    })?;

    cube("cube1", &mut engine.renderer, &mut engine.objects)?;
    cube("cube2", &mut engine.renderer, &mut engine.objects)?;
    cube("cube3", &mut engine.renderer, &mut engine.objects)?;

    engine
        .objects
        .get_mut("cube1")
        .unwrap()
        .set_position(10f32, 1f32, -10f32);
    engine
        .objects
        .get_mut("cube2")
        .unwrap()
        .set_position(-5f32, -5f32, -5f32);
    engine
        .objects
        .get_mut("cube3")
        .unwrap()
        .set_position(0f32, 5f32, -7f32);

    // camera
    let fly_camera = FlyCamera::new(&mut engine.camera);

    // Add fly camera to the engine as plugin
    //engine.plugins.push(Box::new(fly_camera));

    let mut raycast = Raycast::new(&engine.camera);

    engine.update_loop(move |renderer, window, objects, input, camera, _| {
        raycast.update(camera, input, &window.inner_size());

        let obj = objects.get_mut("cube1").unwrap();

        //if input.mouse_pressed(0) {
        let raycast_pos = raycast.get_current_ray();
        //cube("cube5", renderer, objects);
        //obj.position(raycast_pos.x, raycast_pos.y, raycast_pos.z);
        //}
        //println!("{:?}", raycast_pos);

        let transformation_matrix = obj.position_matrix * obj.rotation_matrix * obj.scale_matrix;

        raycast.ray_intersects_bounding_box(
            (
                (transformation_matrix
                    * glm::vec4(obj.position.x, obj.position.y, obj.position.z, 1f32))
                .xyz(),
                (transformation_matrix * glm::vec4(obj.scale.x, obj.scale.y, obj.scale.z, 1f32))
                    .xyz(),
            ),
            1f32,
            camera,
        );
    })?;

    Ok(())
}
