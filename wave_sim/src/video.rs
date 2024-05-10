use std::io::BufWriter;

use indicatif::ProgressBar;
use openh264::{
    self,
    encoder::Encoder,
    formats::{RgbSliceU8, YUVBuffer},
};

use crate::simulation::State;

const SIZE: (usize, usize) = (1920, 1080);

pub fn render_video() {
    let file = std::fs::File::create("output.h264").unwrap();
    let mut file = BufWriter::new(file);
    let mut encoder = Encoder::new().unwrap();

    let mut state = State::new(SIZE.0 as u32, SIZE.1 as u32);

    let progress = ProgressBar::new(5000).with_style(
        indicatif::ProgressStyle::default_bar()
            .template("{spinner} {wide_bar} {pos:>7}/{len:7} {per_sec} {eta} {msg}")
            .unwrap(),
    );
    for _ in 0..5000 {
        progress.inc(1);
        let mut image = ImageBuffer::new(SIZE.0, SIZE.1);
        for y in 0..state.height as usize {
            for x in 0..state.width as usize {
                if x == 300
                    && !((y < 203 - 20 && y > 203 - 20 - 10) || (y > 203 + 20 && y < 203 + 20 + 10))
                {
                    image.set_pixel(x, y, RgbColor::BLACK);
                    continue;
                }

                let val = state.boards[state.active].get(x as u32, y as u32);

                let color = if val > 0.0 {
                    RgbColor::BLUE
                } else {
                    RgbColor::RED
                };

                let val = (val.abs() * 10.0).min(1.0);
                image.set_pixel(
                    x,
                    y,
                    RgbColor::new(
                        (color.r as f32 * val) as u8 + (255.0 - val * 255.0) as u8,
                        (color.g as f32 * val) as u8 + (255.0 - val * 255.0) as u8,
                        (color.b as f32 * val) as u8 + (255.0 - val * 255.0) as u8,
                    ),
                );
            }
        }

        let image = RgbSliceU8::new(&image.data, SIZE);
        let yuv = YUVBuffer::from_rgb_source(image);
        let bitstream = encoder.encode(&yuv).unwrap();
        bitstream.write(&mut file).unwrap();

        state.step();
    }
}

struct ImageBuffer {
    data: Vec<u8>,
    width: usize,
}

struct RgbColor {
    r: u8,
    g: u8,
    b: u8,
}

impl ImageBuffer {
    fn new(width: usize, height: usize) -> Self {
        Self {
            data: vec![0; width * height * 3],
            width,
        }
    }

    fn set_pixel(&mut self, x: usize, y: usize, color: RgbColor) {
        let offset = (y * self.width + x) * 3;
        self.data[offset] = color.r;
        self.data[offset + 1] = color.g;
        self.data[offset + 2] = color.b;
    }
}

impl RgbColor {
    pub const WHITE: Self = Self::new(255, 255, 255);
    pub const BLACK: Self = Self::new(0, 0, 0);

    pub const BLUE: Self = Self::new(0, 0, 255);
    pub const RED: Self = Self::new(255, 0, 0);

    pub const fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }
}
