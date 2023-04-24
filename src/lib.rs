// This file is part of "linbra"
// Under the MIT License
// Copyright (c) 2023 Antonin Hérault

//! Easily do linear algebra in game development, graphics and other sort of 
//! calculations using vectors and matrices.

#![warn(missing_docs)]
#![warn(missing_debug_implementations)]

use std::ops;

pub mod colours;
pub mod matrix;
mod operations;
pub mod points;
pub mod vector;

/// Implements a function to get the zero-value of the type.
/// 
/// This trait is implemented for all the number-primitive types.
pub trait Zero: Copy {
    /// Returns a zero-value of this type.
    fn zero() -> Self;
}

/// Common properties to all the number-primitive types.
/// 
/// No function provided.
pub trait Num
where 
    Self: ops::Add<Output = Self> 
        + ops::Sub<Output = Self>
        + ops::Mul<Output = Self> 
        + ops::AddAssign
        + ops::SubAssign
        + ops::MulAssign
        + PartialEq 
        + Copy 
{}

macro_rules! impl_primitive_numbers {
    ($type:tt, $zero:literal) => {
        impl Zero for $type {
            fn zero() -> Self {
                $zero
            }
        }

        impl Num for $type {}
    };
}

impl_primitive_numbers!(i8, 0);
impl_primitive_numbers!(i16, 0);
impl_primitive_numbers!(i32, 0);
impl_primitive_numbers!(i64, 0);
impl_primitive_numbers!(i128, 0);

impl_primitive_numbers!(u8, 0);
impl_primitive_numbers!(u16, 0);
impl_primitive_numbers!(u32, 0);
impl_primitive_numbers!(u64, 0);
impl_primitive_numbers!(u128, 0);

impl_primitive_numbers!(isize, 0);
impl_primitive_numbers!(usize, 0);

impl_primitive_numbers!(f32, 0.0);
impl_primitive_numbers!(f64, 0.0);
