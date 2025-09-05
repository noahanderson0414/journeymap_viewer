use std::path::Path;

use crate::{options::Options, world::World};
use egui_macroquad::egui::{self, panel::TopBottomSide, Align2, FontSelection, TextStyle};
use macroquad::prelude::*;
use rfd::FileDialog;

pub enum Request {
    LoadWorld,
    ReloadWorld,
    UnloadWorld,
    Exit,
    Pointer,
}

#[derive(Default)]
pub struct UI {
    pub show_options: bool,
}

impl UI {
    pub fn update(&mut self, options: &mut Options, world: &Option<World>) -> Vec<Request> {
        let mut requests = vec![];

        egui_macroquad::ui(|egui_ctx| {
            egui::TopBottomPanel::new(TopBottomSide::Top, "top_bar").show(egui_ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.menu_button("File", |ui| {
                        if ui.button("Load World").clicked() {
                            ui.close_menu();
                            requests.push(Request::LoadWorld);
                        }

                        if ui.button("Options").clicked() {
                            self.show_options = true;
                        }

                        if ui.button("Exit").clicked() {
                            ui.close_menu();
                            requests.push(Request::Exit);
                        }
                    });

                    if world.is_some() {
                        ui.menu_button("World", |ui| {
                            if ui.button("Reload").clicked() {
                                ui.close_menu();
                                requests.push(Request::ReloadWorld);
                            }

                            if ui.button("Unload").clicked() {
                                ui.close_menu();
                                requests.push(Request::UnloadWorld);
                            }
                        });
                    }
                })
            });

            if options.game_path.is_empty() {
                self.show_options = true;
            }

            egui::Window::new("Options")
                .open(&mut self.show_options)
                .default_pos(egui_ctx.available_rect().center())
                .resizable(false)
                .collapsible(false)
                .show(egui_ctx, |ui| {
                    ui.horizontal(|ui| {
                        ui.label("Game Path");
                        
                        if Path::new(options.game_path.as_str()).try_exists().is_ok_and(|x| !x) {
                            ui.separator();
                            ui.add(egui::Label::new("Invalid path!"));
                        }
                    });
                    
                    ui.horizontal(|ui| {
                        ui.add(egui::TextEdit::singleline(&mut options.game_path));
                        
                        if ui.button("...").clicked() {
                            let directory = FileDialog::new().set_directory("/").pick_folder();

                            if let Some(directory) = directory {
                                options.game_path = String::from(directory.to_str().unwrap());
                            }
                        }
                    });
                    
                    ui.separator();

                    ui.label("Minimum Zoom");
                    ui.add(egui::Slider::new(
                        &mut options.minimum_zoom,
                        0.0001..=options.maximum_zoom,
                    ));

                    ui.label("Maximum Zoom");
                    ui.add(egui::Slider::new(
                        &mut options.maximum_zoom,
                        options.minimum_zoom..=1.,
                    ));
                    
                    ui.separator();

                    if ui.button("Reset to Default").clicked() {
                        *options = Options::default();
                    }
                });

            if world.is_none() {
                egui::CentralPanel::default().show(egui_ctx, |ui| {
                    let response = ui.interact(
                        ui.max_rect(),
                        ui.id().with("panel_click"),
                        egui::Sense::click(),
                    );
                    if response.clicked() {
                        requests.push(Request::LoadWorld);
                    }

                    ui.painter().text(
                        ui.max_rect().center(),
                        Align2::CENTER_CENTER,
                        "NO WORLD LOADED",
                        ui.style()
                            .text_styles
                            .get(&TextStyle::Heading)
                            .unwrap()
                            .clone(),
                        ui.style().visuals.text_color(),
                    );
                });
            }

            if egui_ctx.wants_pointer_input() {
                requests.push(Request::Pointer);
            }
        });

        requests
    }
}
