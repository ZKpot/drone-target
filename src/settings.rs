use super::{ Action, Pause};

use dotrix::ecs::{ Mut, Const };
use dotrix::{ Window, State};
use dotrix::math::{ Vec2i, Vec2u };
use dotrix::services::{ Input, };
use dotrix::overlay::Overlay;
use dotrix::window::{ Fullscreen, };

use dotrix::egui::{
    self,
    Egui,
};

pub struct Settings {
    pub show_info_panel: bool,
    pub god_mode: bool,
    window_mode: WindowMode,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            show_info_panel: true,
            god_mode: false,
            window_mode: WindowMode::Windowed,
        }
    }
}

pub fn startup(
    mut window: Mut<Window>,
) {
    window.set_outer_position(
        Vec2i::new(
            (window.screen_size().x - window.outer_size().x) as i32 / 2,
            (window.screen_size().y - window.outer_size().y) as i32 / 2,
        )
    );

    window.set_inner_size(Vec2u::new(1280, 720));

    window.set_cursor_grab(true);
    window.set_cursor_visible(false);
}

pub fn ui_update (
    input: Const<Input>,
    mut state: Mut<State>,
) {
    if input.is_action_activated(Action::Menu) {
        state.push( Pause { handled: false} );
    }
}

pub fn pause_menu (
    overlay: Const<Overlay>,
    mut settings: Mut<Settings>,
    mut window: Mut<Window>,
    mut state: Mut<State>,
    input: Const<Input>,
) {
    window.set_cursor_grab(false);
    window.set_cursor_visible(true);

    let egui = overlay.get::<Egui>()
        .expect("Renderer does not contain an Overlay instance");

    let mut exit_pause = false;

    egui::containers::Window::new("Pause")
        .anchor(egui::Align2::CENTER_CENTER, egui::Vec2::new(0.0, 0.0))
        .collapsible(false)
        .resizable(false)
        .default_width(130.0)
        .show(&egui.ctx, |ui| {
            ui.vertical_centered_justified(|ui| {
                if ui.button("Resume").clicked() {
                    exit_pause = true;
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

                if settings.god_mode == true {
                    if ui.button("God mode: on").clicked() {
                        settings.god_mode = false;
                    }
                } else {
                    if ui.button("God mode: off").clicked() {
                        settings.god_mode = true;
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

    let mut paused = state.get_mut::<Pause>().expect("The system to be run in pause state");

    if paused.handled & input.is_action_activated(Action::Menu) {
        exit_pause = true;
    }

    paused.handled = true;

    if exit_pause {
        window.set_cursor_grab(true);
        window.set_cursor_visible(false);
        state.pop_any();
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub enum WindowMode {
    BorderlessFullscreen,
    Windowed,
}
