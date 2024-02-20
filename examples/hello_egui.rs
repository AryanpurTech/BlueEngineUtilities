/*
 * Blue Engine by Elham Aryanpur
 *
 * Basic GUI example
 *
 * Licensed under Apache-2.0
*/

// Gui is a trait that you'll be using to add your UI
use blue_engine_utilities::egui;
use blue_engine_utilities::egui::egui as gui;

// Basic imports
use blue_engine::{
    header::{Engine, ObjectSettings},
    primitive_shapes::triangle,
};

fn main() {
    // Initialize the engine with default settings
    let mut engine = Engine::new().expect("win");

    // Add a triangle to the screen
    triangle(
        "triangle",
        ObjectSettings::default(),
        &mut engine.renderer,
        &mut engine.objects,
    )
    .unwrap();

    // Start the egui context
    let gui_context = egui::EGUI::new(&engine.event_loop, &mut engine.renderer, &engine.window);

    // We add the gui as plugin, which runs once before everything else to fetch events, and once during render times for rendering and other stuff
    engine.signals.add_signal("egui", Box::new(gui_context));

    let mut color = [1f32, 1f32, 1f32, 1f32];

    // Update loop
    engine
        .update_loop(move |_, window, objects, _, _, signals| {
            // obtain the plugin
            let egui_plugin = signals
                .get_signal::<egui::EGUI>("egui")
                .expect("Plugin not found")
                .expect("Plugin type mismatch");

            // ui function will provide the context
            egui_plugin.ui(
                |ctx| {
                    gui::Window::new("title").show(ctx, |ui| {
                        ui.horizontal(|ui| {
                            ui.label("Pick a color");
                            ui.color_edit_button_rgba_unmultiplied(&mut color);
                        });
                    });

                    objects
                        .get_mut("triangle")
                        .unwrap()
                        .set_uniform_color(color[0], color[1], color[2], color[3])
                        .unwrap();
                },
                &window,
            );
        })
        .expect("Error during update loop");
}
