use crate::core::{Point, Vector};
use crate::matrix::Matrix;
use crate::transform::{scaling, transform, translation};

pub struct Ray {
    origin: Point,
    direction: Vector,
}

impl Ray {
    pub fn new(origin: Point, direction: Vector) -> Ray {
        Ray{ origin, direction }
    }

    pub fn origin(&self) -> Point {
        self.origin
    }

    pub fn direction(&self) -> Vector {
        self.direction
    }

    pub fn position(&self, t: f64) -> Point {
        self.origin + t * self.direction
    }

    pub fn transform(&self, m: &Matrix<f64>) -> Ray {
        Ray {
            origin: transform(self.origin, m).unwrap(),
            direction: transform(self.direction, m).unwrap(),
        }
    }

    pub fn translate(&self, dx: f64, dy: f64, dz: f64) -> Ray {
        self.transform(&translation(dx, dy, dz))
    }

    pub fn scale(&self, sx: f64, sy: f64, sz: f64) -> Ray {
        self.transform(&scaling(sx, sy, sz))
    }
}

// ============================================================================

#[cfg(test)]
mod tests {
    use crate::core::{Point, Vector};
    use crate::testutil::{assert_point_eq, assert_vector_eq};
    use super::Ray;

    #[test]
    fn translating_a_ray() {
        let ray = Ray::new(Point::new(1.0, 2.0, 3.0),
                           Vector::new(0.0, 1.0, 0.0));
        let new_ray = ray.translate(3.0, 4.0, 5.0);

        assert_point_eq(new_ray.origin, Point::new(4.0, 6.0, 8.0));
        assert_vector_eq(new_ray.direction, ray.direction);
    }

    #[test]
    fn scaling_a_ray() {
        let ray = Ray::new(Point::new(1.0, 2.0, 3.0),
                           Vector::new(0.0, 1.0, 0.0));
        let new_ray = ray.scale(2.0, 3.0, 4.0);

        assert_point_eq(new_ray.origin, Point::new(2.0, 6.0, 12.0));
        assert_vector_eq(new_ray.direction, Vector::new(0.0, 3.0, 0.0));
    }
}
