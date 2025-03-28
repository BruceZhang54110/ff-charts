use plotters::prelude::*;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a 800*600 bitmap and start drawing
    let mut backend = BitMapBackend::new("aaaa.png", (500, 500));
    // And if we want SVG backend
    // let mut backend = SVGBackend::new("output.svg", (800, 600));
    backend.draw_rect((50, 50), (300, 300), &RED, true)?;
    backend.present()?;
    Ok(())
}