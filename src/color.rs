pub const BLACK: u32 = rgb_from_u8(0, 0, 0);
pub const RED: u32 = rgb_from_u8(255, 0, 0);
pub const GREEN: u32 = rgb_from_u8(0, 255, 0);
pub const BLUE: u32 = rgb_from_u8(0, 0, 255);
pub const WHITE: u32 = rgb_from_u8(255, 255, 255);

/// Converts 3 u8 values into single u32
///
/// The upper 8-bits of u32 is ignored because `minifb` crate doesn't support it.
/// the next 8-bits are for red channel, the next 8-bits are for green channel,
/// and the next lower 8-bits for the blue channel.
///
/// # Example
/// ```no_run
/// # use pixelforge::color::rgb_from_u8;
///
/// let b = rgb_from_u8(0, 0, 0); // This returns color black in the u32 rgb format
/// ```
pub const fn rgb_from_u8(r: u8, g: u8, b: u8) -> u32 {
    let (r, g, b) = (r as u32, g as u32, b as u32);
    (r << 16) | (g << 8) | b
}
