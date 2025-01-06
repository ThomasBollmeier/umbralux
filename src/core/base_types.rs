//
// Base types used in umbralux
//
use std::ops::{Add, Div, Mul, Sub};
use anyhow::anyhow;

pub type Number = f64;

const EPSILON: Number = f64::EPSILON;

pub fn is_number_equal(a: Number, b: Number) -> bool {
    (a - b).abs() < EPSILON
}

#[derive(Debug, Clone)]
pub struct Point(Number, Number, Number);

impl Point {
    pub fn new(x: Number, y: Number, z: Number) -> Point {
        Point(x, y, z)
    }

    pub fn x(&self) -> Number {
        self.0
    }

    pub fn y(&self) -> Number {
        self.1
    }

    pub fn z(&self) -> Number {
        self.2
    }
}

impl TryFrom<Vec4> for Point {
    type Error = anyhow::Error;
    fn try_from(v: Vec4) -> Result<Point, Self::Error> {
        if !v.is_point() {
            return Err(anyhow!("Vec4 does not contain a point"));
        }
        Ok(Point::new(v.0, v.1, v.2))
    }
}

impl Add<Vector> for Point {
    type Output = Point;
    fn add(self, rhs: Vector) -> Point {
        Point(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl Sub<Point> for Point {
    type Output = Vector;
    fn sub(self, rhs: Point) -> Vector {
        Vector(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}

impl Sub<Vector> for Point {
    type Output = Point;
    fn sub(self, rhs: Vector) -> Point {
        Point(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}

impl Mul<Number> for Point {
    type Output = Point;
    fn mul(self, rhs: Number) -> Point {
        Point(self.0 * rhs, self.1 * rhs, self.2 * rhs)
    }
}

impl Div<Number> for Point {
    type Output = Point;
    fn div(self, rhs: Number) -> Point {
        Point(self.0 / rhs, self.1 / rhs, self.2 / rhs)
    }
}


impl PartialEq for Point {
    fn eq(&self, other: &Point) -> bool {
        is_number_equal(self.0, other.0) &&
            is_number_equal(self.1, other.1) &&
            is_number_equal(self.2, other.2)
    }
}

#[derive(Debug, Clone)]
pub struct Vector(Number, Number, Number);

impl Vector {
    pub fn new(x: Number, y: Number, z: Number) -> Vector {
        Vector(x, y, z)
    }

    pub fn x(&self) -> Number {
        self.0
    }

    pub fn y(&self) -> Number {
        self.1
    }

    pub fn z(&self) -> Number {
        self.2
    }

    pub fn magnitude(&self) -> Number {
        let squares_sum = self.0 * self.0 + self.1 * self.1 + self.2 * self.2;
        squares_sum.sqrt()
    }

    pub fn normalize(&self) -> Vector {
        let magnitude = self.magnitude();
        self.clone() / magnitude
    }

    pub fn dot(&self, other: &Vector) -> Number {
        self.0 * other.0 + self.1 * other.1 + self.2 * other.2
    }

    pub fn cross(&self, other: &Vector) -> Vector {
        Self (
            self.1 * other.2 - self.2 * other.1,
            self.2 * other.0 - self.0 * other.2,
            self.0 * other.1 - self.1 * other.0,
        )
    }
}

impl TryFrom<Vec4> for Vector {
    type Error = anyhow::Error;
    fn try_from(v: Vec4) -> Result<Vector, Self::Error> {
        if !v.is_vector() {
            return Err(anyhow!("Vec4 does not contain a vector"));
        }
        Ok(Vector::new(v.0, v.1, v.2))
    }
}

impl Add<Vector> for Vector {
    type Output = Vector;
    fn add(self, rhs: Vector) -> Vector {
        Vector(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl Sub<Vector> for Vector {
    type Output = Vector;
    fn sub(self, rhs: Vector) -> Vector {
        Vector(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}

impl Mul<Number> for Vector {
    type Output = Vector;
    fn mul(self, rhs: Number) -> Vector {
        Vector(self.0 * rhs, self.1 * rhs, self.2 * rhs)
    }
}

impl Div<Number> for Vector {
    type Output = Vector;
    fn div(self, rhs: Number) -> Vector {
        Vector(self.0 / rhs, self.1 / rhs, self.2 / rhs)
    }
}

impl PartialEq for Vector {
    fn eq(&self, other: &Vector) -> bool {
        is_number_equal(self.0, other.0) &&
            is_number_equal(self.1, other.1) &&
            is_number_equal(self.2, other.2)
    }
}

#[derive(Debug, Clone)]
pub struct Vec4(pub Number, pub Number, pub Number, pub Number);

impl Vec4 {
    pub fn is_point(&self) -> bool {
        is_number_equal(self.3, 1.0)
    }

    pub fn is_vector(&self) -> bool {
        is_number_equal(self.3, 0.0)
    }
}

impl From<Point> for Vec4 {
    fn from(p: Point) -> Vec4 {
        Vec4(p.0, p.1, p.2, 1.0)
    }
}

impl From<Vector> for Vec4 {
    fn from(v: Vector) -> Vec4 {
        Vec4(v.0, v.1, v.2, 0.0)
    }
}

impl PartialEq for Vec4 {
    fn eq(&self, other: &Vec4) -> bool {
        is_number_equal(self.0, other.0) &&
            is_number_equal(self.1, other.1) &&
            is_number_equal(self.2, other.2) &&
            is_number_equal(self.3, other.3)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_create_point() {
        let p = Vec4::from(Point::new(1.0, 2.0, 3.0));
        assert!(p.is_point());
        assert!(!p.is_vector());
    }

    #[test]
    fn can_add_vector_to_point() {
        let p1 = Point::new(1.0, 2.0, 3.0);
        let v = Vector::new(1.0, 0.0, -1.0);
        let p2 = p1 + v;
        assert_eq!(p2, Point::new(2.0, 2.0, 2.0));
    }

    #[test]
    fn can_subtract_points() {
        let p1 = Point::new(1.0, 2.0, 3.0);
        let p2 = Point::new(2.0, 2.0, 2.0);
        let v = p2 - p1;
        assert_eq!(v, Vector::new(1.0, 0.0, -1.0));
    }

    #[test]
    fn can_subtract_vector_from_point() {
        let p1 = Point::new(1.0, 2.0, 3.0);
        let p2 = Point::new(2.0, 2.0, 2.0);
        let v = Vector::new(1.0, 0.0, -1.0);
        assert_eq!(p1, p2 - v);
    }

    #[test]
    fn can_create_vector() {
        let v = Vec4::from(Vector::new(1.0, 2.0, 3.0));
        assert!(!v.is_point());
        assert!(v.is_vector());
    }

    #[test]
    fn can_add_vectors() {
        let v1 = Vector::new(1.0, 2.0, 3.0);
        let v2 = Vector::new(2.0, 3.0, 4.0);
        let expected = Vector::new(3.0, 5.0, 7.0);
        assert_eq!(expected, v1 + v2);
    }

    #[test]
    fn can_subtract_vectors() {
        let v1 = Vector::new(1.0, 2.0, 3.0);
        let v2 = Vector::new(2.0, 3.0, 4.0);
        let expected = Vector::new(-1.0, -1.0, -1.0);
        assert_eq!(expected, v1 - v2);
    }

    #[test]
    fn can_normalize_vector() {
        let v = Vector::new(3.0, 4.0, 0.0);
        let v_norm = Vector::new(0.6, 0.8, 0.0);
        assert_eq!(v_norm, v.normalize());
    }

    #[test]
    fn two_perpendicular_vectors_have_zero_dot_product() {
        let v1 = Vector::new(1.0, 2.0, 4.0);
        let v2 = Vector::new(2.0, 3.0, -2.0);
        assert_eq!(0.0, v1.dot(&v2));
    }

    #[test]
    fn cross_product_works() {
        let x = Vector::new(1.5, 0.0, 0.0).normalize();
        let y = Vector::new(0.0, 2.0, 0.0).normalize();
        let z = Vector::new(0.0, 0.0, 3.0).normalize();
        assert_eq!(z, x.cross(&y));
        assert_eq!(x, y.cross(&z));
        assert_eq!(y, z.cross(&x));
    }

}