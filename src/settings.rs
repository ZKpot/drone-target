use dotrix::ecs::{ Mut };
use dotrix::{ Window };
use dotrix::math::{ Vec2i, Vec2u };

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
