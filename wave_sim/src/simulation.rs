use std::time::Instant;

use macroquad::math::Vec2;

pub struct State {
    pub boards: [Board; 3],
    pub active: usize,
    pub tick: usize,

    pub width: u32,
    pub height: u32,
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
            tick: 0,
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
        self.tick += 1;
        self.active = (self.active + 1) % 3;

        let (width, height) = (self.width, self.height);
        let tick = self.tick;
        let (next, last, last2) = self.get_boards();

        // let center = Vec2::new(width as f32 / 2.0, height as f32 / 2.0);

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
                // let dist = (center - Vec2::new(x as f32, y as f32)).length() / 10.0;
                // let dist = (500.0 - x as f32).abs();
                // sum += 0.002 * (-dist).exp() * (tick as f32 / 30.0).cos();

                *next.get_mut(x, y) = sum;

                if x == 300
                    && !((y < 203 - 20 && y > 203 - 20 - 10) || (y > 203 + 20 && y < 203 + 20 + 10))
                {
                    *next.get_mut(x, y) = 0.0;
                }
            }
        }
    }
}

pub struct Board {
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
