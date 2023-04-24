// This file is part of "linbra"
// Under the MIT License
// Copyright (c) 2023 Antonin Hérault

//! Fixed-size vector and easy-types for different usually used vectors with 
//! into/from implementations on relevant primitives types.

mod operations;
mod vector;

pub use operations::*;
pub use vector::*;

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
/// use linbra::vector::Vector2;
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
/// use linbra::vector::Vector3;
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
/// use linbra::vector::Vector4;
/// 
/// let vec4: Vector4<u8> = (8, 9, 10, 11).into();
/// assert_eq!(vec4, Vector4::<u8>::new([8, 9, 10, 11]));
/// ```
impl<T> From<(T, T, T, T)> for Vector4<T> {
    fn from(value: (T, T, T, T)) -> Self {
        Self::new([value.0, value.1, value.2, value.3])
    }
}
