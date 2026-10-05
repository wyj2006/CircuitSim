pub mod matrix;

use crate::solver::matrix::Matrix;

pub fn lu_decomposition(a: &Matrix) -> (Matrix, Matrix, Matrix) {
    let (rows, cols) = (a.rows, a.cols);
    let mut l = Matrix::zeros(rows, cols);
    let mut u = a.clone();
    let mut p = Matrix::zeros(rows, cols);

    for i in 0..rows {
        p[(i, i)] = 1.0;
    }

    for k in 0..cols {
        let mut pivot = k;

        for i in k..rows {
            if u[(i, k)].abs() > u[(pivot, k)] {
                pivot = i;
            }
        }

        if pivot != k {
            for j in 0..cols {
                p.swap((pivot, j), (k, j));
                l.swap((pivot, j), (k, j));
                u.swap((pivot, j), (k, j));
            }
        }

        l[(k, k)] = 1.0;

        for i in k + 1..rows {
            let m = u[(i, k)] / u[(k, k)];
            l[(i, k)] = m;
            for j in k..cols {
                let t = u[(k, j)] * m;
                if t == 0.0 {
                    continue;
                }
                u[(i, j)] -= t;
            }
        }
    }

    (p, l, u)
}

pub fn solve(coef: &Matrix, rhs: &Matrix) -> Matrix {
    let (rows, cols) = (coef.rows, coef.cols);
    let mut x = Matrix::zeros(rows, 1);
    let mut y = Matrix::zeros(rows, 1);

    let (p, l, u) = lu_decomposition(coef);
    let b = &p * &rhs;

    for i in 0..rows {
        let mut t = b[(i, 0)];
        for j in 0..i {
            t -= y[(j, 0)] * l[(i, j)];
        }
        y[(i, 0)] = t / l[(i, i)];
    }

    for i in (0..rows).rev() {
        let mut t = y[(i, 0)];
        for j in i + 1..cols {
            t -= x[(j, 0)] * u[(i, j)];
        }
        x[(i, 0)] = t / u[(i, i)];
    }

    x
}
