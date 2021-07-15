use std::ops::{Add, Mul, Sub};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Point(f64, f64, f64);

impl Point {

    pub fn new(x: f64, y: f64, z: f64) -> Point {
        Point(x, y, z)
    }
    
    pub fn x(&self) -> f64 {
        self.0
    }

    pub fn y(&self) -> f64 {
        self.1
    }

    pub fn z(&self) -> f64 {
        self.2
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

impl Mul<Point> for f64 {
    type Output = Point;

    fn mul(self: f64, pt: Point) -> Point {
        Point::new(self * pt.0, self * pt.1, self * pt.2)
    }
}

impl Mul<f64> for Point {
    type Output = Point;

    fn mul(self: Point, scale: f64) -> Point {
        Point::new(self.0 * scale, self.1 * scale, self.2 * scale)
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vector(f64, f64, f64);

impl Vector {

    pub fn new(x: f64, y: f64, z: f64) -> Vector {
        Vector(x, y, z)
    }

    pub fn x(&self) -> f64 {
        self.0
    }

    pub fn y(&self) -> f64 {
        self.1
    }

    pub fn z(&self) -> f64 {
        self.2
    }

    pub fn magnitude(self: &Vector) -> f64 {
        (self.0 * self.0 + self.1 * self.1 + self.2 * self.2).sqrt()
    }

    pub fn normalize(self: &Vector) -> Vector {
        let m = self.magnitude();
        Vector::new(self.0 / m, self.1 / m, self.2 / m)
    }

    pub fn dot(self: &Vector, other: Vector) -> f64 {
        self.0 * other.0 +
        self.1 * other.1 + 
        self.2 * other.2
    }

    pub fn cross(self: &Vector, other: Vector) -> Vector {
        Vector::new(
            self.1 * other.2 - self.2 * other.1,
            self.2 * other.0 - self.0 * other.2,
            self.0 * other.1 - self.1 * other.0
        )
    }
    
}

impl Add for Vector {
    type Output = Vector;

    fn add(self: Vector, other: Vector) -> Vector {
        Vector(self.0 + other.0, self.1 + other.1, self.2 + other.2)
    }
}

impl Sub for Vector {
    type Output = Vector;

    fn sub(self: Vector, other: Vector) -> Vector {
        Vector::new(self.0 - other.0, self.1 - other.1, self.2 - other.2)
    }
}

impl Mul<Vector> for f64 {
    type Output = Vector;

    fn mul(self: f64, v: Vector) -> Vector {
        Vector::new(self * v.0, self * v.1, self * v.2)
    }
}

impl Mul<f64> for Vector {
    type Output = Vector;

    fn mul(self: Vector, scale: f64) -> Vector {
        Vector::new(self.0 * scale, self.1 * scale, self.2 * scale)
    }
}



#[cfg(test)]
mod tests {

    use super::{Point, Vector};

    fn assert_point_eq(pt1: Point, pt2: Point) {
        assert_float_absolute_eq!(pt1.0, pt2.0);
        assert_float_absolute_eq!(pt1.1, pt2.1);
        assert_float_absolute_eq!(pt1.2, pt2.2);
    }

    fn assert_vector_eq(v1: Vector, v2: Vector) {
        assert_float_absolute_eq!(v1.0, v2.0);
        assert_float_absolute_eq!(v1.1, v2.1);
        assert_float_absolute_eq!(v1.2, v2.2);
    }

    #[test]
    fn add_vector_to_point() {
        let p1 = Point::new(1.0, 2.0, 3.0);
        let v1 = Vector::new(-1.0, -2.0, -3.0);
        let p2 = p1 + v1;

        assert_point_eq(p2, Point::new(0.0, 0.0, 0.0));
    }

    #[test]
    fn subtract_points() {
        let pt = Point::new(1.0, 2.0, 3.0);

        assert_vector_eq(Vector::new(0.0, 0.0, 0.0), pt - pt);
    }

    #[test]
    fn subtract_vector_from_point() {
        let pt = Point::new(1.0, 2.0, 3.0);
        let v = Vector::new(1.0, 2.0, -3.0);

        assert_point_eq(Point::new(0.0, 0.0, 6.0), pt - v);
    }

    #[test]
    fn scale_point() {
        let pt = Point::new(1.0, 2.0, 3.0);

        assert_point_eq(Point::new(2.0, 4.0, 6.0), 2.0 * pt);
        assert_point_eq(Point::new(2.0, 4.0, 6.0), pt * 2.0);
    }

    #[test]
    fn add_vectors() {
        let v1 = Vector::new(1.0, 0.0, 1.0);
        let v2 = Vector::new(0.0, 2.0, 2.0);

        assert_vector_eq(Vector::new(1.0, 2.0, 3.0), v1 + v2);
    }

    #[test]
    fn subtract_vectors() {
        let v1 = Vector::new(1.0, 0.0, 1.0);
        let v2 = Vector::new(0.0, 2.0, 2.0);

        assert_vector_eq(Vector::new(1.0, -2.0, -1.0), v1 - v2);
    }

    #[test]
    fn scale_vector() {
        let v = Vector::new(1.0, 2.0, 3.0);

        assert_vector_eq(Vector::new(2.0, 4.0, 6.0), 2.0 * v);
        assert_vector_eq(Vector::new(2.0, 4.0, 6.0), v * 2.0);
    }

    #[test]
    fn vector_magnitude() {
        let v = Vector::new(3.0, 4.0, 0.0);

        assert_float_absolute_eq!(5.0, v.magnitude());
    }

    #[test]
    fn vector_normalize() {
        let v = Vector::new(4.0, 0.0, 0.0);

        assert_vector_eq(v.normalize(), Vector::new(1.0, 0.0, 0.0));
    }

    #[test]
    fn vector_dot() {
        let v1 = Vector::new(1.0, 2.0, 3.0);
        let v2 = Vector::new(2.0, 3.0, 4.0);

        assert_float_absolute_eq!(v1.dot(v2), 20.0);
    }

    #[test]
    fn vector_cross() {
        let v1 = Vector::new(1.0, 2.0, 3.0);
        let v2 = Vector::new(2.0, 3.0, 4.0);

        assert_vector_eq(v1.cross(v2), Vector::new(-1.0, 2.0, -1.0));
        assert_vector_eq(v2.cross(v1), Vector::new(1.0, -2.0, 1.0));
        assert_vector_eq(v1.cross(v1), Vector::new(0.0, 0.0, 0.0));
    }
}
