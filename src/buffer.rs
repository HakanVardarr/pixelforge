use super::color;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum BufferError {
    #[error("Given position is out of bounds: (x: {0} y:{1})")]
    OutOfBounds(usize, usize),
}

pub struct Buffer {
    pub width: usize,
    pub height: usize,
    data: Vec<u32>,
}

impl Buffer {
    pub fn new(width: usize, height: usize) -> Self {
        let data = vec![color::BLACK; width * height];
        Self {
            width,
            height,
            data,
        }
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, color: u32) -> Result<(), BufferError> {
        if x > self.width || y > self.height {
            return Err(BufferError::OutOfBounds(x, y));
        }

        self.data[y * self.width + x] = color;
        Ok(())
    }

    pub fn as_bytes<'a>(&'a self) -> &'a [u32] {
        self.data.as_slice()
    }
}
