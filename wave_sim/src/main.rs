use std::f32::consts::PI;

use macroquad::{experimental::camera::mouse, prelude::*};

#[macroquad::main("WaveSim")]
async fn main() {
    let mut board = Board::new(800, 800);
    let mut image = Image {
        bytes: vec![0; 4 * 800 * 800],
        width: 800,
        height: 800,
    };
    let texture = Texture2D::from_image(&image);

    let mut sources = [
        Source {
            pos: Vec2::new(200.0, 400.0),
            val: 1.0,
        },
        Source {
            pos: Vec2::new(600.0, 400.0),
            val: 1.0,
        },
    ];

    loop {
        let (mouse_x, mouse_y) = mouse_position();
        sources[0].pos = Vec2::new(mouse_x, mouse_y);

        for source in &mut sources {
            source.val += 0.1;
            if source.val >= 1.0 {
                source.val = -1.0;
            }
        }

        for y in 0..board.height {
            for x in 0..board.width {
                let pos = Vec2::new(x as f32, y as f32);
                let mut val = 0.0;

                for source in &sources {
                    val += (source.pos.distance(pos) / 4.0 - 2.0 * PI * source.val).sin();
                }

                let int: u8 = (val * 255.0) as u8;
                image.set_pixel(x, y, Color::from_rgba(int, int, int, 255))
            }
        }

        texture.update(&image);
        draw_texture(&texture, 0.0, 0.0, Color::from_hex(0xFFFFFF));

        next_frame().await
    }
}

struct Source {
    pos: Vec2,
    val: f32,
}

struct Board {
    data: Vec<f32>,
    width: u32,
    height: u32,
}

impl Board {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            data: vec![0.0; (width * height) as usize],
            width,
            height,
        }
    }

    pub fn get(&self, x: u32, y: u32) -> f32 {
        self.data[(y * self.width + x) as usize]
    }

    pub fn get_mut(&mut self, x: u32, y: u32) -> &mut f32 {
        &mut self.data[(y * self.width + x) as usize]
    }
}
