use anyhow::Result;
use umbralux::core::{Canvas, Color, Point, Vector};
use umbralux::io::save_canvas;

fn main() -> Result<()> {

    let mut p = Projectile::new(
        Point::new(0.0, 1.0, 0.0),
        Vector::new(1.0, 1.8, 0.0).normalize() * 11.25,
    );

    let env = Environment::new(
        Vector::new(0.0, -0.1, 0.0),
        Vector::new(-0.01, 0.0, 0.0),
    );

    let bg_color = Color::new(0.0, 0.0, 0.0);
    let fg_color = Color::new(1.0, 1.0, 1.0);
    let mut canvas = Canvas::new_with_color(900, 500, &bg_color);

    while p.position.y() > 0.0 {
        let col = p.position.x() as usize;
        let row = canvas.height() - p.position.y() as usize;
        canvas.set_pixel(row, col, &fg_color);
        tick(&mut p, &env);
    }

    save_canvas("trajectory.jpg", &canvas)
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