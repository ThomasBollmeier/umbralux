use crate::matrix::Matrix;
use crate::{Result, Error};
use std::convert::TryFrom;

pub fn translation(dx: f64, dy:f64, dz: f64) -> Matrix<f64> {
    let mut ret = Matrix::identity(4);

    ret.set(0, 3, dx);
    ret.set(1, 3, dy);
    ret.set(2, 3, dz);

    ret
}

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

// ============================================================================

#[cfg(test)]
mod tests {
    use crate::core::Point;
    use super::{translation, transform};
    use crate::testutil::*;

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

}