use std::array;

// Matrix size: Rows x Columns
pub type Matrix<const R: usize, const C: usize> = [[f32; C]; R];

pub trait MatMul<const M: usize, const K: usize> {
    fn mat_mul<const N: usize>(
        &self,
        b: &Matrix<K, N>,
    ) -> Matrix<M, N>;
    fn comp_div(&self, rhs: &Self) -> Self;
    fn comp_mul(&self, rhs: &Self) -> Self;
    fn comp_sub(&self, rhs: &Self) -> Self;
    fn comp_abs(&self) -> Self;
    fn scaler_add(&self, rhs: f32) -> Self;
}

/// Multiplies a (M x K) matrix by a (K x N) matrix to produce a (M x N) matrix.

impl<const M: usize, const K: usize> MatMul<M,K> for Matrix<M, K> {
    fn mat_mul<const N: usize>(
        &self,
        b: &Matrix<K, N>,
    ) -> Matrix<M, N> {
        // Initialize result matrix with zeros
        let mut result: Matrix<M, N> = [[0.0; N]; M];

        for i in 0..M {
            for j in 0..N {
                for l in 0..K {
                    // Standard row-by-column multiplication
                    result[i][j] += self[i][l] * b[l][j];
                }
            }

        }
        result
    }
    
    fn comp_div(&self, rhs: &Self) -> Self {
       array::from_fn(
            |r|
            array::from_fn(
                |c|
                {
                    self[r][c] / rhs[r][c]
                }
            )
        )
    }
    
    fn scaler_add(&self, rhs: f32) -> Self {
        array::from_fn(
            |r|
            array::from_fn(
                |c|
                {
                    self[r][c] + rhs
                }
            )
        )
    }
    
    fn comp_mul(&self, rhs: &Self) -> Self {
       array::from_fn(
            |r|
            array::from_fn(
                |c|
                {
                    self[r][c] * rhs[r][c]
                }
            )
        )
    }
    
    fn comp_sub(&self, rhs: &Self) -> Self {
       array::from_fn(
            |r|
            array::from_fn(
                |c|
                {
                    self[r][c] - rhs[r][c]
                }
            )
        )
    }
    
    fn comp_abs(&self) -> Self {
       array::from_fn(
            |r|
            array::from_fn(
                |c|
                {
                    self[r][c].abs()
                }
            )
        )
    }
}