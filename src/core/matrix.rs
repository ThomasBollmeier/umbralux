use std::ops::Mul;
use crate::core::base_types::Vec4;
use crate::core::Number;

#[derive(Debug, Clone)]
pub struct Matrix {
    num_rows: usize,
    num_cols: usize,
    data: Vec<Vec<Number>>,
}

impl Matrix {
    pub fn new(num_rows: usize, num_cols: usize) -> Matrix {
        Self {
            num_rows,
            num_cols,
            data: vec![vec![0.0; num_cols]; num_rows]
        }
    }

    pub fn new_with_data(data: &Vec<Vec<Number>>) -> Matrix {
        let num_rows = data.len();
        let num_cols = data[0].len();
        Self {
            num_rows,
            num_cols,
            data: data.clone(),
        }
    }

    pub fn new_identity(n: usize) -> Matrix {
        let mut ret = Matrix::new(n, n);
        for i in 0..n {
            ret.set(i, i, 1.0);
        }
        ret
    }

    pub fn num_rows(&self) -> usize {
        self.num_rows
    }

    pub fn num_cols(&self) -> usize {
        self.num_cols
    }

    pub fn get(&self, row: usize, col: usize) -> Number {
        self.data[row][col]
    }

    pub fn set(&mut self, row: usize, col: usize, value: Number) {
        self.data[row][col] = value;
    }

    pub fn transpose(&self) -> Matrix {
        let mut ret = Matrix::new(self.num_cols(), self.num_rows());
        for row in 0..self.num_rows {
            for col in 0..self.num_cols {
                ret.set(col, row, self.get(row, col));
            }
        }

        ret
    }
}

impl From<&Vec4> for Matrix {
    fn from(data: &Vec4) -> Self {
        let &Vec4(one, two, three, four) = data;
        let data = vec![
            vec![one],
            vec![two],
            vec![three],
            vec![four]
        ];
        Matrix::new_with_data(&data)
    }
}

impl TryFrom<&Matrix> for Vec4 {
    type Error = anyhow::Error;

    fn try_from(value: &Matrix) -> Result<Self, Self::Error> {
        if value.num_rows() != 4 || value.num_cols() != 1 {
            return Err(anyhow::anyhow!("Invalid number of rows and columns"));
        }
        Ok(Vec4(
            value.get(0, 0),
            value.get(1, 0),
            value.get(2, 0),
            value.get(3, 0),
        ))
    }
}


impl PartialEq for Matrix {
    fn eq(&self, other: &Matrix) -> bool {
        if self.num_rows != other.num_rows || self.num_cols != other.num_cols {
            return false;
        }
        for row in 0..self.num_rows {
            for col in 0..self.num_cols {
                if self.get(row, col) != other.get(row, col) {
                    return false;
                }
            }
        }

        true
    }
}

impl Mul<Matrix> for Matrix {
    type Output = Matrix;

    fn mul(self, rhs: Matrix) -> Self::Output {
        if self.num_cols != rhs.num_rows {
            panic!("Matrix dimensions do not match");
        }
        let mut ret = Matrix::new(self.num_rows, rhs.num_cols);
        for i in 0..self.num_rows {
            for j in 0..rhs.num_cols {
                let mut sum = 0.0;
                for k in 0..self.num_cols {
                    sum += self.data[i][k] * rhs.data[k][j];
                }
                ret.set(i, j, sum);
            }
        }

        ret
    }
}

#[cfg(test)]
mod tests {
    use crate::core::base_types::Vec4;
    use crate::core::matrix::Matrix;

    #[test]
    fn test_equality() {
        let data = vec![
            vec![1.0, 2.0],
            vec![3.0, 4.0],
            vec![5.0, 6.0]
        ];
        let m1 = Matrix::new_with_data(&data);
        let m2 = Matrix::new_with_data(&data);

        assert_eq!(m1, m2);
    }

    #[test]
    fn test_multiplication() {
        let a = Matrix::new_with_data(&vec![
            vec![1.0, 2.0, 3.0, 4.0],
            vec![5.0, 6.0, 7.0, 8.0],
            vec![9.0, 8.0, 7.0, 6.0],
            vec![5.0, 4.0, 3.0, 2.0]
        ]);
        let b = Matrix::new_with_data(&vec![
            vec![-2.0, 1.0, 2.0, 3.0],
            vec![3.0, 2.0, 1.0, -1.0],
            vec![4.0, 3.0, 6.0, 5.0],
            vec![1.0, 2.0, 7.0, 8.0]
        ]);
        let expected = Matrix::new_with_data(&vec![
            vec![20.0, 22.0, 50.0, 48.0],
            vec![44.0, 54.0, 114.0, 108.0],
            vec![40.0, 58.0, 110.0, 102.0],
            vec![16.0, 26.0, 46.0, 42.0]
        ]);
        let actual = a * b;

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_multiplication_with_vec4() {
        let a = Matrix::new_with_data(&vec![
           vec![1.0, 2.0, 3.0, 4.0],
           vec![2.0, 4.0, 4.0, 2.0],
           vec![8.0, 6.0, 4.0, 1.0],
           vec![0.0, 0.0, 0.0, 1.0]
        ]);
        let b = Matrix::from(&Vec4(1.0, 2.0, 3.0, 1.0));
        let actual = Vec4::try_from(&(a * b)).unwrap();
        let expected = Vec4(18.0, 24.0, 33.0, 1.0);

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_identity_multiplication() {
        let identity = Matrix::new_identity(3);
        let a = Matrix::new_with_data(&vec![
            vec![1.0, 2.0, 3.0],
            vec![4.0, 5.0, 6.0],
            vec![7.0, 8.0, 9.0]
        ]);
        let b = a.clone() * identity.clone();
        let b2 = identity.clone() * a.clone();

        assert_eq!(a, b);
        assert_eq!(a, b2);
    }

    #[test]
    fn test_transpose() {
        let a = Matrix::new_with_data(&vec![
            vec![1.0, 2.0, 3.0],
            vec![4.0, 5.0, 6.0],
            vec![7.0, 8.0, 9.0]
        ]);
        let expected = Matrix::new_with_data(&vec![
            vec![1.0, 4.0, 7.0],
            vec![2.0, 5.0, 8.0],
            vec![3.0, 6.0, 9.0]
        ]);
        let actual = a.transpose();

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_transpose_identity() {
        let identity = Matrix::new_identity(3);

        assert_eq!(identity.transpose(), identity);
    }
}