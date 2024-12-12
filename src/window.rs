use super::buffer::Buffer;
use super::color::rgb_from_u8;

use minifb::{Key, WindowOptions};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum WindowError {
    #[error("Unable to open Window.")]
    CannotOpen,
}

pub struct Window {
    width: usize,
    height: usize,
    close: bool,
    buffer: Buffer,
    minifb_window: minifb::Window,
}

impl Window {
    pub fn new(title: &str, width: usize, height: usize) -> Result<Self, WindowError> {
        let minifb_window = minifb::Window::new(title, width, height, WindowOptions::default())
            .map_err(|_| WindowError::CannotOpen)?;

        Ok(Self {
            width,
            height,
            close: false,
            buffer: Buffer::new(width, height),
            minifb_window,
        })
    }

    pub fn buffer_as_mut(&mut self) -> &mut Buffer {
        &mut self.buffer
    }

    pub fn get_keys(&self) -> Vec<Key> {
        self.minifb_window.get_keys()
    }

    pub fn should_close(&self) -> bool {
        self.close || !self.minifb_window.is_open()
    }

    pub fn close(&mut self) {
        self.close = true;
    }

    pub fn update(&mut self) {
        self.minifb_window
            .update_with_buffer(self.buffer.as_bytes(), self.width, self.height)
            .unwrap();
    }

    pub fn clear(&mut self, r: u8, g: u8, b: u8) {
        for y in 0..self.height {
            for x in 0..self.width {
                self.buffer.set_pixel(x, y, rgb_from_u8(r, g, b)).unwrap();
            }
        }
    }
}
