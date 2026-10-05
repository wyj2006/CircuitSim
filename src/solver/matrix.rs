use std::ops::{Index, IndexMut, Mul};

use indexmap::IndexMap;

#[derive(Debug, Clone)]
pub struct Matrix {
    pub data: IndexMap<(usize, usize), f64>,
    pub rows: usize,
    pub cols: usize,
}

impl Matrix {
    pub fn zeros(rows: usize, cols: usize) -> Matrix {
        Matrix {
            data: IndexMap::new(),
            rows,
            cols,
        }
    }

    pub fn to_vec(&self) -> Vec<Vec<f64>> {
        let mut a = vec![vec![0.0; self.cols]; self.rows];
        for ((i, j), v) in &self.data {
            a[*i][*j] = *v;
        }
        a
    }

    pub fn swap(&mut self, a: (usize, usize), b: (usize, usize)) {
        if self[a] == 0.0 && self[b] == 0.0 {
            return;
        }
        let t = self[a];
        self[a] = self[b];
        self[b] = t;
    }
}

impl Index<(usize, usize)> for Matrix {
    type Output = f64;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        if index.0 >= self.rows || index.1 >= self.cols {
            panic!("index out of bounds")
        }
        self.data.get(&index).unwrap_or(&0.0)
    }
}

impl IndexMut<(usize, usize)> for Matrix {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        if index.0 >= self.rows || index.1 >= self.cols {
            panic!("index out of bounds")
        }
        if !self.data.contains_key(&index) {
            self.data.insert(index, 0.0);
        }
        self.data.get_mut(&index).unwrap()
    }
}

impl<const N: usize> From<(usize, usize, [((usize, usize), f64); N])> for Matrix {
    fn from(value: (usize, usize, [((usize, usize), f64); N])) -> Self {
        let mut a = Matrix::zeros(value.0, value.1);
        for (index, value) in value.2 {
            a[index] = value;
        }
        a
    }
}

impl<const N: usize, const M: usize> From<[[f64; M]; N]> for Matrix {
    fn from(value: [[f64; M]; N]) -> Self {
        let mut a = Matrix::zeros(N, M);
        for i in 0..N {
            for j in 0..M {
                a[(i, j)] = value[i][j];
            }
        }
        a
    }
}

impl Mul<&Matrix> for &Matrix {
    type Output = Matrix;

    fn mul(self, rhs: &Matrix) -> Self::Output {
        let (n, m) = (self.rows, self.cols);
        let (_, s) = (rhs.rows, rhs.cols);
        let mut x = Matrix::zeros(n, s);

        for i in 0..n {
            for j in 0..s {
                for k in 0..m {
                    x[(i, j)] += self[(i, k)] * rhs[(k, j)];
                }
            }
        }

        x
    }
}
