use std::time::Instant;

use macroquad::{
    color::{Color, BLACK},
    input::{is_key_pressed, is_mouse_button_down, mouse_position, KeyCode, MouseButton},
    text::draw_text,
    texture::{draw_texture, Image, Texture2D},
    time::get_frame_time,
    window::{next_frame, Conf},
};
use simulation::State;

mod simulation;
mod video;

fn conf() -> Conf {
    Conf {
        window_title: "wave_sim".into(),
        window_resizable: false,
        window_height: 406,
        window_width: 720,
        ..Default::default()
    }
}

fn main() {
    video::render_video();
}

// #[macroquad::main(conf)]
// async fn main() {

//     let mut state = State::new(720, 406);
//     let mut image = Image {
//         bytes: vec![0; (4 * state.width * state.height) as usize],
//         width: state.width as u16,
//         height: state.height as u16,
//     };
//     let texture = Texture2D::from_image(&image);

//     macroquad::miniquad::window::set_window_size(state.width as u32, state.height as u32);

//     loop {
//         #[cfg(not(target_arch = "wasm32"))]
//         let instant = Instant::now();
//         for _ in 0..2 {
//             state.step();
//         }
//         #[cfg(not(target_arch = "wasm32"))]
//         let update_time = instant.elapsed().as_millis();

//         for y in 0..state.height {
//             for x in 0..state.width {
//                 if x == 300
//                     && !((y < 203 - 20 && y > 203 - 20 - 10) || (y > 203 + 20 && y < 203 + 20 + 10))
//                 {
//                     image.set_pixel(x, y, BLACK);
//                     continue;
//                 }

//                 let val = state.boards[state.active].get(x, y);

//                 let color = if val > 0.0 {
//                     Color::new(0.0, 0.0, 1.0, 1.0)
//                 } else {
//                     Color::new(1.0, 0.0, 0.0, 1.0)
//                 };

//                 let val = (val.abs() * 10.0).min(1.0);
//                 image.set_pixel(
//                     x,
//                     y,
//                     Color::new(
//                         (color.r * val) + (1.0 - val),
//                         (color.g * val) + (1.0 - val),
//                         (color.b * val) + (1.0 - val),
//                         1.0,
//                     ),
//                 );
//             }
//         }

//         let mouse_pos = mouse_position();
//         let (mouse_x, mouse_y) = (mouse_pos.0 as u32, mouse_pos.1 as u32);
//         if mouse_x < state.width && mouse_y < state.height {
//             if is_mouse_button_down(MouseButton::Left) {
//                 *state.boards[state.active].get_mut(mouse_x, mouse_y) = 0.1;
//             } else if is_mouse_button_down(MouseButton::Right) {
//                 *state.boards[state.active].get_mut(mouse_x, mouse_y) = -0.1;
//             }
//         }

//         if is_key_pressed(KeyCode::R) {
//             state = State::new(state.width, state.height);
//         }

//         texture.update(&image);
//         draw_texture(&texture, 0.0, 0.0, Color::from_hex(0xFFFFFF));

//         let delta = get_frame_time();
//         draw_text(&format!("FPS: {:.2}", 1.0 / delta), 10.0, 20.0, 20.0, BLACK);
//         #[cfg(not(target_arch = "wasm32"))]
//         draw_text(&format!("MSPT: {}ms", update_time), 10.0, 35.0, 20.0, BLACK);

//         next_frame().await
//     }
// }
