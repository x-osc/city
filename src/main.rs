#![allow(dead_code)]
#![allow(unused_variables)]

use fastrand::Rng;
use macroquad::prelude::*;

use crate::generator::roads::generate_roads;

mod generator;

#[macroquad::main("city")]
async fn main() {
    let mut rng = Rng::new();

    let roads = generate_roads(&mut rng);

    let mut cam = Camera2D {
        zoom: vec2(1. / 800., screen_width() / screen_height() * 1. / 800.),
        target: vec2(400.0, 300.0),
        ..Default::default()
    };

    loop {
        if is_mouse_button_down(MouseButton::Left)
            || is_mouse_button_down(MouseButton::Right)
            || is_mouse_button_down(MouseButton::Middle)
        {
            let pos = mouse_delta_position();

            cam.target.x += pos.x / cam.zoom.x;
            cam.target.y += pos.y / cam.zoom.y;
        }

        let wheel = mouse_wheel().1;
        if wheel != 0. {
            let before = cam.screen_to_world(Vec2::from(mouse_position()));

            cam.zoom *= 1.0 + wheel / 120. * 0.1;
            cam.zoom.x = cam.zoom.x.clamp(0.0002, 0.1);
            cam.zoom.y = screen_width() / screen_height() * cam.zoom.x;

            let after = cam.screen_to_world(Vec2::from(mouse_position()));

            cam.target += before - after;
        }

        set_camera(&cam);

        clear_background(WHITE);

        for r in &roads {
            draw_line(r.start.x, r.start.y, r.end.x, r.end.y, 8., BLACK);
        }

        next_frame().await
    }
}
