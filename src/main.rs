use umbralux::core::{Point, Vector};

fn main() {

    let mut p = Projectile{
        position: Point::new(0.0, 1.0, 0.0),
        velocity: Vector::new(1.0, 1.0, 0.0).normalize(),
    };

    let env = Environment{
        gravity: Vector::new(0.0, -0.1, 0.0),
        wind: Vector::new(-0.01, 0.0, 0.0),
    };

    while p.position.y() > 0.0 {
        println!("Pos.: {:?} Veloc.: {:?}", p.position, p.velocity);
        p = tick(&p, &env);
    }

}

struct Projectile {
    position: Point,
    velocity: Vector,
}

struct Environment {
    gravity: Vector,
    wind: Vector,
}

fn tick(proj: &Projectile, env: &Environment) -> Projectile {
    let position = proj.position + proj.velocity;
    let velocity = proj.velocity + env.gravity + env.wind;
    Projectile{position, velocity}
}