use macroquad::prelude::*;

#[macroquad::main("WaveSim")]
async fn main() {
    let mut image = Image {
        bytes: vec![0; 4 * 800 * 800],
        width: 800,
        height: 800,
    };
    let texture = Texture2D::from_image(&image);

    let mut state = State::new(800, 800);

    loop {
        state.step();

        for y in 0..800 {
            for x in 0..800 {
                let int = (state.boards[state.active].get(x, y) * 128.0 + 128.0) as u8;
                image.set_pixel(x, y, Color::from_rgba(int, int, int, 255))
            }
        }

        if is_mouse_button_down(MouseButton::Left) {
            let (mouse_x, mouse_y) = mouse_position();
            let (mouse_x, mouse_y) = (mouse_x as u32, mouse_y as u32);
            *state.boards[state.active].get_mut(mouse_x, mouse_y) = 1.0;
        }

        texture.update(&image);
        draw_texture(&texture, 0.0, 0.0, Color::from_hex(0xFFFFFF));

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
                    let pos = Vec2::new(x as f32, y as f32);
                    let dist = (center - pos).length();

                    *board.get_mut(x, y) = 2.0 * (-dist).exp();

                    if x == width - 10 {
                        *board.get_mut(x, y) = 1.0;
                    }
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

    /// Discrete 2d wave equation
    ///
    /// next(x, y) = 2 * current(x, y) - previous(x, y) + C * (current(x-1, y) + current(x+1, y) + current(x, y-1) + current(x, y+1) - 4 * current(x, y))
    pub fn step(&mut self) {
        self.active = (self.active + 1) % 3;
        for y in 1..self.height - 1 {
            for x in 1..self.width - 1 {
                let last = &self.boards[(self.active + 2) % 3];
                let last2 = &self.boards[(self.active + 1) % 3];
                let mut sum = 0.0;
                sum += last.get(x, y) * 2.0 - last2.get(x, y);
                sum += 0.01
                    * (last.get(x - 1, y)
                        + last.get(x + 1, y)
                        + last.get(x, y - 1)
                        + last.get(x, y + 1)
                        - 4.0 * last.get(x, y));

                let next = &mut self.boards[self.active];
                *next.get_mut(x, y) = sum;
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

    pub fn randomize(mut self) -> Self {
        for i in 0..self.data.len() {
            self.data[i] = rand::gen_range(-1.0, 1.0);
        }
        self
    }

    pub fn get(&self, x: u32, y: u32) -> f32 {
        self.data[(y * self.width + x) as usize]
    }

    pub fn get_mut(&mut self, x: u32, y: u32) -> &mut f32 {
        &mut self.data[(y * self.width + x) as usize]
    }
}
