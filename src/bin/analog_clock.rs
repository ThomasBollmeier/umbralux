use umbralux::canvas::Canvas;
use umbralux::core::{Color, Point};
use std::f64::consts::PI;
use umbralux::transform::{rotation_z, transform};
use umbralux::io;
use std::io::Result;

fn main() -> Result<()> {
    let width = 200;
    let height = width;
    let bg_color = Color::new(0.0, 0.0, 0.0);
    let fg_color = Color::new(1.0, 1.0, 1.0);

    let mut canvas = Canvas::with_background(width, height, bg_color);

    let radius = 0.45 * width as f64;
    let mut p = Point::new(0.0, radius, 0.0);
    let rot = rotation_z(-PI / 6.0);

    for _ in 0..12 {
        let x = (p.x() + (width as f64) / 2.0) as usize;
        let y = ((height as f64) / 2.0 - p.y()) as usize;

        canvas.set_pixel(x, y, fg_color);

        p = transform(p, &rot).unwrap();
    }

    io::export_as_ppm(&canvas, "clock.ppm")?;

    Ok(())
}