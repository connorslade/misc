use std::time::Instant;

use macroquad::{
    color::{Color, BLACK, WHITE},
    input::{is_key_pressed, is_mouse_button_down, mouse_position, KeyCode, MouseButton},
    math::Vec2,
    text::draw_text,
    texture::{draw_texture, Image, Texture2D},
    time::get_frame_time,
    window::next_frame,
};

#[macroquad::main("WaveSim")]
async fn main() {
    let mut state = State::new(720, 406);
    let mut image = Image {
        bytes: vec![0; (4 * state.width * state.height) as usize],
        width: state.width as u16,
        height: state.height as u16,
    };
    let texture = Texture2D::from_image(&image);

    loop {
        let instant = Instant::now();
        for _ in 0..2 {
            state.step();
        }
        let update_time = instant.elapsed().as_millis();

        for y in 0..state.height {
            for x in 0..state.width {
                if x == 300
                    && !((y < 203 - 10 && y > 203 - 10 - 10) || (y > 203 + 10 && y < 203 + 10 + 10))
                {
                    image.set_pixel(x, y, BLACK);
                    continue;
                }

                let val = state.boards[state.active].get(x, y);

                let color = if val > 0.0 {
                    Color::new(0.0, 0.0, 1.0, 1.0)
                } else {
                    Color::new(1.0, 0.0, 0.0, 1.0)
                };

                let val = (val.abs() * 10.0).min(1.0);
                image.set_pixel(
                    x,
                    y,
                    Color::new(
                        (color.r * val) + (1.0 - val),
                        (color.g * val) + (1.0 - val),
                        (color.b * val) + (1.0 - val),
                        1.0,
                    ),
                );
            }
        }

        let mouse_pos = mouse_position();
        let (mouse_x, mouse_y) = (mouse_pos.0 as u32, mouse_pos.1 as u32);
        if mouse_x < state.width && mouse_y < state.height {
            if is_mouse_button_down(MouseButton::Left) {
                *state.boards[state.active].get_mut(mouse_x, mouse_y) = 0.1;
            } else if is_mouse_button_down(MouseButton::Right) {
                *state.boards[state.active].get_mut(mouse_x, mouse_y) = -0.1;
            }
        }

        if is_key_pressed(KeyCode::R) {
            state = State::new(state.width, state.height);
        }

        texture.update(&image);
        draw_texture(&texture, 0.0, 0.0, Color::from_hex(0xFFFFFF));

        let delta = get_frame_time();
        draw_text(&format!("FPS: {:.2}", 1.0 / delta), 10.0, 20.0, 20.0, WHITE);
        draw_text(&format!("MSPT: {}ms", update_time), 10.0, 50.0, 20.0, WHITE);

        next_frame().await
    }
}

struct State {
    boards: [Board; 3],
    active: usize,
    width: u32,
    height: u32,
}

impl State {
    pub fn new(width: u32, height: u32) -> Self {
        let mut boards = [
            Board::new(width, height), // next
            Board::new(width, height), // last2
            Board::new(width, height), // last
        ];

        let center = Vec2::new(width as f32 / 2.0, height as f32 / 2.0);
        for board in boards.iter_mut().skip(1) {
            for y in 0..height {
                for x in 0..width {
                    // let pos = Vec2::new(x as f32, y as f32);
                    // let dist = (center - pos).length();
                    let dist = (500.0 - x as f32).abs();

                    *board.get_mut(x, y) = 2.0 * (-dist).exp();
                }
            }
        }

        Self {
            boards,
            active: 2,
            width,
            height,
        }
    }

    // (active, last, last2)
    fn get_boards(&mut self) -> (&mut Board, &Board, &Board) {
        unsafe {
            let active = &mut *(&mut self.boards[self.active] as *mut Board);
            let last = &*(&self.boards[(self.active + 2) % 3] as *const Board);
            let last2 = &*(&self.boards[(self.active + 1) % 3] as *const Board);

            (active, last, last2)
        }
    }

    /// Discrete 2d wave equation
    ///
    /// next(x, y) = 2 * current(x, y) - previous(x, y) + C * (current(x-1, y) + current(x+1, y) + current(x, y-1) + current(x, y+1) - 4 * current(x, y))
    pub fn step(&mut self) {
        self.active = (self.active + 1) % 3;

        let (width, height) = (self.width, self.height);
        let (next, last, last2) = self.get_boards();

        for y in 1..height - 1 {
            for x in 1..width - 1 {
                let mut sum = 0.0;
                sum += last.get(x, y) * 2.0 - last2.get(x, y);
                sum += 0.01
                    * (last.get(x - 1, y)
                        + last.get(x + 1, y)
                        + last.get(x, y - 1)
                        + last.get(x, y + 1)
                        - 4.0 * last.get(x, y));

                *next.get_mut(x, y) = sum;

                if x == 300
                    && !((y < 203 - 10 && y > 203 - 10 - 10) || (y > 203 + 10 && y < 203 + 10 + 10))
                {
                    *next.get_mut(x, y) = 0.0;
                }
            }
        }
    }
}

struct Board {
    data: Vec<f32>,
    width: u32,
}

impl Board {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            data: vec![0.0; (width * height) as usize],
            width,
        }
    }

    pub fn get(&self, x: u32, y: u32) -> f32 {
        self.data[(y * self.width + x) as usize]
    }

    pub fn get_mut(&mut self, x: u32, y: u32) -> &mut f32 {
        &mut self.data[(y * self.width + x) as usize]
    }
}
