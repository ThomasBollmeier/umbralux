use umbralux::core::{Canvas, Color, Number, Point};
use umbralux::io::save_canvas;
use umbralux::transformation::transform;
use umbralux::transformation::Transformation::RotationZ;

fn main() -> anyhow::Result<()> {

    let width = 500;
    let height = 500;

    let bg_color = Color::new(0.0, 0.0, 0.0);
    let fg_color = Color::new(1.0, 1.0, 1.0);
    let mut canvas = Canvas::new_with_color(width, height, &bg_color);

    let step = std::f64::consts::PI / 6.0;
    let start = Point::new(0.0, 0.8, 0.0);

    for i in 0..12 {
        let phi = i as f64 * step;
        let rotation = RotationZ {phi};
        let point = transform(&start, &[&rotation])?;
        let (row, col) = get_pixel_position(&point, width, height);
        canvas.set_pixel(row, col, &fg_color);
    }

    save_canvas("clock.png", &canvas)?;

    Ok(())
}

fn get_pixel_position(point: &Point, width: usize, height: usize) -> (usize, usize) {
    let row = (1.0 - point.y()) / 2.0 * height as Number;
    let col = (1.0 + point.x()) / 2.0 * width as Number;

    (col as usize, row as usize)
}