// This file is part of "linbra"
// Under the MIT License
// Copyright (c) 2023 Antonin Hérault

use std::{ops, array::IntoIter};

use crate::Zero;

/// Linear algebra mathematical tool.
/// 
/// The `N` const generic parameter is used to define the number of values for 
/// the vector.
/// 
/// $$
/// \begin{pmatrix} 
///     a_{1} \\\ 
///     a_{2} \\\ 
///     \vdots \\\ 
///     a_{n} \\\ 
/// \end{pmatrix}
/// $$
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Vector<T, const N: usize> {
    /// Array of data contained by the vector.
    data: [T; N]
}

impl<T, const N: usize> Vector<T, N> {
    /// Creates a new vector.
    pub fn new(data: [T; N]) -> Self {
        Self { data }
    }
}

/// Creates a vector `N` from an array of `N` values.
/// 
/// ## Example
/// ```
/// use linbra::vectors::Vector;
/// 
/// let vec: Vector<u8, 6> = [8, 9, 10, 11, 12, 13].into();
/// assert_eq!(vec, Vector::<u8, 6>::new([8, 9, 10, 11, 12, 13]));
/// ```
impl<T, const N: usize> From<[T; N]> for Vector<T, N> {
    fn from(value: [T; N]) -> Self {
        Self {
            data: value,
        }
    }
}

/// Returns the value at index `n` in the vector.
/// 
/// ## Usage
/// ```
/// use linbra::vectors::{ Vector, Vector3 };
/// 
/// let colour: Vector3<u8> = Vector::new([255, 100, 100]);
/// let red = colour[0];
/// assert_eq!(red, 255);
/// ```
/// Note this project provides a way to manage [`colours`](crate::colours).
impl<T, const N: usize> ops::Index<usize> for Vector<T, N> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

/// Returns the value at index `n` in the vector, as mutable.
/// 
/// ## Usage
/// ```
/// use linbra::vectors::{ Vector, Vector3 };
/// 
/// let mut colour: Vector3<u8> = Vector::new([255, 100, 100]);
/// colour[0] = 100;
/// assert_eq!(colour[0], 100);
/// ```
/// Note this project provides a way to manage [`colours`](crate::colours).
impl<T, const N: usize> ops::IndexMut<usize> for Vector<T, N> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}


/// Implementations iteration on the vector by converting its data array into 
/// an iterator.
impl<T, const N: usize> IntoIterator for Vector<T, N> {
    type Item = T;
    type IntoIter = IntoIter<T, N>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter()
    }
}

/// Implements a constructor filling the vector with zeros for types 
/// implementing the [`Zero`] trait.
/// 
/// All number-primitive types implement [`Zero`].
impl<T: Zero, const N: usize> Vector<T, N> {
    /// Creates a new vector filled with zeros.
    pub fn zeroed() -> Self {
        Self {
            data: [T::zero(); N]
        }
    }
}
