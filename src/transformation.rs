use crate::core::{Matrix, Number, Vec4};
use anyhow::{anyhow, Error};

pub enum Transformation {
    Translation { x: Number, y: Number, z: Number },
    Scaling { x: Number, y: Number, z: Number },
}

impl Transformation {
    pub fn create_matrix(&self) -> Matrix {
        use Transformation::*;
        match self {
            Translation { x, y, z } => Self::create_translation(*x, *y, *z),
            Scaling {x, y, z} => Self::create_scaling(*x, *y, *z),
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
}

pub fn transform<T>(item: &T, transformations: &[&Transformation]) -> Result<T, Error>
where
    Vec4: for<'a> From<&'a T>,
    T: TryFrom<Vec4>,
    <T as TryFrom<Vec4>>::Error: Send + Sync + std::fmt::Display + 'static,
{
    let mut it = Matrix::from(&Vec4::from(item));
    for transformation in transformations {
        let m = transformation.create_matrix();
        it = m * it;
    }
    let it = Vec4::try_from(&it)?;
    let transformed = match T::try_from(it) {
        Ok(v) => v,
        Err(e) => return Err(anyhow!("{}", e)),
    };

    Ok(transformed)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::{Point, Vector};

    #[test]
    fn test_translate_point() {
        use Transformation::*;
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
        use Transformation::*;
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
        use Transformation::*;
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
        use Transformation::*;
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
}
