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
use umbralux::features::pattern::{NestedPattern, Pattern, TwoColorPattern};
use umbralux::io::export_as_png;
use umbralux::objects::object3d::Object3D;
use umbralux::objects::plane::Plane;
use umbralux::objects::world::World;
use umbralux::transform::view_transform;

fn main() -> Result<()> {

    let (width, height) = parse_args();

    let mut world = World::new();
    let plane: Rc<dyn Object3D> = Rc::new(create_plane());

    world.add_object(&plane);

    let light = Rc::new(PointLight {
        intensity: Color::new(1.0,1.0, 1.0, ),
        position: Point::new( - 10.0, 10.0,-10.0),
    });
    world.set_light(&light);

    let mut camera = Camera::new(width, height, FRAC_PI_3);
    camera.set_transformation(view_transform(Point::new(0.0, 1.5, -5.8),
                                             Point::new(0.0, 1.0, 0.0),
                                             Vector::new(0.0, 1.0, 0.0)));

    let canvas = camera.render(&world);

    export_as_png(&canvas, "pattern_demo.png")?;

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

fn create_plane() -> Plane {
    let ret = Plane::new();
    let pattern_a: Rc<dyn Pattern> = Rc::new(TwoColorPattern::new_stripes(
        Color::new(1., 0., 0.),
        Color::new(1., 1., 1.)
    ));
    let pattern_b: Rc<dyn Pattern> = Rc::new(TwoColorPattern::new_stripes(
        Color::new(0., 0., 1.),
        Color::new(0., 1., 0.)
    ));
    let pattern: Rc<dyn Pattern> = Rc::new(NestedPattern::new_checkers3d(
        pattern_a,
        pattern_b
    ));

    let mat = MaterialBuilder::new()
        .pattern(&pattern)
        .build();
    ret.change_material(mat);

    ret
}
