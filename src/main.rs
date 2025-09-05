mod camera;
mod options;
mod ui;
mod world;

use crate::{options::Options, ui::*, world::World};
use macroquad::prelude::*;
use rfd::FileDialog;

#[macroquad::main("Journeymap Viewer")]
async fn main() {
    let mut ui = UI::default();
    let mut options = Options::default();
    let mut world: Option<World> = None;

    'main_loop: loop {
        clear_background(BLACK);

        let mut pointer_requested = false;
        let requests = ui.update(&mut options, &world);
        for request in requests.iter() {
            match request {
                Request::LoadWorld => {
                    let directory = FileDialog::new().set_directory("/").pick_folder();

                    if let Some(directory) = directory {
                        world = Some(World::from_directory(directory.to_str().unwrap()).await);
                    }
                }
                Request::ReloadWorld => {
                    if let Some(world) = &mut world {
                        world.reload().await;
                    }
                }
                Request::UnloadWorld => world = None,
                Request::Exit => break 'main_loop,
                Request::Pointer => pointer_requested = true,
            }
        }

        if let Some(world) = &mut world {
            world.update(&options, pointer_requested);

            set_camera(&world.camera.camera);
            world.draw();
        }

        set_default_camera();
        egui_macroquad::draw();

        next_frame().await;
    }
}
