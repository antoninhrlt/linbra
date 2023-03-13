// This file is part of "linbra"
// Under the MIT License
// Copyright (c) 2023 Antonin Hérault

//! Easily do linear algebra in game development, graphics and other sort of 
//! calculations using vectors and matrices.

#![warn(missing_docs)]
#![warn(missing_debug_implementations)]

use std::ops;

pub mod colours;
pub mod matrices;
pub mod points;
pub mod vectors;

/// Implements a function to get the zero-value of the type.
/// 
/// This trait is implemented for all the number-primitive types.
pub trait Zero: Copy {
    /// Returns a zero-value of this type.
    fn zero() -> Self;
}

macro_rules! impl_zero {
    ($type:tt, $zero:literal) => {
        impl Zero for $type {
            fn zero() -> Self {
                $zero
            }
        }
    };
}

impl_zero!(i8, 0);
impl_zero!(i16, 0);
impl_zero!(i32, 0);
impl_zero!(i64, 0);

impl_zero!(u8, 0);
impl_zero!(u16, 0);
impl_zero!(u32, 0);
impl_zero!(u64, 0);

impl_zero!(isize, 0);
impl_zero!(usize, 0);

impl_zero!(f32, 0.0);
impl_zero!(f64, 0.0);

/// Common properties to all the number-primitive types.
/// 
/// No function provided.
pub trait Num
where 
    Self: ops::Add<Output = Self> + ops::Sub<Output = Self> + PartialEq + Copy 
{}
