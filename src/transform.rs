use crate::matrix::Matrix;
use crate::{Result, Error};
use std::convert::TryFrom;

pub fn transform<T>(value: T, trans: &Matrix<f64>) -> Result<T>
    where Matrix<f64>: From<T>, T: TryFrom<Matrix<f64>>
{
    let m = Matrix::<f64>::from(value);
    let transformed = trans.multiply(&m)?;
    match T::try_from(transformed) {
        Ok(ret) => Ok(ret),
        Err(_) => Err(Error{message: "Conversion failed".to_string()})
    }
}

pub fn translation(dx: f64, dy:f64, dz: f64) -> Matrix<f64> {
    let mut ret = Matrix::identity(4);

    ret.set(0, 3, dx);
    ret.set(1, 3, dy);
    ret.set(2, 3, dz);

    ret
}

pub fn scaling(sx: f64, sy: f64, sz: f64) -> Matrix<f64> {
    let mut ret = Matrix::identity(4);

    ret.set(0, 0, sx);
    ret.set(1, 1, sy);
    ret.set(2, 2, sz);

    ret
}

pub fn rotation_x(phi: f64) -> Matrix<f64> {
    let mut ret = Matrix::identity(4);

    ret.set(1, 1, phi.cos());
    ret.set(1, 2, -phi.sin());
    ret.set(2, 1, phi.sin());
    ret.set(2, 2, phi.cos());

    ret
}

pub fn rotation_y(phi: f64) -> Matrix<f64> {
    let mut ret = Matrix::identity(4);

    ret.set(0, 0, phi.cos());
    ret.set(0, 2, phi.sin());
    ret.set(2, 0, -phi.sin());
    ret.set(2, 2, phi.cos());

    ret
}

pub fn rotation_z(phi: f64) -> Matrix<f64> {
    let mut ret = Matrix::identity(4);

    ret.set(0, 0, phi.cos());
    ret.set(0, 1, -phi.sin());
    ret.set(1, 0, phi.sin());
    ret.set(1, 1, phi.cos());

    ret
}

pub fn shearing(xy: f64, xz: f64, yx:f64, yz: f64, zx: f64, zy: f64) -> Matrix<f64> {
    let mut ret = Matrix::identity(4);

    ret.set(0, 1, xy);
    ret.set(0, 2, xz);
    ret.set(1, 0, yx);
    ret.set(1, 2, yz);
    ret.set(2, 0, zx);
    ret.set(2, 1, zy);

    ret
}

// ============================================================================

#[cfg(test)]
mod tests {
    use crate::core::{Point, Vector};
    use super::{translation, transform};
    use crate::testutil::*;
    use crate::transform::{scaling, rotation_x, rotation_y, rotation_z, shearing};
    use std::f64::consts::PI;

    #[test]
    fn translate_point() {
        let p = Point::new(-3.0, 4.0, 5.0);
        let t = translation(5.0, -3.0, 2.0);
        let t_inv = t.invert().unwrap();

        let exp = Point::new(2.0, 1.0, 7.0);
        let mut act = transform(p, &t).unwrap();

        assert_point_eq(exp, act);

        act = transform(act, &t_inv).unwrap();

        assert_point_eq(p, act);
    }

    #[test]
    fn translate_vector() {
        let v = Vector::new(-3.0, 4.0, 5.0);
        let t = translation(5.0, -3.0, 2.0);
        let v_translated = transform(v, &t).unwrap();

        assert_vector_eq(v, v_translated);
    }

    #[test]
    fn scale_point() {
        let p = Point::new(2.0,3.0,4.0);
        let s = scaling(-4.0, 6.0, 8.0);
        let exp = Point::new(-8.0, 18.0, 32.0);

        assert_point_eq(exp, transform(p, &s).unwrap());
    }

    #[test]
    fn scale_vector() {
        let v = Vector::new(2.0,3.0,4.0);
        let s = scaling(-4.0, 6.0, 8.0);
        let exp = Vector::new(-8.0, 18.0, 32.0);

        assert_vector_eq(exp, transform(v, &s).unwrap());
    }

    #[test]
    fn rotate_point_x() {
        let p = Point::new(0.0, 1.0, 0.0);
        let half_quarter = transform(p, &rotation_x(PI / 4.0)).unwrap();
        let full_quarter = transform(p, &rotation_x(PI / 2.0)).unwrap();

        assert_point_eq(half_quarter, Point::new(0.0, 2.0_f64.sqrt()/2.0, 2.0_f64.sqrt()/2.0));
        assert_point_eq(full_quarter, Point::new(0.0, 0.0, 1.0));
    }

    #[test]
    fn rotate_point_y() {
        let p = Point::new(0.0, 0.0,1.0);
        let half_quarter = transform(p, &rotation_y(PI / 4.0)).unwrap();
        let full_quarter = transform(p, &rotation_y(PI / 2.0)).unwrap();

        assert_point_eq(half_quarter, Point::new(2.0_f64.sqrt()/2.0, 0.0, 2.0_f64.sqrt()/2.0));
        assert_point_eq(full_quarter, Point::new(1.0, 0.0, 0.0));
    }

    #[test]
    fn rotate_point_z() {
        let p = Point::new(0.0, 1.0,0.0);
        let half_quarter = transform(p, &rotation_z(PI / 4.0)).unwrap();
        let full_quarter = transform(p, &rotation_z(PI / 2.0)).unwrap();

        assert_point_eq(half_quarter, Point::new(-2.0_f64.sqrt()/2.0, 2.0_f64.sqrt()/2.0, 0.0));
        assert_point_eq(full_quarter, Point::new(-1.0, 0.0, 0.0));
    }

    #[test]
    fn shear_point() {
        let p = Point::new(2.0, 3.0, 4.0);

        let mut t = shearing(1.0, 0.0, 0.0, 0.0, 0.0, 0.0);
        assert_point_eq(Point::new(5.0, 3.0, 4.0), transform(p, &t).unwrap());

        t = shearing(0.0, 1.0, 0.0, 0.0, 0.0, 0.0);
        assert_point_eq(Point::new(6.0, 3.0, 4.0), transform(p, &t).unwrap());

        t = shearing(0.0, 0.0, 1.0, 0.0, 0.0, 0.0);
        assert_point_eq(Point::new(2.0, 5.0, 4.0), transform(p, &t).unwrap());

        t = shearing(0.0, 0.0, 0.0, 1.0, 0.0, 0.0);
        assert_point_eq(Point::new(2.0, 7.0, 4.0), transform(p, &t).unwrap());

        t = shearing(0.0, 0.0, 0.0, 0.0, 1.0, 0.0);
        assert_point_eq(Point::new(2.0, 3.0, 6.0), transform(p, &t).unwrap());

        t = shearing(0.0, 0.0, 0.0, 0.0, 0.0, 1.0);
        assert_point_eq(Point::new(2.0, 3.0, 7.0), transform(p, &t).unwrap());
    }

    #[test]
    fn chaining_transformations() {

        let a = rotation_x(PI / 2.0);
        let b = scaling(5.0, 5.0, 5.0);
        let c = translation(10.0, 5.0,7.0);
        let d = c.multiply(&b).unwrap().multiply(&a).unwrap();

        let p = Point::new(1.0, 0.0, 1.0);
        let p2 = transform(p, &a).unwrap();
        let p3 = transform(p2, &b).unwrap();
        let p4 = transform(p3, &c).unwrap();

        let p5 = transform(p, &d).unwrap();

        assert_point_eq(p4, p5);
    }
}