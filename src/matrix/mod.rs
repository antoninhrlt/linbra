// This file is part of "linbra"
// Under the MIT License
// Copyright (c) 2023 Antonin Hérault

//! Matrix types and functions to perform calculations on matrices.

mod matrix;
pub use matrix::*;

/// Matrix with a fixed-length of 2x2.
pub type Matrix2<T> = Matrix<T, 2, 2>;
/// Matrix with a fixed-length of 3x3.
pub type Matrix3<T> = Matrix<T, 3, 3>;
/// Matrix with a fixed-length of 4x4.
pub type Matrix4<T> = Matrix<T, 4, 4>;