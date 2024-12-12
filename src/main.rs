use minifb::{Key, KeyRepeat, Window, WindowOptions};

const WIDTH: usize = 800;
const HEIGHT: usize = 600;

fn main() {
    let mut buffer = [rgb(0, 0, 0); WIDTH * HEIGHT];
    for row in 0..HEIGHT {
        for col in 0..WIDTH {
            let r = ((row as f32 / HEIGHT as f32) * 255.0) as u8;
            let g = ((col as f32 / WIDTH as f32) * 255.0) as u8;
            let b = 0u8;

            buffer[row * WIDTH + col] = rgb(r, g, b);
        }
    }

    let mut window = Window::new("Software Renderer", WIDTH, HEIGHT, WindowOptions::default())
        .expect("Failed to create window!");

    'main_loop: while window.is_open() {
        for key in window.get_keys_pressed(KeyRepeat::No) {
            match key {
                Key::Q => break 'main_loop,
                _ => {}
            }
        }

        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}

fn rgb(r: u8, g: u8, b: u8) -> u32 {
    let (r, g, b) = (r as u32, g as u32, b as u32);
    (r << 16) | (g << 8) | b
}
