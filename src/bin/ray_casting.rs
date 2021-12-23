use std::io::Result;
use std::rc::Rc;
use umbralux::canvas::Canvas;
use umbralux::core::{Color, Point};
use umbralux::features::light::{lighting, PointLight};
use umbralux::features::material::MaterialBuilder;
use umbralux::io::export_as_ppm;
use umbralux::objects::ray::Ray;
use umbralux::objects::sphere::Sphere;
use umbralux::objects::object3d::{find_hit, find_intersections, Intersection, Object3D};
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
    let sphere: Rc<dyn Object3D> = Rc::new(create_sphere());
    let light = create_light();

    let bg_color = Color::new(0.0, 0.0, 0.0);

    let mut canvas = Canvas::with_background(csize.width, csize.height, bg_color);

    for row in 0..csize.height {
        let y = row_to_y(row, &csize, &wsize);
        for col in 0..csize.width {
            let x = col_to_x(col, &csize, &wsize);
            let ray = create_ray(x, y, camera);
            let hit_opt = find_hit(find_intersections(&ray, &sphere));
            if let Some(hit) = hit_opt {
                let color = determine_color(&ray, &light, &hit);
                canvas.set_pixel(col, row, color);
            }
        }
    }

    export_as_ppm(&canvas, "ray_casting.ppm")?;

    Ok(())
}

fn determine_color(ray: &Rc<Ray>, light: &PointLight, hit: &Intersection) -> Color {
    let partner = hit.partner();
    let pos = hit.position();
    let surface = partner.normal_at(pos);
    let eye = -1.0 * ray.direction();

    lighting(
        &partner.material(),
        &light,
        &pos,
        &eye,
        &surface,
        false)
}

fn create_sphere() -> Sphere {
    let sphere = Sphere::new_unit();
    sphere.change_transformation(scaling(1.0, 1.5, 1.0) * translation(0.0, 0.0, 5.0));
    let material = MaterialBuilder::new()
        .color(Color::new(1.0, 0.2, 1.0))
        .build();
    sphere.change_material(material);
    sphere
}

fn create_light() -> PointLight {
    PointLight{
        intensity: Color::new(1.0, 1.0, 1.0),
        position: Point::new(0.0, 10.0, 10.0),
    }
}

fn row_to_y(row: usize, csize: &CanvasSize, wsize: &WorldSize) -> f64 {
    wsize.y_max - row as f64 * (wsize.y_max - wsize.y_min) / (csize.height as f64 - 1.0)
}

fn col_to_x(col: usize, csize: &CanvasSize, wsize: &WorldSize) -> f64 {
    wsize.x_min + col as f64 * (wsize.x_max - wsize.x_min) / (csize.width as f64 - 1.0)
}

fn create_ray(x: f64, y: f64, camera: Point) -> Rc<Ray> {
    let direction = (Point::new(x, y, 0.0) - camera).normalize();
    Rc::new(Ray::new(camera, direction))
}
