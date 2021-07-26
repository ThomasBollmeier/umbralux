use std::ops::{Add, Mul, Sub};
use crate::matrix::Matrix;
use std::convert::TryFrom;

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

impl TryFrom<Matrix<f64>> for Point {
    type Error = crate::Error;

    fn try_from(value: Matrix<f64>) -> Result<Self, Self::Error> {
        let (n, m) = value.size();
        if n < 3 || m != 1 {
            return Err(Self::Error{message: "Invalid matrix size".to_string()});
        }

        Ok(Point(
            value.get(0, 0),
            value.get(1, 0),
            value.get(2, 0),
        ))
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
        Point(self * pt.0, self * pt.1, self * pt.2)
    }
}

impl Mul<f64> for Point {
    type Output = Point;

    fn mul(self: Point, scale: f64) -> Point {
        Point(self.0 * scale, self.1 * scale, self.2 * scale)
    }
}
// ============================================================================

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
        Vector(self.0 / m, self.1 / m, self.2 / m)
    }

    pub fn dot(self: &Vector, other: Vector) -> f64 {
        self.0 * other.0 + self.1 * other.1 + self.2 * other.2
    }

    pub fn cross(self: &Vector, other: Vector) -> Vector {
        Vector(
            self.1 * other.2 - self.2 * other.1,
            self.2 * other.0 - self.0 * other.2,
            self.0 * other.1 - self.1 * other.0,
        )
    }
}

impl TryFrom<Matrix<f64>> for Vector {
    type Error = crate::Error;

    fn try_from(value: Matrix<f64>) -> Result<Self, Self::Error> {
        let (n, m) = value.size();
        if n < 3 || m != 1 {
            return Err(Self::Error{message: "Invalid matrix size".to_string()});
        }

        Ok(Vector(
            value.get(0, 0),
            value.get(1, 0),
            value.get(2, 0),
        ))
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
        Vector(self.0 - other.0, self.1 - other.1, self.2 - other.2)
    }
}

impl Mul<Vector> for f64 {
    type Output = Vector;

    fn mul(self: f64, v: Vector) -> Vector {
        Vector(self * v.0, self * v.1, self * v.2)
    }
}

impl Mul<f64> for Vector {
    type Output = Vector;

    fn mul(self: Vector, scale: f64) -> Vector {
        Vector(self.0 * scale, self.1 * scale, self.2 * scale)
    }
}

// ============================================================================

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Color(f64, f64, f64);

impl Color {
    pub fn new(red: f64, green: f64, blue: f64) -> Color {
        Color(red, green, blue)
    }

    pub fn red(&self) -> f64 {
        self.0
    }

    pub fn green(&self) -> f64 {
        self.1
    }

    pub fn blue(&self) -> f64 {
        self.2
    }
}

impl Add for Color {
    type Output = Color;

    fn add(self: Color, other: Color) -> Color {
        Color(self.0 + other.0, self.1 + other.1, self.2 + other.2)
    }
}

impl Sub for Color {
    type Output = Color;

    fn sub(self: Color, other: Color) -> Color {
        Color(self.0 - other.0, self.1 - other.1, self.2 - other.2)
    }
}

impl Mul<Color> for f64 {
    type Output = Color;

    fn mul(self: f64, c: Color) -> Color {
        Color(self * c.0, self * c.1, self * c.2)
    }
}

impl Mul<f64> for Color {
    type Output = Color;

    fn mul(self: Color, scale: f64) -> Color {
        Color(self.0 * scale, self.1 * scale, self.2 * scale)
    }
}

impl Mul<Color> for Color {
    type Output = Color;

    fn mul(self: Color, other: Color) -> Color {
        // Hadamard product
        Color(self.0 * other.0, self.1 * other.1, self.2 * other.2)
    }
}

// ============================================================================

#[cfg(test)]
mod tests {

    use super::{Point, Vector, Color};
    use crate::matrix::Matrix;
    use std::convert::TryFrom;

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

    fn assert_color_eq(c1: Color, c2: Color) {
        assert_float_absolute_eq!(c1.0, c2.0);
        assert_float_absolute_eq!(c1.1, c2.1);
        assert_float_absolute_eq!(c1.2, c2.2);
    }

    #[test]
    fn point_conversion() {
        let p1 = Point::new(1.0, 2.0, 3.0);
        let m = Matrix::<f64>::from(p1);

        assert_float_absolute_eq!(m.get(3, 0), 1.0);

        let p2 = Point::try_from(m).unwrap();

        assert_point_eq(p1, p2);
    }

    #[test]
    fn vector_conversion() {
        let v1 = Vector::new(1.0, 2.0, 3.0);
        let m = Matrix::<f64>::from(v1);

        assert_float_absolute_eq!(m.get(3, 0), 0.0);

        let v2 = Vector::try_from(m).unwrap();

        assert_vector_eq(v1, v2);
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

    #[test]
    fn add_colors() {
        let c1 = Color(0.9, 0.6, 0.75);
        let c2 = Color(0.7, 0.1, 0.25);

        assert_color_eq(c1 + c2, Color(1.6, 0.7, 1.0));
    }

    #[test]
    fn subtract_colors() {
        let c1 = Color(0.9, 0.6, 0.75);
        let c2 = Color(0.7, 0.1, 0.25);

        assert_color_eq(c1 - c2, Color(0.2, 0.5, 0.5));
    }

    #[test]
    fn multiply_by_scalar() {
        let c = Color(0.2, 0.3, 0.4);
        let exp = Color(0.4, 0.6, 0.8);

        assert_color_eq(2.0 * c, exp);
        assert_color_eq(c * 2.0, exp);
    }

    #[test]
    fn multiply_colors() {
        let c1 = Color(1.0, 0.2, 0.4);
        let c2 = Color(0.9, 1.0, 0.1);
        let exp = Color(0.9, 0.2, 0.04);

        assert_color_eq(c1 * c2, exp);
    }

}
