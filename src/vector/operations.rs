// This file is part of "linbra"
// Under the MIT License
// Copyright (c) 2023 Antonin Hérault

//! Implementations for operators only related to vectors together.
//! 
//! The following operations are implemented:
//! - scalar product (vector * x)
//! - vectorial product (vector1 * vector2)
//! - vectors addition (vector1 + vector 2)
//! - vectors subtraction (vector1 - vector 2)

use crate::{ Num, Zero };
use crate::vector::Vector;

use std::ops::{ Add, Sub, Mul, MulAssign };

/// Implementation for scalar product
/// 
/// ## Formula
/// $$ x \times
/// \begin{pmatrix} 
///     a_{1} \\\ 
///     a_{2} \\\ 
///     \vdots \\\ 
///     a_{n} \\\ 
/// \end{pmatrix} = 
/// \begin{pmatrix} 
///     a_{1} \times x \\\ 
///     a_{2} \times x \\\ 
///     \vdots \\\ 
///     a_{n} \times x \\\ 
/// \end{pmatrix}
/// $$
/// 
/// ## Example
/// 
/// $$ 2 \times
/// \begin{pmatrix} 
///     5 \\\ 
///     8 \\\ 
/// \end{pmatrix} = 
/// \begin{pmatrix} 
///     5 \times 2 \\\ 
///     8 \times 2 \\\ 
/// \end{pmatrix} = 
/// \begin{pmatrix} 
///     10 \\\ 
///     16 \\\ 
/// \end{pmatrix}
/// $$
/// 
/// ```
/// use linbra::vector::Vector2;
/// 
/// let v = Vector2::new([5, 8]);
/// assert_eq!(v * 2, Vector2::new([10, 16]));
/// ```
impl<T: Zero + Num + MulAssign<U>, U: Num, const N: usize> Mul<U> for Vector<T, N> {
    type Output = Self;

    fn mul(self, rhs: U) -> Self::Output {
        let mut vector = self;
        
        for n in 0..N {
            vector[n] *= rhs; 
        }

        vector
    }
}

/// Implementation for vectorial product.
/// 
/// ## Formula
/// $$ 
/// \begin{pmatrix} 
///     a_{1} \\\ 
///     a_{2} \\\ 
///     \vdots \\\ 
///     a_{n} \\\ 
/// \end{pmatrix} 
/// \times
/// \begin{pmatrix} 
///     b_{1} \\\ 
///     b_{2} \\\ 
///     \vdots \\\ 
///     b_{n} \\\ 
/// \end{pmatrix} = 
/// \begin{pmatrix} 
///     a_{1} \times b_{1} \\\ 
///     a_{2} \times b_{2} \\\ 
///     \vdots \\\ 
///     a_{n} \times b_{n} \\\ 
/// \end{pmatrix}
/// $$
/// 
/// ## Example
/// 
/// $$
/// \begin{pmatrix} 
///     2 \\\ 
///     3 \\\ 
/// \end{pmatrix} 
/// \times
/// \begin{pmatrix} 
///     5 \\\ 
///     8 \\\ 
/// \end{pmatrix} = 
/// \begin{pmatrix} 
///     2 \times 5 \\\ 
///     3 \times 8 \\\ 
/// \end{pmatrix} = 
/// \begin{pmatrix} 
///     10 \\\ 
///     24 \\\ 
/// \end{pmatrix}
/// $$
/// 
/// ```
/// use linbra::vector::Vector2;
/// 
/// let vecA = Vector2::new([2, 3]);
/// let vecB = Vector2::new([5, 8]);
///
/// assert_eq!(vecA * vecB, Vector2::new([10, 24]));
/// ```
impl<T: Zero + Num, const N: usize> Mul<Self> for Vector<T, N> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut vector = self;
        
        for n in 0..N {
            vector[n] *= rhs[n]; 
        }

        vector
    }
}

/// Implementation for vectors addition.
/// 
/// ## Formula
/// $$
/// \begin{pmatrix} 
///     a_{1} \\\ 
///     a_{2} \\\ 
///     \vdots \\\ 
///     a_{n} \\\ 
/// \end{pmatrix} +
/// \begin{pmatrix} 
///     b_{1} \\\ 
///     b_{2} \\\ 
///     \vdots \\\ 
///     b_{n} \\\ 
/// \end{pmatrix} = 
/// \begin{pmatrix} 
///     a_{1} + b_{1} \\\ 
///     a_{2} + b_{2} \\\ 
///     \vdots \\\ 
///     a_{n} + b_{n} \\\ 
/// \end{pmatrix}
/// $$
/// 
/// ## Example
/// 
/// $$
/// \begin{pmatrix} 
///     5 \\\ 
///     8 \\\ 
///     2 \\\ 
/// \end{pmatrix} + 
/// \begin{pmatrix} 
///     3 \\\ 
///     1 \\\ 
///     2 \\\ 
/// \end{pmatrix} = 
/// \begin{pmatrix} 
///     5 + 3 \\\ 
///     8 + 1 \\\ 
///     2 + 2 \\\ 
/// \end{pmatrix} = 
/// \begin{pmatrix} 
///     8 \\\ 
///     9 \\\ 
///     4 \\\ 
/// \end{pmatrix}
/// $$
/// 
/// ```
/// use linbra::vector::Vector3;
/// 
/// let vector1 = Vector3::new([5, 8, 2]);
/// let vector2 = Vector3::new([3, 1, 2]);
/// 
/// assert_eq!(vector1 + vector2, Vector3::new([8, 9, 4]));
/// ```
impl<T: Zero + Num, const N: usize> Add<Self> for Vector<T, N> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        // The returned vector.
        let mut output = self;
        
        // Adds each value of `rhs` to each value of `data`.
        for (i, value) in rhs.into_iter().enumerate() {
            output[i] += value;
        }

        output
    }
}

/// Implementation for vectors subtraction.
/// 
/// ## Formula
/// $$
/// \begin{pmatrix} 
///     a_{1} \\\ 
///     a_{2} \\\ 
///     \vdots \\\ 
///     a_{n} \\\ 
/// \end{pmatrix} -
/// \begin{pmatrix} 
///     b_{1} \\\ 
///     b_{2} \\\ 
///     \vdots \\\ 
///     b_{n} \\\ 
/// \end{pmatrix} = 
/// \begin{pmatrix} 
///     a_{1} - b_{1} \\\ 
///     a_{2} - b_{2} \\\ 
///     \vdots \\\ 
///     a_{n} - b_{n} \\\ 
/// \end{pmatrix}
/// $$
/// 
/// ## Example
/// 
/// $$
/// \begin{pmatrix} 
///     5 \\\ 
///     8 \\\ 
///     2 \\\ 
/// \end{pmatrix} -
/// \begin{pmatrix} 
///     3 \\\ 
///     1 \\\ 
///     2 \\\ 
/// \end{pmatrix} = 
/// \begin{pmatrix} 
///     5 - 3 \\\ 
///     8 - 1 \\\ 
///     2 - 2 \\\ 
/// \end{pmatrix} = 
/// \begin{pmatrix} 
///     2 \\\ 
///     7 \\\ 
///     0 \\\ 
/// \end{pmatrix}
/// $$
/// 
/// ## Example
/// ```
/// use linbra::vector::Vector3;
/// 
/// let vector1 = Vector3::new([5, 8, 2]);
/// let vector2 = Vector3::new([3, 1, 2]);
/// 
/// assert_eq!(vector1 - vector2, Vector3::new([2, 7, 0]));
/// ```
impl<T: Zero + Num, const N: usize> Sub<Self> for Vector<T, N> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        // The returned vector.
        let mut output = self;
        
        // Subtracts each value of `rhs` to each value of `data`.
        for (i, value) in rhs.into_iter().enumerate() {
            output[i] -= value;
        }

        output
    }
}
