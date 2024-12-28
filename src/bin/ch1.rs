use umbralux::core::{Point, Vector};

fn main() {

    let mut p = Projectile::new(
        Point::new(0.0, 1.0, 0.0),
        Vector::new(1.0, 1.0, 0.0),
    );

    let env = Environment::new(
        Vector::new(0.0, -0.1, 0.0),
        Vector::new(-0.01, 0.0, 0.0),
    );

    let mut t = 0;
    while p.position.y() > 0.0 {
        println!("{}: {:?} {:?}", t, p.position, p.velocity);
        t += 1;
        tick(&mut p, &env);
    }

}

fn tick(projectile: &mut Projectile, env: &Environment) {
    let p = projectile.clone();
    projectile.position = p.position + p.velocity.clone();
    projectile.velocity = p.velocity + env.gravity.clone() + env.wind.clone();
}

#[derive(Clone)]
struct Projectile {
    position: Point,
    velocity: Vector,
}

impl Projectile {
    fn new(position: Point, velocity: Vector) -> Projectile {
        Self { position, velocity }
    }
}

struct Environment {
    gravity: Vector,
    wind: Vector,
}

impl Environment {
    pub fn new(gravity: Vector, wind: Vector) -> Environment {
        Self { gravity, wind }
    }

}