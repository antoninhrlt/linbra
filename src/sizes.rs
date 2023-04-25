// This file is part of "linbra"
// Under the MIT License
// Copyright (c) 2023 Antonin Hérault

//! Traits to get width, height and depths values of a 2d-objects or 3d-objects, 
//! and constructors for these sizes.

use std::ops;
use crate::vector::{Vector2, Vector3};

/// Implements functions to retrieve the width and the height of the size 
/// vector.
pub trait Size2<T: Copy>: ops::Index<usize, Output = T> {
    /// Returns the width.
    fn w(&self) -> T {
        self[0]
    }

    /// Returns the height.
    fn h(&self) -> T {
        self[1]
    }
}

/// Implements the [`Size2`] trait for vectors 2.
impl<T: Copy> Size2<T> for Vector2<T> {}

/// Implements the [`Size2`] trait for vectors 3.
impl<T: Copy> Size2<T> for Vector3<T> {}

/// Implements a constructor for 2d-sizes.
/// 
/// ## Example
/// ```
/// use linbra::{
///     sizes::Size2,
///     vector::Vector2
/// };
/// 
/// let size = Vector2::size(10, 5);
/// 
/// assert_eq!(size.w(), 10);
/// assert_eq!(size.h(), 5);
/// ```
impl<T: Copy> Vector2<T> {
    /// Creates a new 2d-size.
    pub fn size(w: T, h: T) -> Self {
        [w, h].into()
    }
}

/// Implements a function to retrieve the depth of the size vector.
pub trait Size3<T: Copy>: Size2<T> {
    /// Returns the depth.
    fn d(&self) -> T {
        self[1]
    }
}

/// Implements a constructor for 3d-sizes.
/// 
/// ## Example
/// ```
/// use linbra::{
///     sizes::{ Size2, Size3 },
///     vector::Vector2
/// };
/// 
/// let size = Vector3::size(10, 5, 2);
/// 
/// assert_eq!(size.w(), 10);
/// assert_eq!(size.h(), 5);
/// assert_eq!(size.d(), 2);
/// ```
impl<T: Copy> Vector3<T> {
    /// Creates a new 3d-size.
    pub fn size(w: T, h: T, d: T) -> Self {
        [w, h, d].into()
    }
}
