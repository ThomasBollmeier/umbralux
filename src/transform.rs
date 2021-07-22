use num_traits::{Num, One, Zero};
use std::fmt::Debug;

#[derive(PartialEq, Debug)]
pub struct Matrix<T> {
    n: usize,
    m: usize,
    elements: Vec<Vec<T>>,
}

impl<T: Num + Zero + One + Copy + Debug> Matrix<T> {
    pub fn new(n: usize, m: usize, init_value: T) -> Self {
        let mut elements = Vec::with_capacity(n);
        for _ in 0..n {
            let mut row: Vec<T> = Vec::with_capacity(m);
            for _ in 0..m {
                row.push(init_value);
            }
            elements.push(row);
        }

        Matrix { n, m, elements }
    }

    pub fn from_elements(elements: &Vec<Vec<T>>) -> Self {
        let n = elements.len();
        assert!(n > 0);
        let m = elements[0].len();
        assert!(m > 0);

        let mut new_elements = Vec::with_capacity(n);

        for row in elements {
            let mut new_row = Vec::with_capacity(m);
            for val in row {
                new_row.push(*val);
            }
            new_elements.push(new_row);
        }

        Matrix {
            n,
            m,
            elements: new_elements,
        }
    }

    pub fn size(&self) -> (usize, usize) {
        (self.n, self.m)
    }

    pub fn get(&self, row: usize, col: usize) -> T {
        self.elements[row][col]
    }

    pub fn set(&mut self, row: usize, col: usize, value: T) {
        self.elements[row][col] = value;
    }

    pub fn scale(&self, factor: T) -> Self {
        let mut new_elems = Vec::with_capacity(self.n);

        for row in &self.elements {
            let mut new_row = Vec::with_capacity(self.m);
            for val in row {
                new_row.push(*val * factor);
            }
            new_elems.push(new_row);
        }

        Matrix {
            n: self.n,
            m: self.m,
            elements: new_elems,
        }
    }

    pub fn multiply(&self, other: &Matrix<T>) -> Option<Self> {
        let (n1, m1) = self.size();
        let (n2, m2) = other.size();

        if m1 != n2 {
            return None;
        }

        let mut new_elements = Vec::new();

        for row in 0..n1 {
            let mut new_row = Vec::new();
            let vals1 = &self.elements[row];
            for col in 0..m2 {
                let mut vals2 = Vec::with_capacity(n2);
                for i in 0..n2 {
                    vals2.push(other.get(i, col));
                }
                new_row.push(Self::sum_of_products(m1, vals1, &vals2));
            }
            new_elements.push(new_row);
        }

        Some(Matrix {
            n: n1,
            m: m2,
            elements: new_elements,
        })
    }

    pub fn transpose(&self) -> Self {
        let mut elements = Vec::new();
        let (n, m) = self.size();

        for col_idx in 0..m {
            let mut row = Vec::new();
            for row_idx in 0..n {
                row.push(self.get(row_idx, col_idx));
            }
            elements.push(row);
        }

        Matrix {n: m, m: n, elements}
    }

    pub fn determinant(&self) -> Option<T> {
        let (n, m) = self.size();
        if n != m {
            return None;
        }

        if n == 1 {
            return Some(self.elements[0][0]);
        }

        let mut sign = T::one();
        let mut ret = T::zero();

        for col in 0..m {
            ret = ret + sign * self.get(0, col) * self.sub_matrix(0, col).determinant().unwrap();
            sign = T::zero() - sign;
        }

        Some(ret)
    }

    pub fn invert(&self) -> Option<Self> {
        if self.n != self.m {
            return None;
        }

        let det = self.determinant().unwrap();
        let mut inv = Matrix::new(self.m, self.n, T::zero());

        for r in 0..self.n {
            for c in 0..self.m {
                let det_sub = self.sub_matrix(r, c).determinant().unwrap();
                let sign = if (r + c) % 2 == 0 {
                    T::one()
                } else {
                    T::zero() - T::one()
                };
                inv.set(c, r, sign * det_sub / det);
            }
        }

        Some(inv)
    }

    fn sub_matrix(&self, row: usize, col: usize) -> Self {
        let (n, m) = self.size();
        let mut elements = Vec::new();

        for r in 0..n {
            if r == row {
                continue;
            }
            let mut new_row = Vec::new();
            for c in 0..m {
                if c == col {
                    continue;
                }
                new_row.push(self.get(r, c));
            }
            elements.push(new_row);
        }

        Matrix {n: n - 1, m: m - 1, elements}
    }

    fn sum_of_products(n: usize, vals1: &Vec<T>, vals2: &Vec<T>) -> T {
        let mut ret = vals1[0] * vals2[0];

        for i in 1..n {
            ret = ret + vals1[i] * vals2[i];
        }

        ret
    }
}

// ============================================================================

#[cfg(test)]
mod tests {
    use crate::transform::Matrix;

    fn assert_matrix_float_eq(a: &Matrix<f64>, b: &Matrix<f64>) {
        let (na, ma) = a.size();
        let (nb, mb) = b.size();

        assert_eq!(na, nb);
        assert_eq!(ma, mb);

        for r in 0..na {
            for c in 0..ma {
                assert_float_absolute_eq!(a.get(r, c), b.get(r, c));
            }
        }
    }

    #[test]
    fn matrix_creation() {
        let mut m = Matrix::new(4, 1, 0.0);

        let (n_rows, n_cols) = m.size();
        assert_eq!(n_rows, 4);
        assert_eq!(n_cols, 1);

        m.set(1, 0, 42.0);

        for row in 0..n_rows {
            for col in 0..n_cols {
                let exp = if row == 1 && col == 0 { 42.0 } else { 0.0 };
                assert_float_absolute_eq!(exp, m.get(row, col));
            }
        }
    }

    #[test]
    fn matrix_eqality() {
        let m1 = Matrix::from_elements(&vec![
            vec![1, 2],
            vec![3, 4]
        ]);

        let m2 = Matrix::from_elements(&vec![
            vec![1, 2],
            vec![3, 4]
        ]);

        let m3 = Matrix::from_elements(&vec![
            vec![4, 3],
            vec![2, 1]
        ]);

        assert_eq!(m1, m2);
        assert_ne!(m1, m3);
    }

    #[test]
    fn matrix_scale() {
        let m = Matrix::from_elements(&vec![
            vec![1, 2],
            vec![3, 4]
        ]);

        let exp = Matrix::from_elements(&vec![
            vec![2, 4],
            vec![6, 8]
        ]);

        assert_eq!(exp, m.scale(2));
    }

    #[test]
    fn matrix_multiplication() {
        let a = Matrix::from_elements(&vec![
            vec![1, 2, 3],
            vec![3, 4, 5]
        ]);

        let b = Matrix::from_elements(&vec![
            vec![1, 2],
            vec![3, 4],
            vec![5, 6]
        ]);

        let exp = Matrix::from_elements(&vec![
            vec![22, 28],
            vec![40, 52]
        ]);

        assert_eq!(exp, a.multiply(&b).unwrap());
    }

    #[test]
    fn matrix_transpose() {
        let m = Matrix::from_elements(&vec![
            vec![0, 9, 3, 0],
            vec![9, 8, 0, 8],
            vec![1, 8, 5, 3],
            vec![0, 0, 5, 8],
            vec![1, 2, 3, 4],
        ]);

        let exp = Matrix::from_elements(&vec![
            vec![0, 9, 1, 0, 1],
            vec![9, 8, 8, 0, 2],
            vec![3, 0, 5, 5, 3],
            vec![0, 8, 3, 8, 4],
        ]);

        assert_eq!(exp, m.transpose());
    }

    #[test]
    fn sub_matrix() {
        let m = Matrix::from_elements(&vec![
            vec![1, 2, 3],
            vec![4, 5, 6],
            vec![7, 8, 9],
        ]);

        let exp = Matrix::from_elements(&vec![
            vec![1, 3],
            vec![7, 9],
        ]);

        assert_eq!(exp, m.sub_matrix(1, 1));
    }

    #[test]
    fn matrix_determinant() {
        let m = Matrix::from_elements(&vec![
            vec![1, 2, 6],
            vec![-5, 8, -4],
            vec![2, 6, 4],
        ]);

        let exp = -196;

        assert_eq!(exp, m.determinant().unwrap());

        let m = Matrix::from_elements(&vec![
            vec![-2, -8, 3, 5],
            vec![-3, 1, 7, 3],
            vec![1, 2, -9, 6],
            vec![-6, 7, 7, -9],
        ]);

        let exp = -4071;

        assert_eq!(exp, m.determinant().unwrap());
    }

    #[test]
    fn matrix_inverse() {
        let m = Matrix::from_elements(&vec![
            vec![1.0, 2.0, 3.0],
            vec![4.0, -5.0, 6.0],
            vec![7.0, 8.0, -10.0],
        ]);

        let exp = Matrix::from_elements(&vec![
            vec![1.0, 0.0, 0.0],
            vec![0.0, 1.0, 0.0],
            vec![0.0, 0.0, 1.0],
        ]);

        let m_inv = m.invert().unwrap();

        assert_matrix_float_eq(&exp, &m.multiply(&m_inv).unwrap());
        assert_matrix_float_eq(&exp, &m_inv.multiply(&m).unwrap());

        let m_inv_inv = m_inv.invert().unwrap();

        assert_matrix_float_eq(&m, &m_inv_inv);
    }
}
