use super::{ Action, };

use dotrix::ecs::{ Mut, Const };
use dotrix::{ Window, };
use dotrix::math::{ Vec2i, Vec2u };
use dotrix::services::{ Input, };
use dotrix::overlay::Overlay;
use dotrix::window::{ Fullscreen, };

use dotrix::egui::{
    self,
    Egui,
};

pub struct Settings {
    pub paused: bool,
    pub show_info_panel: bool,
    window_mode: WindowMode,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            paused: false,
            show_info_panel: true,
            window_mode: WindowMode::Windowed,
        }
    }
}

pub fn startup(
    mut window: Mut<Window>,
) {
    window.set_cursor_grab(true);
    window.set_cursor_visible(false);

    window.set_outer_position(
        Vec2i::new(
            (window.screen_size().x - window.outer_size().x) as i32 / 2,
            (window.screen_size().y - window.outer_size().y) as i32 / 2,
        )
    );

    window.set_inner_size(Vec2u::new(1280, 720));
}

pub fn update (
    mut settings: Mut<Settings>,
    input: Const<Input>,
) {
    if input.is_action_activated(Action::Menu) {
        settings.paused = !settings.paused;
    }
}

pub fn menu (
    overlay: Const<Overlay>,
    mut settings: Mut<Settings>,
    mut window: Mut<Window>,
) {
    window.set_cursor_grab(!settings.paused);
    window.set_cursor_visible(settings.paused);

    let egui = overlay.get::<Egui>()
        .expect("Renderer does not contain an Overlay instance");

    if settings.paused {
        egui::containers::Window::new("Pause")
            .anchor(egui::Align2::CENTER_CENTER, egui::Vec2::new(0.0, 0.0))
            .collapsible(false)
            .resizable(false)
            .default_width(130.0)
            .show(&egui.ctx, |ui| {
                ui.vertical_centered_justified(|ui| {
                    if ui.button("Resume").clicked() {
                        settings.paused = false;
                    }

                    if settings.show_info_panel == true {
                        if ui.button("Hide info panel").clicked() {
                            settings.show_info_panel = false;
                        }
                    } else {
                        if ui.button("Show info panel").clicked() {
                            settings.show_info_panel = true;
                        }
                    }

                    if settings.window_mode == WindowMode::BorderlessFullscreen {
                        if ui.button("Windowed").clicked() {
                            window.set_fullscreen(None);
                            settings.window_mode = WindowMode::Windowed;
                        }
                    } else {
                        if ui.button("Fullscreen").clicked() {
                            window.set_fullscreen(Some(Fullscreen::Borderless(0)));
                            settings.window_mode = WindowMode::BorderlessFullscreen;
                        }
                    }

                    if ui.button("Exit").clicked() {
                        window.close();
                    }
                }
            )
        });
    };
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub enum WindowMode {
    BorderlessFullscreen,
    Windowed,
}
