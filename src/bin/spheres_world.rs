use std::f64::consts::{FRAC_PI_2, FRAC_PI_3, FRAC_PI_4};
use std::io::Result;
use std::rc::Rc;
use umbralux::camera::Camera;
use umbralux::core::{Color, Point, Vector};
use umbralux::features::light::PointLight;
use umbralux::features::material::MaterialBuilder;
use umbralux::io::{export_as_png, export_as_ppm};
use umbralux::objects::object3d::Object3D;
use umbralux::objects::sphere::Sphere;
use umbralux::objects::world::World;
use umbralux::transform::{rotation_x, rotation_y, scaling, translation, view_transform};

fn main() -> Result<()> {

    let mut world = World::new();

    let floor = create_floor();
    world.add_object(&floor);
    world.add_object(&create_left_wall(&floor));
    world.add_object(&create_right_wall(&floor));
    world.add_object(&create_middle_sphere());
    world.add_object(&create_smaller_sphere());
    world.add_object(&create_smallest_sphere());

    let light = Rc::new(PointLight {
        intensity: Color::new(1.0,1.0, 1.0, ),
        position: Point::new( - 10.0, 10.0,-10.0),
    });
    world.set_light(&light);

    let mut camera = Camera::new(400, 200, FRAC_PI_3);
    camera.set_transformation(view_transform(Point::new(0.0, 1.5, -5.8),
        Point::new(0.0, 1.0, 0.0), Vector::new(0.0, 1.0, 0.0)));

    let canvas = camera.render(&world);

    export_as_ppm(&canvas, "spheres_world.ppm")?;

    export_as_png(&canvas, "spheres_world.png")?;

    Ok(())
}

fn create_floor() -> Rc<dyn Object3D> {

    let floor = Sphere::new_unit();

    floor.change_transformation(scaling(10.0, 0.01, 10.0));

    let mat = MaterialBuilder::new()
        .color(Color::new(1.0, 0.9, 0.9))
        .specular(0.0)
        .build();
    floor.change_material(mat);

    Rc::new(floor)
}

fn create_left_wall(floor: &Rc<dyn Object3D>) -> Rc<dyn Object3D> {
    let left_wall = Sphere::new_unit();

    left_wall.change_transformation(
        translation(0.0, 0.0, 5.0) * rotation_y(-1.0 * FRAC_PI_4) *
            rotation_x(FRAC_PI_2) * scaling(10.0, 0.01, 10.0));
    left_wall.change_material(floor.material());

    Rc::new(left_wall)
}

fn create_right_wall(floor: &Rc<dyn Object3D>) -> Rc<dyn Object3D> {
    let right_wall = Sphere::new_unit();

    right_wall.change_transformation(
        translation(0.0, 0.0, 5.0) * rotation_y(FRAC_PI_4) *
            rotation_x(FRAC_PI_2) * scaling(10.0, 0.01, 10.0));
    right_wall.change_material(floor.material());

    Rc::new(right_wall)
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