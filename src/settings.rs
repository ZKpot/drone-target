use super::{ Action, };

use dotrix::ecs::{ Mut, Const };
use dotrix::{ Window, };
use dotrix::math::{ Vec2i, Vec2u };
use dotrix::services::{ Input, };

pub struct Settings {
    pub paused: bool
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            paused: false
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
    };
}
