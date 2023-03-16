// This file is part of "linbra"
// Under the MIT License
// Copyright (c) 2023 Antonin Hérault

//! Implementations of mathematical operators on the different types of the 
//! project like multiplying a matrix by a vector or subtract two matrices.
use std::ops::{ Sub, Add };

use crate::{ vectors::Vector, Zero, Num };

/// Implementation to add two vectors.
/// 
/// ## Example
/// ```
/// use linbra::vectors::Vector3;
/// Mul
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

/// Implementation to subtract two vectors.
/// 
/// ## Example
/// ```
/// use linbra::vectors::Vector3;
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
