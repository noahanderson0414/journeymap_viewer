mod camera;
mod world;

use egui_macroquad::egui;
use macroquad::prelude::*;
use crate::world::World;

#[macroquad::main("Journeymap Viewer")]
async fn main() {
    let mut camera = camera::Camera::default();
    let world = World::load_from_directory("assets").await;

    loop {
        clear_background(BLACK);

        // Process keys, mouse etc.
        
        camera.update();

        egui_macroquad::ui(|egui_ctx| {
            egui::Window::new("Journeymap Viewer").show(egui_ctx, |ui| {
                ui.label("Test");
            });
        });

        // Draw things before egui

        set_camera(&camera.camera);
        world.draw();

        set_default_camera();
        egui_macroquad::draw();

        // Draw things after egui

        next_frame().await;
    }
}
