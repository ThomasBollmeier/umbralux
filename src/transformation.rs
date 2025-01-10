use crate::core::{Matrix, Number, Point, Vec4};
use anyhow::Result;

pub enum Transformation {
    Translation{x: Number, y: Number, z: Number},
}

impl Transformation {
    pub fn create_matrix(&self) -> Matrix {
        match self {
            Transformation::Translation{x, y, z} =>
                Self::create_translation(*x, *y, *z)
        }
    }

    pub fn apply(point: &Point, transformations: &[&Transformation]) -> Result<Point> {
        let mut pt = Matrix::from(&Vec4::from(point));
        for transformation in transformations {
            let m = transformation.create_matrix();
            pt = m * pt;
        }
        let pt = Vec4::try_from(&pt)?;
        let transformed = Point::try_from(pt)?;

        Ok(transformed)
    }

    fn create_translation(x: Number, y: Number, z: Number) -> Matrix {
        let mut ret = Matrix::new_identity(4);
        ret.set(0, 3, x);
        ret.set(1, 3, y);
        ret.set(2, 3, z);
        ret
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::Point;

    #[test]
    fn test_translation() {
        use Transformation::*;
        let point = Point::new(-3.0, 4.0, 5.0);
        let expected = Point::new(2.0, 1.0, 7.0);
        let actual = Transformation::apply(&point,
                                           &vec![
                                               &Translation{
                                                   x: 5.0,
                                                   y: -3.0,
                                                   z: 2.0
                                               },
                                           ]).unwrap();

        assert_eq!(actual, expected);
    }
}