use std::env;
use std::f64::consts::FRAC_PI_3;
use std::io::Result;
use std::iter::FromIterator;
use std::rc::Rc;
use std::str::FromStr;
use umbralux::camera::Camera;
use umbralux::core::{Color, Point, Vector};
use umbralux::features::light::PointLight;
use umbralux::features::material::MaterialBuilder;
use umbralux::io::{export_as_png, export_as_ppm};
use umbralux::objects::object3d::Object3D;
use umbralux::objects::plane::Plane;
use umbralux::objects::sphere::Sphere;
use umbralux::objects::world::World;
use umbralux::transform::{scaling, translation, view_transform};

fn main() -> Result<()> {

    let mut world = World::new();
    let (width, height) = parse_args();

    println!("Creating world image: {} x {}", width, height);

    let floor = create_floor();
    world.add_object(&floor);
    world.add_object(&create_middle_sphere());
    world.add_object(&create_smaller_sphere());
    world.add_object(&create_smallest_sphere());

    let light = Rc::new(PointLight {
        intensity: Color::new(1.0,1.0, 1.0, ),
        position: Point::new( - 10.0, 10.0,-10.0),
    });
    world.set_light(&light);

    let mut camera = Camera::new(width, height, FRAC_PI_3);
    camera.set_transformation(view_transform(Point::new(0.0, 1.5, -5.8),
        Point::new(0.0, 1.0, 0.0), Vector::new(0.0, 1.0, 0.0)));

    let canvas = camera.render(&world);

    export_as_ppm(&canvas, "spheres_above_plane.ppm")?;

    export_as_png(&canvas, "spheres_above_plane.png")?;

    Ok(())
}

fn parse_args() -> (usize, usize) {
    let args = Vec::from_iter(env::args());
    match args.len() {
        2 => {
            let width = usize::from_str(&args[1]).unwrap();
            let height = width;
            (width, height)
        }
        3 => {
            let width = usize::from_str(&args[1]).unwrap();
            let height= usize::from_str(&args[2]).unwrap();
            (width, height)
        }
        _ => (400, 200)
    }
}

fn create_floor() -> Rc<dyn Object3D> {

    let floor = Plane::new();

    let mat = MaterialBuilder::new()
        .color(Color::new(1.0, 0.9, 0.9))
        .specular(0.0)
        .build();
    floor.change_material(mat);

    Rc::new(floor)
}

fn create_middle_sphere() -> Rc<dyn Object3D> {
    let middle = Sphere::new_unit();

    middle.change_transformation(translation(-0.5, 1.0, 0.5));

    let mat = MaterialBuilder::new()
        .color(Color::new(0.1, 1.0, 0.5))
        .diffuse(0.7)
        .specular(0.3)
        .build();
    middle.change_material(mat);

    Rc::new(middle)
}

fn create_smaller_sphere() -> Rc<dyn Object3D> {
    let sphere = Sphere::new_unit();

    sphere.change_transformation(translation(1.5, 0.5, -0.5) *
        scaling(0.5, 0.5, 0.5));

    let mat = MaterialBuilder::new()
        .color(Color::new(0.5, 1.0, 0.1))
        .diffuse(0.7)
        .specular(0.3)
        .build();
    sphere.change_material(mat);

    Rc::new(sphere)
}

fn create_smallest_sphere() -> Rc<dyn Object3D> {
    let sphere = Sphere::new_unit();

    sphere.change_transformation(translation(-1.5, 0.33, -0.75) *
        scaling(0.33, 0.33, 0.33));

    let mat = MaterialBuilder::new()
        .color(Color::new(1.0, 0.8, 0.1))
        .diffuse(0.7)
        .specular(0.3)
        .build();
    sphere.change_material(mat);

    Rc::new(sphere)
}