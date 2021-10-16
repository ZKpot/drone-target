use dotrix::ecs::{ Mut };
use dotrix::{ Window };

pub fn startup(
    mut window: Mut<Window>,
) {
    window.set_cursor_grab(true);
    window.set_cursor_visible(false);
}
