// This file is part of "linbra"
// Under the MIT License
// Copyright (c) 2023 Antonin Hérault

//! Easily do linear algebra in game development, graphics and other sort of 
//! calculations using vectors and matrices.
//! 
//! Every implementation, function or item is documented mathematically and 
//! programmatically. Browses the items and their functions to learn how to use 
//! them!
//! 
//! ## Overview
//! - Objects:
//! 
//! | Mathematics | Linbra | Related types |
//! | --- | --- | --- |
//! | $$ \begin{pmatrix} x_{1,1} & x_{1,2} & \dots & x_{1,C} \\\ x_{2,1} & x_{2,2} & \dots & x_{2,C} \\\ \vdots & \vdots & \ddots & \vdots \\\ x_{R,1} & x_{R,2} & \dots & x_{R,C} \\\ \end{pmatrix} $$ | [`Matrix<T, C, R>`](matrix::Matrix) | <ul><li>[`Matrix2<T>`](matrix::Matrix2)</li> <li>[`Matrix3<T>`](matrix::Matrix3)</li> <li>[`Matrix4<T>`](matrix::Matrix4)</li></ul> |
//! | | | |
//! | $$ \begin{pmatrix} a_{1} \\\ a_{2} \\\ \vdots \\\ a_{n} \\\ \end{pmatrix} $$ | [`Vector<T, N>`](vector::Vector) | <ul><li>[`Vector2<T>`](vector::Vector2)</li> <li>[`Vector3<T>`](vector::Vector3)</li> <li>[`Vector4<T>`](vector::Vector4)</li></ul> |
//! 
//! - Tools:
//! 
//! | Mathematics | Related traits | and their functions |
//! | --- | --- | --- |
//! | $$ \begin{pmatrix} x \\\ y \\\ \end{pmatrix} or \begin{pmatrix} x \\\ y \\\ z \\\ \end{pmatrix} $$ | <ul><li>[`Point2<T>`](points::Point2)</li><li>[`Point3<T>`](points::Point3)</li></ul> | <ul><li>[`::x()`](points::Point2::x)</li><li>[`::y()`](points::Point2::y)</li><li>[`::z()`](points::Point3::z)</li></ul> |
//! | | | |
//! | $$ \begin{pmatrix} w \\\ h \\\ \end{pmatrix} or \begin{pmatrix} w \\\ h \\\ d \\\ \end{pmatrix} $$ | <ul><li>[`Size2<T>`](sizes::Size2)</li><li>[`Size3<T>`](sizes::Size3)</li></ul> | <ul><li>[`::w()`](sizes::Size2::w)</li><li>[`::h()`](sizes::Size2::h)</li><li>[`::d()`](sizes::Size3::d)</li></ul> |
//! | | | |
//! | $$ \begin{pmatrix} r \\\ g \\\ b \\\ \end{pmatrix} or \begin{pmatrix} r \\\ g \\\ b \\\ a \\\ \end{pmatrix} $$ | <ul><li>[`RGB<T>`](colours::RGB)</li><li>[`RGBA<T>`](colours::RGBA)</li></ul> | <ul><li>[`::r()`](colours::RGB::r)</li><li>[`::g()`](colours::RGB::g)</li><li>[`::b()`](colours::RGB::b)</li><li>[`::a()`](colours::RGBA::a)</li></ul> |

#![warn(missing_docs)]
#![warn(missing_debug_implementations)]

use std::ops;

pub mod colours;
pub mod matrix;
mod operations;
pub mod points;
pub mod vector;
pub mod sizes;

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
