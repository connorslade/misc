use macroquad::{
    color,
    input::{is_mouse_button_down, mouse_delta_position},
    math::{vec2, Vec2},
    miniquad::MouseButton,
    shapes::{draw_circle, draw_line},
    ui::{hash, root_ui, widgets},
    window::{self, clear_background, next_frame, screen_height, screen_width},
};
use ordered_float::OrderedFloat;

struct State {
    x_offset: f32,
    y_offset: f32,
    initial_velocity: f32,
    initial_angle: f32,
    acceleration: f32,
    steps: f32,
}

#[macroquad::main("ProjectileSim")]
async fn main() {
    let mut state = State {
        x_offset: 0.0,
        y_offset: 0.0,
        initial_velocity: 5.0,
        initial_angle: 45.0,
        acceleration: 9.8,
        steps: 20.0,
    };

    loop {
        let (width, height) = (window::screen_width(), window::screen_height());
        clear_background(color::BLACK);

        widgets::Window::new(hash!(), vec2(0.0, 0.0), vec2(300., 200.))
            .label("Config")
            .titlebar(true)
            .ui(&mut *root_ui(), |ui| {
                ui.slider(hash!(), "Vi", 0.001..150f32, &mut state.initial_velocity);
                ui.slider(hash!(), "theta", -90f32..90f32, &mut state.initial_angle);
                ui.slider(hash!(), "accel", 0f32..20f32, &mut state.acceleration);
                ui.slider(hash!(), "steps", 1f32..10f32, &mut state.steps);
                ui.separator();
                ui.slider(hash!(), "x offset", 0f32..width, &mut state.x_offset);
                ui.slider(
                    hash!(),
                    "y offset",
                    height / -2.0..height / 2.0,
                    &mut state.y_offset,
                );
            });

        let delta = mouse_delta_position();
        if is_mouse_button_down(MouseButton::Middle) {
            state.x_offset -= delta.x * width * 0.1;
            state.y_offset -= delta.y * height * 0.1;
        }

        let steps = state.steps.round();
        let mut points = simulate(
            steps as u32,
            state.initial_velocity,
            -state.initial_angle,
            state.acceleration,
        );
        for point in points.iter_mut() {
            *point = *point + vec2(state.x_offset, state.y_offset);
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
            state.y_offset + height / 2.0,
            screen_width(),
            state.y_offset + height / 2.0,
            1.0,
            color::GRAY,
        );
        draw_circle(
            state.x_offset,
            state.y_offset + height / 2.0,
            5.0,
            color::BLUE,
        );

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
