// This file is part of "linbra"
// Under the MIT License
// Copyright (c) 2023 Antonin Hérault

//! Implementations for operators which relate to different types. For example: 
//! the matrix-vector products.
//! 
//! The following operations are implemented:
//! - matrix-vector product (matrix * vector)

use std::ops::Mul;

use crate::{vector::Vector, Zero, Num, matrix::Matrix};

/// Implementation for matrix-vector product.
/// 
/// ## Formula
/// $$ 
/// \begin{pmatrix} 
///     a_{1,1} & a_{1,2} & \dots & a_{1,n} \\\ 
///     a_{2,1} & a_{2,2} & \dots & a_{2,n} \\\ 
///     \vdots & \vdots & \ddots & \vdots \\\ 
///     a_{m,1} & a_{m,2} & \dots & a_{m,n} \\\ 
/// \end{pmatrix}
/// \times
/// \begin{pmatrix} 
///     x_{1} \\\ 
///     x_{2} \\\ 
///     \vdots \\\ 
///     x_{n} \\\ 
/// \end{pmatrix} =
/// \begin{pmatrix} 
///     a_{1,1} \times x_{1} + a_{1,2} \times x_{2} + \dots + a_{1,n} \times x_{n} \\\ 
///     a_{2,1} \times x_{1} + a_{2,2} \times x_{2} + \dots + a_{1,n} \times x_{n} \\\ 
///     \vdots \\\ 
///     a_{m,1} \times x_{1} + a_{m,2} \times x_{2} + \dots + a_{m,n} \times x_{n} \\\ 
/// \end{pmatrix} =
/// \begin{pmatrix}
///     c_{1} \\\ 
///     c_{2} \\\ 
///     \vdots \\\ 
///     c_{m} \\\ 
/// \end{pmatrix}
/// $$
/// 
/// ## Example
/// $$ 
/// \begin{pmatrix} 
///     10 & 15 & 18 \\\ 
///     20 & 25 & 28 \\\ 
///     30 & 35 & 38 \\\ 
///     40 & 45 & 48 \\\ 
/// \end{pmatrix}
/// \times
/// \begin{pmatrix} 
///     6 \\\ 
///     7 \\\ 
///     8 \\\ 
/// \end{pmatrix} =
/// \begin{pmatrix} 
///     10 \times 6 + 15 \times 7 + 18 \times 8 \\\ 
///     20 \times 6 + 25 \times 7 + 28 \times 8 \\\ 
///     30 \times 6 + 35 \times 7 + 38 \times 8 \\\ 
///     40 \times 6 + 45 \times 7 + 48 \times 8 \\\ 
/// \end{pmatrix} = 
/// \begin{pmatrix}
///     309 \\\ 
///     519 \\\ 
///     729 \\\ 
///     939 \\\ 
/// \end{pmatrix}
/// $$
/// 
/// ```
/// use linbra::matrix::Matrix;
/// use linbra::vector::Vector;
/// 
/// let vector = Vector::<i32, 3>::new([6, 7, 8]);
/// 
/// let matrix = Matrix::<i32, 3, 4>::natural([
///     [10, 15, 18],
///     [20, 25, 28],
///     [30, 35, 38],
///     [40, 45, 48],
/// ]);
/// 
/// assert_eq!(vector * matrix, Vector::<i32, 4>::new([309, 519, 729, 939]));
/// ```
impl<const M: usize, const N: usize, T: Zero + Num> Mul<Matrix<T, N, M>> for Vector<T, N> {
    type Output = Vector<T, M>;

    fn mul(self, rhs: Matrix<T, N, M>) -> Self::Output {
        let mut vector = Self::Output::zeroed();

        for n in 0..N {
            for m in 0..M {
                vector[m] += self[n] * rhs[n][m];
            }
        }

        vector
    }
}
