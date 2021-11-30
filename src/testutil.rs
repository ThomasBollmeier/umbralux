#![cfg(test)]

use crate::core::{Point, Vector, Color};
use crate::matrix::Matrix;

const EPS: f64 = 1.0E-5;

pub fn assert_point_eq(pt1: Point, pt2: Point) {
    assert_float_absolute_eq!(pt1.x(), pt2.x(), EPS);
    assert_float_absolute_eq!(pt1.y(), pt2.y(), EPS);
    assert_float_absolute_eq!(pt1.z(), pt2.z(), EPS);
}

pub fn assert_vector_eq(v1: Vector, v2: Vector) {
    assert_float_absolute_eq!(v1.x(), v2.x(), EPS);
    assert_float_absolute_eq!(v1.y(), v2.y(), EPS);
    assert_float_absolute_eq!(v1.z(), v2.z(), EPS);
}

pub fn assert_color_eq(c1: Color, c2: Color) {
    assert_float_absolute_eq!(c1.red(), c2.red(), EPS);
    assert_float_absolute_eq!(c1.green(), c2.green(), EPS);
    assert_float_absolute_eq!(c1.blue(), c2.blue(), EPS);
}

pub fn assert_matrix_float_eq(a: &Matrix<f64>, b: &Matrix<f64>) {
    let (na, ma) = a.size();
    let (nb, mb) = b.size();

    assert_eq!(na, nb);
    assert_eq!(ma, mb);

    for r in 0..na {
        for c in 0..ma {
            assert_float_absolute_eq!(a.get(r, c), b.get(r, c), EPS);
        }
    }
}