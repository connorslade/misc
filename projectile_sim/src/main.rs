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
    let mut initial_velocity = 5.0;
    let mut initial_angle = 45.0;
    let mut acceleration = 9.8;
    let mut steps = 100.0;

    loop {
        clear_background(color::BLACK);

        widgets::Window::new(hash!(), vec2(0.0, 0.0), vec2(300., 100.))
            .label("Config")
            .titlebar(true)
            .ui(&mut *root_ui(), |ui| {
                ui.slider(hash!(), "Vi", MIN..150f32, &mut initial_velocity);
                ui.slider(hash!(), "theta", -90f32..90f32, &mut initial_angle);
                ui.slider(hash!(), "accel", 0f32..10f32, &mut acceleration);
                ui.slider(hash!(), "steps", 0f32..1000f32, &mut steps);
            });

        steps = steps.round();
        let height = window::screen_height();

        let points = simulate(steps as u32, initial_velocity, -initial_angle, acceleration);

        for (prev, point) in points.iter().zip(points.iter().skip(1)) {
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
            draw_circle(x, y + height / 2.0, 3.0, color::WHITE);
        }

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

    let mut out = Vec::new();
    let mut step = 0;
    while x < screen_width() && y < screen_height() {
        step += 1;
        let delta = step as f32 / steps as f32;

        velocity += acceleration * delta;
        y += velocity * delta;
        x += vx * delta;
        out.push(vec2(x, y));
    }

    out
}
