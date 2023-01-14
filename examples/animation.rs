use blue_engine::{
    header::{Engine, WindowDescriptor},
    primitive_shapes::cube,
};

use blue_engine_utilities::{animation::Animation, AnimationKeyframe};

fn main() -> anyhow::Result<()> {
    let mut engine = Engine::new(WindowDescriptor {
        width: 1280,
        height: 720,
        title: "Animation test",
        ..Default::default()
    })?;
    engine
        .window
        .set_fullscreen(Some(blue_engine::winit::window::Fullscreen::Exclusive(
            engine
                .window
                .current_monitor()
                .unwrap()
                .video_modes()
                .next()
                .unwrap(),
        )));

    cube("cube", &mut engine.renderer, &mut engine.objects)?;

    let mut animation = Animation::new("cube");
    animation
        .keyframes
        .push((0.0, AnimationKeyframe::default()));
    animation.keyframes.push((
        5.0,
        AnimationKeyframe {
            position: (3f32, 0f32, 0f32).into(),
            rotation: (45f32, 45f32, 0f32).into(),
            size: (500f32, 100f32, 100f32).into(),
        },
    ));
    animation.keyframes.push((
        8.0,
        AnimationKeyframe {
            position: (0f32, 3f32, 0f32).into(),
            rotation: (-45f32, -45f32, 0f32).into(),
            size: (100f32, 50f32, 50f32).into(),
        },
    ));
    animation.keyframes.push((
        10.0,
        AnimationKeyframe {
            position: (0f32, 0f32, 0f32).into(),
            rotation: (0f32, 0f32, 0f32).into(),
            ..Default::default()
        },
    ));

    animation.start().expect("Couldn't compile the animation");

    engine.update_loop(move |_, window, objects, _, _, _| {
        animation.animate(objects, window.inner_size());
    })?;

    Ok(())
}
