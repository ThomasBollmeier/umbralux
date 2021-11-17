use std::io::Result;
use umbralux::canvas::Canvas;
use umbralux::core::{Color, Point, Vector};
use umbralux::io::export_as_ppm;
use umbralux::objects::ray::Ray;
use umbralux::objects::sphere::Sphere;
use umbralux::objects::object3d::Intersect;
use umbralux::transform::{scaling, translation};

struct WorldSize {
    x_min: f64,
    x_max: f64,
    y_min: f64,
    y_max: f64,
}

struct CanvasSize {
    width: usize,
    height: usize,
}

fn main() -> Result<()> {

    let csize = CanvasSize{
        width: 200,
        height: 200,
    };

    let wsize = WorldSize{
        x_min: -4.0,
        x_max: 4.0,
        y_min: -4.0,
        y_max: 4.0,
    };

    let camera = Point::new(0.0, 0.0, 10.0);

    let mut sphere = Sphere::new_unit();
    sphere.set_transformation(scaling(1.0, 1.5, 1.0) * translation(0.0, 0.0, 5.0));

    let bg_color = Color::new(0.0, 0.0, 0.0);
    let fg_color = Color::new(1.0, 0.0, 0.0);

    let mut canvas = Canvas::with_background(csize.width, csize.height, bg_color);

    for row in 0..csize.height {
        let y = row_to_y(row, &csize, &wsize);
        for col in 0..csize.width {
            let x = col_to_x(col, &csize, &wsize);
            let direction = camera - Point::new(x, y, 0.0);
            if ray_hits_sphere(camera, direction, &sphere) {
                canvas.set_pixel(col, row, fg_color);
            }
        }
    }

    export_as_ppm(&canvas, "ray_casting.ppm")?;

    Ok(())
}

fn row_to_y(row: usize, csize: &CanvasSize, wsize: &WorldSize) -> f64 {
    wsize.y_max - row as f64 * (wsize.y_max - wsize.y_min) / (csize.height as f64 - 1.0)
}

fn col_to_x(col: usize, csize: &CanvasSize, wsize: &WorldSize) -> f64 {
    wsize.x_min + col as f64 * (wsize.x_max - wsize.x_min) / (csize.width as f64 - 1.0)
}

fn ray_hits_sphere(camera: Point, direction: Vector, sphere: &Sphere) -> bool {
    let ray = Ray::new(camera, direction);
    let xs = sphere.intersect(&ray);

    !xs.is_empty()
}
