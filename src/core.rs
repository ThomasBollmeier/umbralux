use std::ops::{Add, Sub};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Point(f32, f32, f32);

impl Point {
    pub fn new(x: f32, y: f32, z: f32) -> Point {
        Point(x, y, z)
    }
}

impl Add<Vector> for Point {
    type Output = Point;

    fn add(self: Point, movement: Vector) -> Point {
        Point(
            self.0 + movement.0,
            self.1 + movement.1,
            self.2 + movement.2,
        )
    }
}

impl Sub for Point {
    type Output = Vector;

    fn sub(self: Point, other: Point) -> Vector {
        Vector::new(self.0 - other.0, self.1 - other.1, self.2 - other.2)
    }
}

impl Sub<Vector> for Point {
    type Output = Point;

    fn sub(self: Point, movement: Vector) -> Point {
        Point(
            self.0 - movement.0,
            self.1 - movement.1,
            self.2 - movement.2,
        )
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vector(f32, f32, f32);

impl Vector {
    pub fn new(x: f32, y: f32, z: f32) -> Vector {
        Vector(x, y, z)
    }
}

impl Add for Vector {

    type Output = Vector;

    fn add(self: Vector, other: Vector) -> Vector {
        Vector(
            self.0 + other.0,
            self.1 + other.1,
            self.2 + other.2,
        )
    }

}

#[cfg(test)]
mod tests {

    use super::{Point, Vector};

    #[test]
    fn add_vector_to_point() {

        let p1 = Point::new(1.0, 2.0, 3.0);
        let v1 = Vector::new(-1.0, -2.0, -3.0);
        let p2 = p1 + v1;

        println!("{:?}, {:?}", p1, p2);

        assert_eq!(2 + 2, 3);
    }
}