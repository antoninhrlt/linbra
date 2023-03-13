// This file is part of "linbra"
// Under the MIT License
// Copyright (c) 2023 Antonin Hérault

//! Matrix types and functions to perform calculations on matrices.

use std::ops;

/// Matrix with a fixed-length of 2x2.
pub type Matrix2<T> = Matrix<T, 2, 2>;
/// Matrix with a fixed-length of 3x3.
pub type Matrix3<T> = Matrix<T, 3, 3>;
/// Matrix with a fixed-length of 4x4.
pub type Matrix4<T> = Matrix<T, 4, 4>;

/// Linear algebra mathematical tool used for transformations.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Matrix<T, const X: usize, const Y: usize> {
    /// Two-dimensional array of the data contained by the matrix.
    data: [[T; X]; Y],
}

/// Returns the row at index `n` in the matrix.
/// 
/// ## Example
/// ```
/// use linbra::matrices::{ Matrix, Matrix4 };
/// 
/// let matrix4 = Matrix4::<f32>::new([
///     [1.0, 0.0, 1.0, 0.0],
///     [0.0, 1.0, 0.0, 1.0],
///     [1.0, 0.0, 0.0, 1.0],
///     [0.0, 1.0, 1.0, 0.0],
/// ]);
/// 
/// let row1 = matrix4[0];
/// assert_eq!(row1, [1.0, 0.0, 1.0, 0.0]);
/// ```
impl<T, const X: usize, const Y: usize> ops::Index<usize> for Matrix<T, X, Y> {
    type Output = [T; X];

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

/// Returns the row at index `n` in the matrix, as mutable.
/// 
/// ## Example
/// ```
/// use linbra::matrices::{ Matrix, Matrix4 };
/// 
/// let mut matrix4 = Matrix4::<f32>::new([
///     [1.0, 0.0, 1.0, 0.0],
///     [0.0, 1.0, 0.0, 1.0],
///     [1.0, 0.0, 0.0, 1.0],
///     [0.0, 1.0, 1.0, 0.0],
/// ]);
/// 
/// matrix4[0] = [1.0, 1.0, 1.0, 1.0];
/// assert_eq!(matrix4[0], [1.0, 1.0, 1.0, 1.0]);
/// ```
impl<T, const X: usize, const Y: usize> ops::IndexMut<usize> for Matrix<T, X, Y> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}

impl<T, const X: usize, const Y: usize> Matrix<T, X, Y> {
    /// Creates a new matrix.
    pub fn new(data: [[T; X]; Y]) -> Self {
        Self { data }
    }
}
