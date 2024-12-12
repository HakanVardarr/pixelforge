use minifb::{Key, KeyRepeat, WindowOptions};
use pixelforge::buffer::Buffer;
use pixelforge::color::rgb_from_u8;

const WIDTH: usize = 800;
const HEIGHT: usize = 600;

fn main() {
    let mut buffer = Buffer::new(WIDTH, HEIGHT);
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let r = ((y as f32 / HEIGHT as f32) * 255.0) as u8;
            let g = ((x as f32 / WIDTH as f32) * 255.0) as u8;
            let b = 0u8;

            buffer.set_pixel(x, y, rgb_from_u8(r, g, b)).unwrap();
        }
    }

    let mut window =
        minifb::Window::new("Software Renderer", WIDTH, HEIGHT, WindowOptions::default())
            .expect("Failed to create window!");

    'main_loop: while window.is_open() {
        for key in window.get_keys_pressed(KeyRepeat::No) {
            match key {
                Key::Q => break 'main_loop,
                _ => {}
            }
        }

        window
            .update_with_buffer(buffer.as_bytes(), WIDTH, HEIGHT)
            .unwrap();
    }
}
