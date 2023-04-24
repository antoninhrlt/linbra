// This file is part of "linbra"
// Under the MIT License
// Copyright (c) 2023 Antonin Hérault

//! The matrix structure and associated functions.

use std::ops;

use crate::Zero;

/// Linear algebra mathematical tool used for transformations for example.
/// 
/// The `C` and `R` const generic parameters are used to define the number of 
/// columns and rows for the matrix.
/// 
/// $$
/// \begin{pmatrix} 
///     x_{1,1} & x_{1,2} & \dots & x_{1,C} \\\ 
///     x_{2,1} & x_{2,2} & \dots & x_{2,C} \\\ 
///     \vdots & \vdots & \ddots & \vdots \\\ 
///     x_{R,1} & x_{R,2} & \dots & x_{R,C} \\\ 
/// \end{pmatrix}
/// $$
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Matrix<T, const C: usize, const R: usize> {
    data: [[T; R]; C]
}

impl<T: Zero, const C: usize, const R: usize> Matrix<T, C, R> {
    /// Creates a new matrix from a natural order. 
    /// 
    /// Indeed, it is programmatically in the wrong order but visually in the 
    /// right order. To create a matrix from the programmatically order, see 
    /// [`Matrix::new`].
    /// 
    /// ## Example
    /// $$
    /// \begin{pmatrix} 
    ///     10 & 20 & 30 & 40 \\\ 
    ///     15 & 25 & 35 & 45 \\\ 
    ///     17 & 27 & 37 & 47 \\\ 
    /// \end{pmatrix}
    /// $$
    /// ```
    /// use linbra::matrix::{ Matrix, Matrix4 };
    /// 
    /// let mut matrix = Matrix::<i32, 4, 3>::natural([
    ///     [10, 20, 30, 40],
    ///     [15, 25, 35, 45],
    ///     [17, 27, 37, 47],
    /// ]);
    /// ```
    pub fn natural(data: [[T; C]; R]) -> Self {
        let mut reversed: [[T; R]; C] = [[T::zero(); R]; C];

        for column in 0..C {
            for row in 0..R {
                reversed[column][row] = data[row][column];
            }
        }

        Self { data: reversed }
    }

    /// Creates a new matrix. 
    /// 
    /// It is programmatically in the right order but visually in the wrong 
    /// order. To create a matrix from the natural order, see 
    /// [`Matrix::natural`].
    /// 
    /// ## Example
    /// $$
    /// \begin{pmatrix} 
    ///     10 & 20 & 30 & 40 \\\ 
    ///     15 & 25 & 35 & 45 \\\ 
    ///     17 & 27 & 37 & 47 \\\ 
    /// \end{pmatrix}
    /// $$
    /// ```
    /// use linbra::matrix::{ Matrix, Matrix4 };
    /// 
    /// let mut matrix = Matrix::<i32, 4, 3>::new([
    ///     [10, 15, 17],
    ///     [20, 25, 27],
    ///     [30, 35, 37],
    ///     [40, 45, 47]
    /// ]);
    /// ```
    pub fn new(data: [[T; R]; C]) -> Self {
        Self { data }
    }
}

/// Returns the row at index `n` in the matrix.
/// 
/// ## Example
/// ```
/// use linbra::matrix::{ Matrix, Matrix4 };
/// 
/// let matrix4 = Matrix4::<f32>::natural([
///     [1.0, 1.0, 1.0, 0.0],
///     [0.0, 1.0, 0.0, 1.0],
///     [1.0, 0.0, 0.0, 1.0],
///     [0.0, 1.0, 1.0, 0.0],
/// ]);
/// 
/// let row1 = matrix4[0];
/// assert_eq!(row1, [1.0, 0.0, 1.0, 0.0]);
/// ```
impl<T, const C: usize, const R: usize> ops::Index<usize> for Matrix<T, C, R> {
    type Output = [T; R];

    fn index(&self, row: usize) -> &Self::Output {
        &self.data[row]
    }
}

/// Returns the row at index `n` in the matrix, as mutable.
/// 
/// ## Example
/// ```
/// use linbra::matrix::{ Matrix, Matrix4 };
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
impl<T, const C: usize, const R: usize> ops::IndexMut<usize> for Matrix<T, C, R> {
    fn index_mut(&mut self, row: usize) -> &mut Self::Output {
        &mut self.data[row]
    }
}
