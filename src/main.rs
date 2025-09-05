mod camera;
mod ui;
mod world;

use crate::{ui::*, world::World};
use macroquad::prelude::*;
use rfd::FileDialog;

#[macroquad::main("Journeymap Viewer")]
async fn main() {
    let mut world: Option<World> = None;

    let mut exit_requested = false;

    loop {
        clear_background(BLACK);

        // Process keys, mouse etc.
        
        if let Some(world) = &mut world {
            world.update();
        }

        let requests = update_ui(&world);
        for request in requests.iter() {
            match request {
                Request::LoadWorld => {
                    let directory = FileDialog::new()
                        .set_directory("/")
                        .pick_folder();
                    
                    if let Some(directory) = directory {
                        world = Some(World::from_directory(directory.to_str().unwrap()).await);
                    }
                },
                Request::ReloadWorld => {
                    if let Some(world) = &mut world {
                        world.reload().await;
                    }
                }
                Request::UnloadWorld => world = None,
                Request::Exit => exit_requested = true,
            }
        }

        // Draw things before egui

        if let Some(world) = &world {
            set_camera(&world.camera.camera);
            world.draw();
        }

        set_default_camera();
        egui_macroquad::draw();

        // Draw things after egui

        next_frame().await;
        
        if exit_requested {
            break;
        }
    }
}
