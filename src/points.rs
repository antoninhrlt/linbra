// This file is part of "linbra"
// Under the MIT License
// Copyright (c) 2023 Antonin Hérault

//! Traits to get x, y and z values of a point 2 or a point 3, and constructors 
//! for points.

use std::ops;

use crate::vectors::{Vector2, Vector3};

/// Implements functions to retrieve vertical and horizontal positions of a 
/// point in a 2D plan.
pub trait Point2<T: Copy>: ops::Index<usize, Output = T> {
    /// Returns the value on the x-axis of the point.
    fn x(&self) -> T {
        self[0]
    }

    /// Returns the value on the y-axis of the point.
    fn y(&self) -> T {
        self[1]
    }
}

/// Implements the [`Point2`] trait for vectors 2.
impl<T: Copy> Point2<T> for Vector2<T> {}

/// Implements the [`Point3`] trait for vectors 3.
impl<T: Copy> Point2<T> for Vector3<T> {}

/// Implements a constructor for points 2.
/// 
/// ## Example
/// ```
/// use linbra::{
///     points::Point2,
///     vectors::Vector2
/// };
/// 
/// let point = Vector2::at(10, 5);
/// 
/// assert_eq!(point.x(), 10);
/// assert_eq!(point.y(), 5);
/// ```
impl<T: Copy> Vector2<T> {
    /// Creates a new point on a 2D plan.
    pub fn at(x: T, y: T) -> Self {
        [x, y].into()
    }
}

/// Implements a function to retrieve the depth of a point in a 3D plan.
pub trait Point3<T: Copy>: Point2<T> {
    /// Returns the value on the z-axis of the point.
    fn z(&self) -> T {
        self[2]
    }
}

/// Implements the [`Point3`] trait for vectors 2.
impl<T: Copy> Point3<T> for Vector3<T> {}

/// Implements a constructor for points 3.
/// 
/// ## Example
/// ```
/// use linbra::{
///     points::{ Point2, Point3 },
///     vectors::Vector3
/// };
/// 
/// let point = Vector3::at(10, 5, 2);
/// 
/// assert_eq!(point.x(), 10);
/// assert_eq!(point.y(), 5);
/// assert_eq!(point.z(), 2);
/// ```
impl<T: Copy> Vector3<T> {
    /// Creates a new point on a 3D plan.
    pub fn at(x: T, y: T, z: T) -> Self {
        [x, y, z].into()
    }
}
