use crate::core::{Matrix, Number, Vec4};
use anyhow::{anyhow, Error};

pub enum Transformation {
    Translation { x: Number, y: Number, z: Number },
    Scaling { x: Number, y: Number, z: Number },
    RotationX { phi: Number },
    RotationY { phi: Number },
    RotationZ { phi: Number },
}

impl Transformation {
    pub fn create_matrix(&self) -> Matrix {
        use Transformation::*;
        match self {
            Translation { x, y, z } => Self::create_translation(*x, *y, *z),
            Scaling {x, y, z} => Self::create_scaling(*x, *y, *z),
            RotationX {phi} => Self::create_rotation_x(*phi),
            RotationY {phi} => Self::create_rotation_y(*phi),
            RotationZ {phi} => Self::create_rotation_z(*phi),
        }
    }

    fn create_translation(x: Number, y: Number, z: Number) -> Matrix {
        let mut ret = Matrix::new_identity(4);
        ret.set(0, 3, x);
        ret.set(1, 3, y);
        ret.set(2, 3, z);
        ret
    }

    fn create_scaling(x: Number, y: Number, z: Number) -> Matrix {
        let mut ret = Matrix::new_identity(4);
        ret.set(0, 0, x);
        ret.set(1, 1, y);
        ret.set(2, 2, z);
        ret
    }

    fn create_rotation_x(phi: Number) -> Matrix {
        let mut ret = Matrix::new_identity(4);
        ret.set(1, 1, phi.cos());
        ret.set(1, 2, -phi.sin());
        ret.set(2, 1, phi.sin());
        ret.set(2, 2, phi.cos());
        ret
    }

    fn create_rotation_y(phi: Number) -> Matrix {
        let mut ret = Matrix::new_identity(4);
        ret.set(0, 0, phi.cos());
        ret.set(0, 2, phi.sin());
        ret.set(2, 0, -phi.sin());
        ret.set(2, 2, phi.cos());
        ret
    }

    fn create_rotation_z(phi: Number) -> Matrix {
        let mut ret = Matrix::new_identity(4);
        ret.set(0, 0, phi.cos());
        ret.set(0, 1, -phi.sin());
        ret.set(1, 0, phi.sin());
        ret.set(1, 1, phi.cos());
        ret
    }

}

pub fn transform<T>(item: &T, transformations: &[&Transformation]) -> Result<T, Error>
where
    Vec4: for<'a> From<&'a T>,
    T: TryFrom<Vec4>,
    <T as TryFrom<Vec4>>::Error: Send + Sync + std::fmt::Display + 'static,
{
    let it = transformations.into_iter().fold(
        Matrix::from(&Vec4::from(item)),
        |m, &t| {
            t.create_matrix() * m
        },
    );
    let it = Vec4::try_from(&it)?;
    let transformed = match T::try_from(it) {
        Ok(v) => v,
        Err(e) => return Err(anyhow!("{}", e)),
    };

    Ok(transformed)
}

#[cfg(test)]
mod tests {
    use std::f64::consts::{FRAC_1_SQRT_2, FRAC_PI_2, FRAC_PI_4};
    use super::*;
    use crate::core::{Point, Vector};
    use crate::transformation::Transformation::*;

    #[test]
    fn test_translate_point() {
        let point = Point::new(-3.0, 4.0, 5.0);
        let expected = Point::new(2.0, 1.0, 7.0);
        let actual = transform(
            &point,
            &vec![&Translation {
                x: 5.0,
                y: -3.0,
                z: 2.0,
            }],
        )
        .unwrap();

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_translate_vector() {
        let vector = Vector::new(-3.0, 4.0, 5.0);
        let expected = vector.clone();
        let actual = transform(
            &vector,
            &vec![&Translation {
                x: 5.0,
                y: -3.0,
                z: 2.0,
            }],
        )
            .unwrap();

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_scale_point() {
        let point = Point::new(-3.0, 4.0, 5.0);
        let expected = Point::new(-3.0, -8.0, 15.0);
        let actual = transform(
            &point,
            &vec![&Scaling {
                x: 1.0,
                y: -2.0,
                z: 3.0,
            }],
        )
            .unwrap();

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_scale_vector() {
        let v = Vector::new(-3.0, 4.0, 5.0);
        let expected = Vector::new(-3.0, -8.0, 15.0);
        let actual = transform(
            &v,
            &vec![&Scaling {
                x: 1.0,
                y: -2.0,
                z: 3.0,
            }],
        )
            .unwrap();

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_undo_scaling() {
        let vector = Vector::new(-3.0, 4.0, 5.0);
        let (x, y, z) = (1.0, -2.0, 3.0);
        let scaling = Scaling { x, y, z };
        let scaling_inv = Scaling { x: 1.0/x, y: 1.0/y, z: 1.0/z };
        let scaled = transform(&vector, &vec![&scaling]).unwrap();
        let undone = transform(&scaled, &vec![&scaling_inv]).unwrap();

        assert_eq!(undone, vector);
    }

    #[test]
    fn test_rotation_x() {
        let point = Point::new(0.0, 1.0, 0.0);
        let expected = Point::new(0.0, 0.0, 1.0);
        let rot = RotationX { phi: FRAC_PI_2 };
        let actual = transform(&point, &vec![&rot]).unwrap();

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_rotation_y() {
        let point = Point::new(0.0, 0.0, 1.0);
        let expected = Point::new(FRAC_1_SQRT_2, 0.0, FRAC_1_SQRT_2);
        let rot = RotationY { phi: FRAC_PI_4 };
        let actual = transform(&point, &vec![&rot]).unwrap();

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_rotation_z() {
        let point = Point::new(0.0, 1.0, 0.0);
        let expected = Point::new(-FRAC_1_SQRT_2, FRAC_1_SQRT_2, 0.0);
        let rot = RotationZ { phi: FRAC_PI_4 };
        let actual = transform(&point, &vec![&rot]).unwrap();

        assert_eq!(actual, expected);
    }
}
