use super::{ drone::Stats, };

use dotrix::ecs::{ Const, Entity };
use dotrix::services::{ World };
use dotrix::overlay::Overlay;

use dotrix::egui::{
    self,
    Egui,
};

pub fn update(
    world: Const<World>,
    overlay: Const<Overlay>,
) {
    // default value
    let mut health        = 0;
    let mut charge        = 0;
    let mut strike_charge = 0;

    let egui = overlay.get::<Egui>()
        .expect("Renderer does not contain an Overlay instance");

    // Query all drones to display their stats
    let query = world.query::<( &Entity, &Stats )>();

    for (entity, stats) in query {
        if stats.is_player {
            health        = stats.health as u8;
            charge        = stats.charge as u8;
            strike_charge = stats.strike_charge as u8;
        }

    }

    // draw the status bar
    let margin = 8.0;

    egui::containers::Window::new("status_bar")
        .anchor(egui::Align2::RIGHT_BOTTOM, egui::Vec2::new(-margin, -margin))
        .collapsible(false)
        .title_bar(false)
        .resizable(false)
        .frame(egui::containers::Frame{
            fill: egui::Color32::from_black_alpha(192),
            corner_radius: 2.5,
            margin: egui::Vec2::new(4.0, 4.0),
            ..Default::default()
        })
        .show(&egui.ctx, |ui| {
            egui::Grid::new("status_bar_grid")
                .max_col_width(75.0)
                .show(ui, |ui| {
                ui.vertical_centered_justified(|ui| {
                    ui.add(
                        egui::Label::new("HEALTH")
                            .text_color(egui::Color32::LIGHT_GRAY)
                            .strong()
                    );
                    ui.add(egui::Label::new(format!("{number:0>width$}", number = health, width = 3))
                        .text_color(egui::Color32::LIGHT_GRAY)
                        .heading()
                    );
                });

                ui.vertical_centered_justified(|ui| {
                    ui.add(
                        egui::Label::new("CHARGE")
                            .text_color(egui::Color32::LIGHT_GRAY)
                            .strong()
                    );
                    ui.add(egui::Label::new(format!("{number:0>width$}", number = charge, width = 3))
                        .text_color(egui::Color32::LIGHT_GRAY)
                        .heading()
                    );
                });

                ui.vertical_centered_justified(|ui| {
                    ui.add(
                        egui::Label::new("STRIKE")
                            .text_color(egui::Color32::LIGHT_GRAY)
                            .strong()
                    );
                    ui.add(egui::Label::new(format!("{number:0>width$}", number = strike_charge, width = 3))
                        .text_color(egui::Color32::LIGHT_GRAY)
                        .heading()
                    );
                });
            });
        });

}
