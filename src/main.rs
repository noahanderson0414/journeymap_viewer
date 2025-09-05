mod camera;
mod world;

use egui_macroquad::egui;
use macroquad::prelude::*;
use crate::world::World;

#[macroquad::main("Journeymap Viewer")]
async fn main() {
    let mut camera = camera::Camera::default();
    let world = World::load_from_directory(r"C:\Users\Frosty\AppData\Roaming\PrismLauncher\instances\1.21.8 Fabric\minecraft\journeymap\data\sp\world\overworld\day").await;

    loop {
        clear_background(BLACK);

        // Process keys, mouse etc.
        
        let (scroll_x, scroll_y) = mouse_wheel();
        println!("{} {}", scroll_x, scroll_y);

        egui_macroquad::ui(|egui_ctx| {
            egui::Window::new("Journeymap Viewer").show(egui_ctx, |ui| {
                ui.label("Test");
            });
        });

        // Draw things before egui

        set_camera(&Camera2D {
            zoom: vec2(1., screen_width() / screen_height()) / screen_width(),
            ..Default::default()
        });
        world.draw();

        set_default_camera();
        egui_macroquad::draw();

        // Draw things after egui

        next_frame().await;
    }
}
