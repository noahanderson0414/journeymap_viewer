use crate::world::World;
use egui_macroquad::egui::{self, panel::TopBottomSide, Align2, TextStyle};

pub enum Request {
    LoadWorld,
    ReloadWorld,
    UnloadWorld,
    Exit,
}

pub fn update_ui(world: &Option<World>) -> Vec<Request> {
    let mut requests = vec![];

    egui_macroquad::ui(|egui_ctx| {
        egui::TopBottomPanel::new(TopBottomSide::Top, "top_bar").show(egui_ctx, |ui| {
            ui.horizontal(|ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Load World").clicked() {
                        ui.close_menu();
                        requests.push(Request::LoadWorld);
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
        
        if world.is_none() {
            egui::CentralPanel::default().show(egui_ctx, |ui| {
                let response = ui.interact(ui.max_rect(), ui.id().with("panel_click"), egui::Sense::click());
                if response.clicked() {
                    requests.push(Request::LoadWorld);
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

    requests
}