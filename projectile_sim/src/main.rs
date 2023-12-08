use macroquad::{
    color,
    math::{vec2, Vec2},
    shapes::{draw_circle, draw_line},
    ui::{hash, root_ui, widgets},
    window::{self, clear_background, next_frame, screen_height, screen_width},
};
use ordered_float::OrderedFloat;

const MIN: f32 = 0.001;

#[macroquad::main("ProjectileSim")]
async fn main() {
    let mut x_offset = 0.0;
    let mut y_offset = 0.0;
    let mut initial_velocity = 5.0;
    let mut initial_angle = 45.0;
    let mut acceleration = 9.8;
    let mut steps = 1.0;

    loop {
        let (width, height) = (window::screen_width(), window::screen_height());
        clear_background(color::BLACK);

        widgets::Window::new(hash!(), vec2(0.0, 0.0), vec2(300., 200.))
            .label("Config")
            .titlebar(true)
            .ui(&mut *root_ui(), |ui| {
                ui.slider(hash!(), "Vi", MIN..150f32, &mut initial_velocity);
                ui.slider(hash!(), "theta", -90f32..90f32, &mut initial_angle);
                ui.slider(hash!(), "accel", 0f32..20f32, &mut acceleration);
                ui.slider(hash!(), "steps", 1f32..10f32, &mut steps);
                ui.separator();
                ui.slider(hash!(), "x offset", 0f32..width, &mut x_offset);
                ui.slider(
                    hash!(),
                    "y offset",
                    height / -2.0..height / 2.0,
                    &mut y_offset,
                );
            });

        steps = steps.round();
        let mut points = simulate(steps as u32, initial_velocity, -initial_angle, acceleration);
        for point in points.iter_mut() {
            *point = *point + vec2(x_offset, y_offset);
        }

        let mut i = 0;
        for (prev, point) in points.iter().zip(points.iter().skip(1)) {
            i += 1;
            let (x, y) = (point.x, point.y);
            let (prev_x, prev_y) = (prev.x, prev.y);
            draw_line(
                prev_x,
                prev_y + height / 2.0,
                x,
                y + height / 2.0,
                2.0,
                color::WHITE,
            );
            if i % steps as u32 == 0 {
                draw_circle(x, y + height / 2.0, 3.0, color::WHITE);
            }
        }

        draw_line(
            0.0,
            y_offset + height / 2.0,
            screen_width(),
            y_offset + height / 2.0,
            1.0,
            color::GRAY,
        );
        draw_circle(x_offset, y_offset + height / 2.0, 5.0, color::BLUE);

        let highest = points
            .iter()
            .min_by_key(|point| OrderedFloat(point.y))
            .unwrap();
        draw_circle(highest.x, highest.y + height / 2.0, 5.0, color::RED);

        next_frame().await
    }
}

fn simulate(steps: u32, initial_velocity: f32, initial_angle: f32, acceleration: f32) -> Vec<Vec2> {
    let (vx, vy) = (
        initial_velocity * initial_angle.to_radians().cos(),
        initial_velocity * initial_angle.to_radians().sin(),
    );

    let (mut x, mut y) = (0.0, 0.0);
    let mut velocity = vy;

    let mut out = vec![vec2(x, y)];
    while x < screen_width() && y < screen_height() {
        let delta = (steps as f32).recip();

        velocity += acceleration * delta;
        y += velocity * delta;
        x += vx * delta;
        out.push(vec2(x, y));
    }

    out
}
