use pixelforge::window::Window;

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut window = Window::new("Hello, window!", 800, 600)?;

    while !window.should_close() {
        window.clear(255, 0, 0);
        window.update()
    }

    Ok(())
}
