use std::ops::{Add, Mul};

#[derive(PartialEq, Debug)]
pub struct Matrix<T: Copy + Add<Output = T> + Mul<Output = T>> {
    n: usize,
    m: usize,
    elements: Vec<Vec<T>>,
}

impl<T: Copy + Add<Output = T> + Mul<Output = T>> Matrix<T> {
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
}
