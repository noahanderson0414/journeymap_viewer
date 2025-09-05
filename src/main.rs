mod camera;
mod world;

use crate::world::World;
use egui_macroquad::egui::{self, panel::TopBottomSide, Align2, TextStyle};
use macroquad::prelude::*;
use rfd::FileDialog;

#[macroquad::main("Journeymap Viewer")]
async fn main() {
    let mut world: Option<World> = None;

    let mut load_requested = false;
    let mut reload_requested = false;
    let mut unload_requested = false;
    let mut exit_requested = false;

    loop {
        clear_background(BLACK);

        // Process keys, mouse etc.
        
        if let Some(world) = &mut world {
            world.update();
        }

        egui_macroquad::ui(|egui_ctx| {
            egui::TopBottomPanel::new(TopBottomSide::Top, "top_bar").show(egui_ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.menu_button("File", |ui| {
                        if ui.button("Load World").clicked() {
                            load_requested = true;
                            ui.close_menu();
                        }

                        if ui.button("Exit").clicked() {
                            exit_requested = true;
                            ui.close_menu();
                        }
                    });
                    
                    if world.is_some() {
                        ui.menu_button("World", |ui| {
                            if ui.button("Reload").clicked() {
                                reload_requested = true;
                                ui.close_menu();
                            }

                            if ui.button("Unload").clicked() {
                                unload_requested = true;
                                ui.close_menu();
                            }
                        });
                    }
                })
            });
            
            if world.is_none() {
                egui::CentralPanel::default().show(egui_ctx, |ui| {
                    let response = ui.interact(ui.max_rect(), ui.id().with("panel_click"), egui::Sense::click());
                    if response.clicked() {
                        load_requested = true;
                    }

                    ui.painter().text(
                        ui.max_rect().center(),
                        Align2::CENTER_CENTER,
                        "NO WORLD LOADED",
                        ui.style().text_styles.get(&TextStyle::Heading).unwrap().clone(),
                        ui.style().visuals.text_color()
                    );
                });
            }
        });
        
        if load_requested {
            let directory = FileDialog::new()
                .set_directory("/")
                .pick_folder();
            
            if let Some(directory) = directory {
                world = Some(World::from_directory(directory.to_str().unwrap()).await);
            }

            load_requested = false;
        }

        if reload_requested && let Some(world) = &mut world {
            world.reload().await;
            reload_requested = false;
        }
        
        if unload_requested {
            world = None;
            unload_requested = false;
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
