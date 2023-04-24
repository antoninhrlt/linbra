// This file is part of "linbra"
// Under the MIT License
// Copyright (c) 2023 Antonin Hérault

//! Traits to retrieve the red, blue, green (and alpha) channels of colour 
//! vectors and the implements of these traits for vectors 3 and 4.

use std::ops;
use crate::vector::{Vector3, Vector4};

/// Implements functions to retrieve the red, blue and green channels of a 
/// colour
pub trait RGB: ops::Index<usize, Output = u8> {
    /// Returns the red channel being the first element of the data structure.
    fn r(&self) -> u8 {
        self[0]
    }

    /// Returns the green channel being the second element of the data 
    /// structure.
    fn g(&self) -> u8 {
        self[1]
    }

    /// Returns the blue channel being the third element of the data structure.
    fn b(&self) -> u8 {
        self[2]
    }
}

/// Implements the [`RGB`] trait for 3-vectors of `u8` values.  
impl RGB for Vector3<u8> {}

/// Implements the [`RGB`] trait for 4-vectors of `u8` values to let the 
/// implementation of the [`RGBA`] trait possible.
impl RGB for Vector4<u8> {}

/// Implements a named constructor for RGB structures.
impl Vector3<u8> {
    fn rgb(r: u8, g: u8, b: u8) -> Self {
        [r, g, b].into()
    }
}

/// Creates a 3-vector of `u8` from an hexadecimal value as `#RRGGBB`.
/// 
/// ## Example
/// ```
/// use linbra::{
///     vector::Vector3,
///     colours::RGB,
/// };
/// 
/// let hex: u32 = 0xFF0000;
/// let rgb: Vector3<u8> = hex.into();
/// 
/// assert_eq!(255, rgb.r());
/// ```
impl From<u32> for Vector3<u8> {
    fn from(value: u32) -> Self { 
        // Retrieves the RR byte of the hexadecimal value.
        let r = (value >> 16) & 0xFF;
        // Retrieves the GG byte of the hexadecimal value.
        let g = (value >> 8) & 0xFF;
        // Retrieves the BB byte of the hexadecimal value.
        let b = (value) & 0xFF;
        
        Self::rgb(r as u8, g as u8, b as u8)
    }
}

/// Creates an hexadecimal value as `#RRGGBB` from a 3-vector of `u8`.
/// 
/// ## Example
/// ```
/// use linbra::{
///     vector::Vector3,
///     colours::RGB,
/// };
/// 
/// let rgb: Vector3<u8> = [255, 0, 0].into();
/// let hex: u32 = rgb.into();
/// 
/// assert_eq!(0xFF0000, hex);
/// ```
impl Into<u32> for Vector3<u8> {
    fn into(self) -> u32 {
        ((self[0] as u32) << 16) | ((self[1] as u32) << 8) | (self[2] as u32)
    }
}

/// Implements a function to retrieve the alpha channel of a colour more than
/// the red, blue and green channels.
pub trait RGBA: RGB {
    /// Returns the alpha channel being the third element of the data structure.
    fn a(&self) -> u8 {
        self[3]
    }
}

/// Implements the [`RGBA`] for 4-vectors of `u8` values. 
/// 
/// The [`RGB`] trait is also implemented for 4-vectors in this module. 
impl RGBA for Vector4<u8> {}

/// Implements a named constructor for RGBA structures.
impl Vector4<u8> {
    fn rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        [r, g, b, a].into()
    }
}

/// Creates a 4-vector of `u8` from an hexadecimal value like `#RRGGBBAA`.
/// 
/// ## Example
/// ```
/// use linbra::{
///     vector::Vector4,
///     colours::{ RGB, RGBA },
/// };
/// 
/// let hex: u32 = 0xFF0000FF;
/// let rgba: Vector4<u8> = hex.into();
/// 
/// assert_eq!(255, rgba.r());
/// assert_eq!(255, rgba.a());
/// ```
impl From<u32> for Vector4<u8> {
    fn from(value: u32) -> Self { 
        // Retrieves the RR byte of the hexadecimal value.
        let r = (value >> 24) & 0xFF;
        // Retrieves the GG byte of the hexadecimal value.
        let g = (value >> 16) & 0xFF;
        // Retrieves the BB byte of the hexadecimal value.
        let b = (value >> 8) & 0xFF;
        // Retrieves the AA byte of the hexadecimal value.
        let a =  (value) & 0xFF;
        
        Self::rgba(r as u8, g as u8, b as u8, a as u8)
    }
}

/// Creates an hexadecimal value as `#RRGGBBAA` from a 4-vector of `u8`.
/// 
/// ## Example
/// ```
/// use linbra::{
///     vector::Vector4,
///     colours::RGBA,
/// };
/// 
/// let rgba: Vector4<u8> = [255, 0, 0, 255].into();
/// let hex: u32 = rgba.into();
/// 
/// assert_eq!(0xFF0000FF, hex);
/// ```
impl Into<u32> for Vector4<u8> {
    fn into(self) -> u32 {
        ((self[0] as u32) << 24) | 
        ((self[1] as u32) << 16) | 
        ((self[2] as u32) << 8) | 
        (self[3] as u32)
    }
}
