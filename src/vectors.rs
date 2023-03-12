// This file is part of "linbra"
// Under the MIT License
// Copyright (c) 2023 Antonin Hérault

//! Fixed-size vector and easy-types for different usually used vectors with 
//! into/from implementations on relevant primitives types.

use std::ops;

/// Vector with a fixed-length of 2.
pub type Vector2<T> = Vector<T, 2>;
/// Vector with a fixed-length of 3.
pub type Vector3<T> = Vector<T, 3>;
/// Vector with a fixed-length of 4.
pub type Vector4<T> = Vector<T, 4>;

/// Creates a vector 2 from a tuple of two values.
/// 
/// ## Example
/// ```
/// use linbra::vectors::Vector2;
/// 
/// let vec2: Vector2<u8> = (8, 9).into();
/// assert_eq!(vec2, Vector2::<u8>::new([8, 9]));
/// ```
impl<T> From<(T, T)> for Vector2<T> {
    fn from(value: (T, T)) -> Self {
        Self::new([value.0, value.1])
    }
}

/// Creates a vector 3 from a tuple of three values.
/// 
/// ## Example
/// ```
/// use linbra::vectors::Vector3;
/// 
/// let vec3: Vector3<u8> = (8, 9, 10).into();
/// assert_eq!(vec3, Vector3::<u8>::new([8, 9, 10]));
/// ```
impl<T> From<(T, T, T)> for Vector3<T> {
    fn from(value: (T, T, T)) -> Self {
        Self::new([value.0, value.1, value.2])
    }
}

/// Creates a vector 4 from a tuple of four values.
/// 
/// ## Example
/// ```
/// use linbra::vectors::Vector4;
/// 
/// let vec4: Vector4<u8> = (8, 9, 10, 11).into();
/// assert_eq!(vec4, Vector4::<u8>::new([8, 9, 10, 11]));
/// ```
impl<T> From<(T, T, T, T)> for Vector4<T> {
    fn from(value: (T, T, T, T)) -> Self {
        Self::new([value.0, value.1, value.2, value.3])
    }
}

/// Has a fixed size defined in the generic arguments as `N`.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Vector<T, const N: usize> {
    /// Array of data contained by the vector.
    data: [T; N]
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

/// Implements the index operator for vectors.
/// 
/// ## Usage
/// ```
/// use linbra::vectors::{ Vector, Vector3 };
/// 
/// let colour: Vector3<u8> = Vector::new([255, 100, 100]);
/// let red = colour[0];
/// assert_eq!(red, 255);
/// ```
/// This project provides a way to manage [`colours`](crate::colours).
impl<T, const N: usize> ops::Index<usize> for Vector<T, N> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl<T, const N: usize> Vector<T, N> {
    /// Creates a new vector.
    pub fn new(data: [T; N]) -> Self {
        Self {
            data,
        }
    }
}
