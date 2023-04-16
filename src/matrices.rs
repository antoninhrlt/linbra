// This file is part of "linbra"
// Under the MIT License
// Copyright (c) 2023 Antonin Hérault

//! Matrix types and functions to perform calculations on matrices.

use std::ops;

use crate::Zero;

/// Matrix with a fixed-length of 2x2.
pub type Matrix2<T> = Matrix<T, 2, 2>;
/// Matrix with a fixed-length of 3x3.
pub type Matrix3<T> = Matrix<T, 3, 3>;
/// Matrix with a fixed-length of 4x4.
pub type Matrix4<T> = Matrix<T, 4, 4>;

/// Linear algebra mathematical tool used for transformations.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Matrix<T, const R: usize, const C: usize> {
    data: [[T; R]; C]
}

/// Returns the row at index `n` in the matrix.
/// 
/// ## Example
/// ```
/// use linbra::matrices::{ Matrix, Matrix4 };
/// 
/// let matrix4 = Matrix4::<f32>::new([
///     [1.0, 1.0, 1.0, 0.0],
///     [0.0, 1.0, 0.0, 1.0],
///     [1.0, 0.0, 0.0, 1.0],
///     [0.0, 1.0, 1.0, 0.0],
/// ]);
/// 
/// let row1 = matrix4[0];
/// assert_eq!(row1, [1.0, 0.0, 1.0, 0.0]);
/// ```
impl<T, const R: usize, const C: usize> ops::Index<usize> for Matrix<T, R, C> {
    type Output = [T; R];

    fn index(&self, row: usize) -> &Self::Output {
        &self.data[row]
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
impl<T, const R: usize, const C: usize> ops::IndexMut<usize> for Matrix<T, R, C> {
    fn index_mut(&mut self, row: usize) -> &mut Self::Output {
        &mut self.data[row]
    }
}

impl<T: Zero, const R: usize, const C: usize> Matrix<T, R, C> {
    /// Creates a new matrix.
    /// 
    /// ## Example
    /// ```
    /// use linbra::matrices::{ Matrix, Matrix4 };
    /// 
    /// let mut matrix = Matrix::<f32, 3, 4>::new([
    ///     [1.0, 0.0, 1.0, 0.0],
    ///     [0.0, 1.0, 0.0, 1.0],
    ///     [1.0, 0.0, 0.0, 1.0],
    /// ]);
    /// ```
    pub fn new(data: [[T; C]; R]) -> Self {
        let mut reversed: [[T; R]; C] = [[T::zero(); R]; C];

        for column in 0..C {
            for row in 0..R {
                reversed[column][row] = data[row][column];
            }
        }

        Self { data: reversed }
    }
}
