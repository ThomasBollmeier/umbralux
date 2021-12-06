use std::f64::consts::{FRAC_PI_2, FRAC_PI_4};
use std::io::Result;
use std::rc::Rc;
use umbralux::core::Color;
use umbralux::features::material::MaterialBuilder;
use umbralux::objects::object3d::Object3D;
use umbralux::objects::sphere::Sphere;
use umbralux::objects::world::World;
use umbralux::transform::{rotation_x, rotation_y, scaling, translation};

fn main() -> Result<()> {

    let mut world = World::new();

    let floor = create_floor();
    world.add_object(&floor);

    world.add_object(&create_left_wall(&floor));
    world.add_object(&create_right_wall(&floor));

    Ok(())
}

fn create_floor() -> Rc<dyn Object3D> {

    let floor: Rc<dyn Object3D> = Rc::new(Sphere::new_unit());

    let mat = MaterialBuilder::new()
        .color(Color::new(1.0, 0.9, 0.9))
        .specular(0.0)
        .build();
    floor.change_material(mat);

    floor
}

fn create_left_wall(floor: &Rc<dyn Object3D>) -> Rc<dyn Object3D> {
    let mut left_wall = Sphere::new_unit();

    left_wall.set_transformation(
        translation(0.0, 0.0, 5.0) * rotation_y(-1.0 * FRAC_PI_4) *
            rotation_x(FRAC_PI_2) * scaling(10.0, 0.01, 10.0));
    left_wall.change_material(floor.material());

    Rc::new(left_wall)
}

fn create_right_wall(floor: &Rc<dyn Object3D>) -> Rc<dyn Object3D> {
    let mut right_wall = Sphere::new_unit();

    right_wall.set_transformation(
        translation(0.0, 0.0, 5.0) * rotation_y(FRAC_PI_4) *
            rotation_x(FRAC_PI_2) * scaling(10.0, 0.01, 10.0));
    right_wall.change_material(floor.material());

    Rc::new(right_wall)
}