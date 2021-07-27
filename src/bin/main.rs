use umbralux::canvas::Canvas;
use umbralux::core::{Color, Point, Vector};
use umbralux::io;

fn main() {
    let mut p = Projectile {
        position: Point::new(0.0, 1.0, 0.0),
        velocity: Vector::new(1.0, 1.8, 0.0).normalize() * 11.25,
    };

    let env = Environment {
        gravity: Vector::new(0.0, -0.1, 0.0),
        wind: Vector::new(-0.01, 0.0, 0.0),
    };

    let white = Color::new(1.0, 1.0, 1.0);

    let width = 900;
    let height = 550;
    let mut canvas = Canvas::new(width, height);

    while p.position.y() > 0.0 {
        let x = p.position.x() as i32;
        let y = height as i32 - p.position.y() as i32;

        if (0..width as i32).contains(&x) && (0..height as i32).contains(&y) {
            canvas.set_pixel(x as usize, y as usize, white);
        }

        p = tick(&p, &env);
    }

    io::export_as_ppm(&canvas, "demo.ppm").expect("Export as ppm failed!");
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
    Projectile { position, velocity }
}
