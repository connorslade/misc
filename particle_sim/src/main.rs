use macroquad::{
    color::{self, colors::WHITE},
    input::mouse_wheel,
    math::{vec2, Vec2},
    rand,
    shapes::{draw_circle, draw_line},
    text::{draw_text, measure_text},
    ui::{hash, root_ui, widgets},
    window::{self, clear_background, next_frame, screen_height, screen_width},
};

struct State {
    particles: Vec<Particle>,
    attraction: f32,
}

#[derive(Debug, Copy, Clone)]
struct Particle {
    pos: Vec2,
    charge: f32,
}

#[macroquad::main("ParticleSim")]
async fn main() {
    let mut state = State {
        particles: Vec::new(),
        attraction: 0.1,
    };

    let (width, height) = (window::screen_width(), window::screen_height());
    for i in 0..2000 {
        state.particles.push(Particle {
            pos: Vec2::new(
                (rand::rand() as f32 / u32::MAX as f32) * width,
                (rand::rand() as f32 / u32::MAX as f32) * height,
            ),
            charge: (rand::rand() as f32 / u32::MAX as f32) * 2.0 - 1.0,
        });
    }

    loop {
        let (width, height) = (window::screen_width(), window::screen_height());
        clear_background(color::BLACK);

        widgets::Window::new(hash!(), vec2(0.0, 0.0), vec2(300., 170.))
            .label("Config")
            .titlebar(true)
            .ui(&mut *root_ui(), |ui| {
                ui.slider(
                    hash!(),
                    "attracation",
                    -700f32..700f32,
                    &mut state.attraction,
                );
            });

        for i in 0..state.particles.len() {
            let this = state.particles[i];
            let mut vec = Vec2::ZERO;

            for j in 0..state.particles.len() {
                let other = state.particles[j];

                let dir = other.pos - this.pos;
                let dist = dir.length_squared();
                if dist == 0.0  {
                    continue;
                }

                let repel = this.charge.is_sign_negative() ^ other.charge.is_sign_negative();
                let attraction = (this.charge.abs() * other.charge.abs() * state.attraction) / dist;
                vec += dir.normalize_or_zero()
                    * attraction.min(dist.sqrt())
                    * if repel { 1.0 } else { -1.0 };
            }

            state.particles[i].pos += vec;
        }

        for particle in &state.particles {
            draw_circle(particle.pos.x, particle.pos.y, 2.0, WHITE);
        }

        next_frame().await
    }
}
