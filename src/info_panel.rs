use super::{ drone::Stats, settings, Pause };

use dotrix::ecs::{ Const, Entity };
use dotrix::services::{ World };
use dotrix::overlay::Overlay;
use dotrix::{ Frame, State, };

use dotrix::egui::{
    self,
    Egui,
};

pub fn update(
    world: Const<World>,
    overlay: Const<Overlay>,
    settings: Const<settings::Settings>,
    frame: Const<Frame>,
    state: Const<State>,
) {
    let info_ui_frame = egui::containers::Frame{
        fill: egui::Color32::from_black_alpha(192),
        corner_radius: 2.5,
        margin: egui::Vec2::new(4.0, 4.0),
        ..Default::default()
    };

    // default value
    let mut health        = 0;
    let mut charge        = 0;
    let mut strike_charge = 0;

    let egui = overlay.get::<Egui>()
        .expect("Renderer does not contain an Overlay instance");

    // Query all drones to display their stats
    let query = world.query::<( &Entity, &Stats )>();

    let paused = state.get::<Pause>().is_some();

    // draw info panel
    if settings.show_info_panel {
        egui::SidePanel::left("info_panel")
            .resizable(false)
            .frame(info_ui_frame)
            .show(&egui.ctx, |ui| {
                egui::Grid::new("info_grid").show(ui, |ui| {
                    ui.label("FPS");
                    ui.label(format!("{:05.1}", frame.fps()));
                    ui.end_row();
                });

            egui::ScrollArea::auto_sized()
                .enable_scrolling(paused)
                .show(ui, |ui|{
                    for (entity, stats) in query {
                        if stats.is_player {
                            health        = stats.health as u8;
                            charge        = stats.charge as u8;
                            strike_charge = stats.strike_charge as u8;
                        }

                        let label = if stats.is_player{
                            format!("{:?} - player", entity)
                        } else {
                            format!("{:?} - bot", entity)
                        };

                        // add label and value for each structure field
                        let status = format!("{:?}", stats);
                        let mut part: Vec<&str> = status.split(" { ").collect();
                        part = part[1].split(" }").collect();

                        egui::CollapsingHeader::new(label)
                            .default_open(stats.is_player)
                            .enabled(paused)
                            .show(ui, |ui| {

                                let glabel = format!("{:?} - grid", entity);
                                egui::Grid::new(glabel).show(ui, |ui| {
                                    for s in part[0].split(", ") {
                                        let s_part: Vec<&str> = s.split(": ").collect();
                                        ui.label(format!("{}:", s_part[0]));
                                        ui.label(s_part[1]);
                                        ui.end_row();
                                    }
                                });
                            });
                    }
            });
        });
    }

    // draw the status bar
    let margin = 8.0;

    egui::containers::Window::new("status_bar")
        .anchor(egui::Align2::RIGHT_BOTTOM, egui::Vec2::new(-margin, -margin))
        .collapsible(false)
        .title_bar(false)
        .resizable(false)
        .frame(info_ui_frame)
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
