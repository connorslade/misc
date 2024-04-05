use macroquad::{
    color::{self, Color},
    input::{is_mouse_button_down, mouse_position, MouseButton},
    math::{vec2, Vec2},
    rand,
    shapes::draw_circle,
    ui::{hash, root_ui, widgets},
    window::{self, next_frame},
};

struct State {
    particles: Vec<Particle>,
    attraction: f32,
    friction: f32,
    pointer_strength: f32,

    pan: Vec2,
}

#[derive(Debug, Copy, Clone)]
struct Particle {
    pos: Vec2,
    vel: Vec2,
    charge: f32,
}

#[macroquad::main("ParticleSim")]
async fn main() {
    let mut state = State {
        particles: Vec::new(),
        attraction: 1.0,
        friction: 0.01,
        pointer_strength: 5.0,

        pan: Vec2::new(1000.0, 700.0),
    };

    let (width, height) = (window::screen_width(), window::screen_height());
    for _ in 0..1000 {
        state.particles.push(Particle {
            pos: Vec2::new(
                (rand::rand() as f32 / u32::MAX as f32) * width,
                (rand::rand() as f32 / u32::MAX as f32) * height,
            ),
            vel: Vec2::ZERO,
            charge: (rand::rand() as f32 / u32::MAX as f32) * 2.0 - 1.0,
        });
    }

    loop {
        widgets::Window::new(hash!(), vec2(0.0, 0.0), vec2(300., 170.))
            .label("Config")
            .titlebar(true)
            .ui(&mut *root_ui(), |ui| {
                ui.slider(hash!(), "attracation", -2f32..2f32, &mut state.attraction);
                ui.slider(hash!(), "friction", 0.0..1.0, &mut state.friction);
                ui.slider(
                    hash!(),
                    "pointer strength",
                    0.0..10.0,
                    &mut state.pointer_strength,
                );
            });

        let mouse = Vec2::from(mouse_position()) - state.pan;

        for i in 0..state.particles.len() {
            let this = state.particles[i];
            let mut vec = Vec2::ZERO;

            for j in 0..state.particles.len() {
                let other = state.particles[j];

                let dir = other.pos - this.pos;
                let dist = dir.length().max(4.0);
                if dist == 0.0 {
                    continue;
                }

                let repel = this.charge.is_sign_negative() ^ other.charge.is_sign_negative();
                let attraction = (this.charge.abs() * other.charge.abs() * state.attraction) / dist;
                vec += dir.normalize_or_zero() * attraction * if repel { 1.0 } else { -1.0 };
            }

            if is_mouse_button_down(MouseButton::Left) {
                let dir = mouse - this.pos;
                let dist = dir.length().max(4.0);
                let attraction = this.charge.abs() * state.pointer_strength / dist;
                vec += dir.normalize_or_zero()
                    * attraction.abs()
                    * if is_mouse_button_down(MouseButton::Right) {
                        -1.0
                    } else {
                        1.0
                    };
            }

            state.particles[i].vel += vec;
        }

        for particle in &mut state.particles {
            particle.pos += particle.vel;
            particle.vel *= 1.0 - state.friction;
        }

        for particle in &state.particles {
            let pos = particle.pos + state.pan;

            let t = (particle.charge + 1.0) / 2.0;
            let color = lerp_color(color::RED, color::BLUE, t);

            draw_circle(pos.x, pos.y, 2.0, color);
        }

        next_frame().await
    }
}

fn lerp_color(a: Color, b: Color, t: f32) -> Color {
    Color {
        r: a.r + (b.r - a.r) * t,
        g: a.g + (b.g - a.g) * t,
        b: a.b + (b.b - a.b) * t,
        a: a.a + (b.a - a.a) * t,
    }
}
