// Copyright 2017-2018 Thomas de Zeeuw
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT
// or http://opensource.org/licenses/MIT>, at your option. This file may not be
// used, copied, modified, or distributed except according to those terms.

#![warn(missing_debug_implementations,
        missing_docs,
        trivial_casts,
        trivial_numeric_casts,
        unused_import_braces,
        unused_qualifications,
        unused_results,
)]

// TODO: conversion to and from difference `Multiple`s.

// TODO: implement serde.

//! The `human_size` represents sizes for humans.
//!
//! The main type is [`SpecificSize`], which (as the name might suggests)
//! represents a size in specific multiple. Alternatively [`Size`] can be used
//! to represent a size with a generic multiple (not defined at compile type).
//!
//! [`SpecificSize`]: struct.SpecificSize.html
//! [`Size`]: type.Size.html
//!
//! # Example
//!
//! Below is small example that parses a size from a string and prints it
//!
//! ```
//! # extern crate human_size;
//! # fn main() {
//! use human_size::{Size, SpecificSize, Kilobyte};
//!
//! let size1 = "10000 B".parse::<Size>().unwrap();
//! assert_eq!(size1.to_string(), "10000 B");
//!
//! // Or using a specific multiple.
//! let size2 = "10000 B".parse::<SpecificSize<Kilobyte>>().unwrap();
//! assert_eq!(size2.to_string(), "10 kB");
//!
//! // Generic and specific sizes can be compared.
//! assert_eq!(size1, size2);
//! # }
//! ```
//!
//! # Notes
//!
//! Internally `f64` is used to represent the size, so when comparing sizes with
//! different multiples be wary of rounding errors related to usage of floating
//! point numbers.

use std::fmt;
use std::cmp::Ordering;
use std::error::Error;
use std::str::FromStr;

pub mod multiples;

pub use multiples::*;

/// Size with a generic `Multiple`.
///
/// Note that the size of `Size` is 16 bytes, but using a specific multiple,
/// e.g. `SpecificSize<Byte>`, requires only 8 bytes.
pub type Size = SpecificSize<Any>;

/// `SpecificSize` represents a size in bytes with a multiple.
///
/// `SpecificSize` can be created using the `new` function, or parsed from a
/// string using the [`FromStr`] trait.
///
/// ```
/// # extern crate human_size;
/// # fn main() {
/// use human_size::{SpecificSize, Size, Byte, Any};
///
/// let size1 = SpecificSize::new(1000, Byte).unwrap();
/// assert_eq!(size1.to_string(), "1000 B");
///
/// // `Size` is a type alias for `SpecificSize<Any>`.
/// let size2: Size = "1 kB".parse().unwrap();
/// assert_eq!(size2.to_string(), "1 kB");
///
/// // Even though the multiples are different we can still compare them.
/// assert_eq!(size1, size2);
/// # }
/// ```
///
/// Creating a `SpecificSize` with a specific [`Multiple`], e.g. [`Kilobyte`],
/// only uses 8 bytes. Using the generic mulitple, i.e. [`Any`], it can
/// represent all multiples but requires an extra 8 bytes for a total of 16
/// bytes.
///
/// ```
/// # extern crate human_size;
/// # fn main() {
/// use std::mem;
///
/// use human_size::{SpecificSize, Size, Byte, Any};
///
/// assert_eq!(mem::size_of::<SpecificSize<Byte>>(), 8);
/// assert_eq!(mem::size_of::<Size>(), 16);
/// # }
/// ```
///
/// # Notes
///
/// When comparing sizes with one another it is possible compare different
/// multiples, see the first example above. However due to a lack of precision
/// in floating point numbers equality ignores a difference less then
/// `0.00000001`, after applying the multiple. See the `PartialEq`
/// implementation (via [src] to the right) for details.
///
/// [`FromStr`]: https://doc.rust-lang.org/nightly/core/str/trait.FromStr.html
/// [`Multiple`]: trait.Multiple.html
/// [`Kilobyte`]: multiples/struct.Kilobyte.html
/// [`Any`]: multiples/enum.Any.html
#[derive(Copy, Clone, Debug)]
pub struct SpecificSize<M = Any> {
    value: f64,
    multiple: M,
}

impl<M: Multiple> SpecificSize<M> {
    // TODO: change print statements to assertions.
    /// Create a new `SpecificSize` with the given value and multiple. If the
    /// `value` is [not normal] this will return an error, however zero is
    /// allowed. If the `value` is normal the result can be safely unwraped.
    ///
    /// ```
    /// # extern crate human_size;
    /// # fn main() {
    /// use std::f64;
    /// use human_size::{SpecificSize, Kilobyte};
    ///
    /// let size = SpecificSize::new(100, Kilobyte).unwrap();
    /// println!("size: {}", size); // 100 kB
    ///
    /// let size = SpecificSize::new(f64::NAN, Kilobyte);
    /// println!("size is ok: {}", size.is_ok()); // false, NAN is not a valid number.
    /// # }
    /// ```
    ///
    /// [not normal]: https://doc.rust-lang.org/nightly/std/primitive.f64.html#method.is_normal
    pub fn new<V: Into<f64>>(value: V, multiple: M) -> Result<SpecificSize<M>, InvalidValueError> {
        let value = value.into();
        if is_valid_value(value) {
            Ok(SpecificSize { value, multiple })
        } else {
            Err(InvalidValueError)
        }
    }
}

/// Check if the provided `value` is valid.
fn is_valid_value(value: f64) -> bool {
    // Zero is not considered normal, but should be accepted.
    value.is_normal() || value == 0.0
}

impl<M: Multiple> FromStr for SpecificSize<M> {
    type Err = ParsingError;

    fn from_str(input: &str) -> Result<SpecificSize<M>, Self::Err> {
        let input = input.trim();
        if input.is_empty() {
            return Err(ParsingError::EmptyInput);
        }

        let multiple_index = input
            .chars()
            .position(|c| !(c.is_numeric() || c == '.'))
            .ok_or(ParsingError::MissingMultiple)?;

        let value_part = &input[0..multiple_index].trim();
        if value_part.is_empty() {
            return Err(ParsingError::MissingValue);
        }
        let value = value_part.parse::<f64>()
            .map_err(|_| ParsingError::InvalidValue)?;

        let multiple_part = &input[multiple_index..].trim();
        let multiple = multiple_part.parse()?;

        if !is_valid_value(value) {
            Err(ParsingError::InvalidValue)
        } else {
            Ok(M::from_any(value, multiple))
        }
    }
}

/*
TODO: enable to specialisation for the same M.
impl<M> PartialEq for SpecificSize<M> {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}
*/

/// The allowed margin to consider two floats still equal, after applying the
/// multiple. Keep in sync with the Notes section of `SpecificSize`.
const CMP_MARGIN: f64 = 0.00000001;

impl<LM, RM> PartialEq<SpecificSize<RM>> for SpecificSize<LM>
    where LM: Multiple + Copy,
          RM: Multiple + Copy,
{
    fn eq(&self, other: &SpecificSize<RM>) -> bool {
        // Ah... floating points...
        // To negate the loss in accuracy we check if the difference between the
        // values is really low and consider that the same.
        let (left, right) = into_same_multiples(*self, *other);
        let diff = left - right;
        diff.abs() < CMP_MARGIN
    }
}

/*
TODO: enable to specialisation for the same M.
impl<M> PartialOrd for SpecificSize<M> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.value.partial_cmp(&other.value)
    }
}
*/

impl<LM, RM> PartialOrd<SpecificSize<RM>> for SpecificSize<LM>
    where LM: Multiple + Copy,
          RM: Multiple + Copy,
{
    fn partial_cmp(&self, other: &SpecificSize<RM>) -> Option<Ordering> {
        let (left, right) = into_same_multiples(*self, *other);
        left.partial_cmp(&right)
    }
}

/// Convert the provided `left` and `right` sizes into the same multiples,
/// returning the values. For example if left is `1 Kilobyte`, and right is
/// `1000 Byte`, it will return `(1, 1)` (in the multiple of Kilobyte).
fn into_same_multiples<LM, RM>(left: SpecificSize<LM>, right: SpecificSize<RM>) -> (f64, f64)
    where LM: Multiple,
          RM: Multiple,
{
    let (left_value, left_multiple) = LM::into_any(left);
    let (right_value, right_multiple) = RM::into_any(right);
    let multiply = left_multiple.multiple_of_bytes() as f64 / right_multiple.multiple_of_bytes() as f64;
    (left_value * multiply, right_value)
}

impl<M: fmt::Display> fmt::Display for SpecificSize<M> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(precision) = f.precision() {
            write!(f, "{:.*} {}", precision, self.value, self.multiple)
        } else {
            write!(f, "{} {}", self.value, self.multiple)
        }
    }
}

/// Trait to convert a `SpecificSize` to and from different `Multiple`s.
pub trait Multiple: Sized {
    /// Create a new [`SpecificSize`] from a `value` and `multiple`, the
    /// provided `value` must always valid (see [`SpecificSize::new`]).
    ///
    /// [`SpecificSize`]: struct.SpecificSize.html
    /// [`SpecificSize::new`]: struct.SpecificSize.html#method.new
    fn from_any(value: f64, multiple: Any) -> SpecificSize<Self>;

    /// The opposite of `from_any`, converting self into the value and the
    /// generic multiple.
    fn into_any(size: SpecificSize<Self>) -> (f64, Any);
}

/// The error returned when trying to create a new [`SpecificSize`] with an
/// invalid value.
///
/// [`SpecificSize`]: struct.SpecificSize.html
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct InvalidValueError;

impl fmt::Display for InvalidValueError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.pad(self.description())
    }
}

impl Error for InvalidValueError {
    fn description(&self) -> &str {
        ParsingError::InvalidValue.description()
    }
}

/// The error returned when trying to parse a [`SpecificSize`], using the
/// [`FromStr`] trait.
///
/// [`SpecificSize`]: struct.SpecificSize.html
/// [`FromStr`]: https://doc.rust-lang.org/nightly/core/str/trait.FromStr.html
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ParsingError {
    /// The provided string is empty, i.e. "".
    EmptyInput,
    /// The provided string is missing a value, e.g. "B".
    MissingValue,
    /// The value is invalid, see [`SpecificSize::new`].
    ///
    /// [`SpecificSize::new`]: struct.SpecificSize.html#method.new
    InvalidValue,
    /// The value is missing the multiple of bytes, e.g. "100".
    MissingMultiple,
    /// The multiple in the string is invalid, e.g. "100 invalid".
    InvalidMultiple,
}

impl fmt::Display for ParsingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.pad(self.description())
    }
}

impl Error for ParsingError {
    fn description(&self) -> &str {
        match *self {
            ParsingError::EmptyInput => "input is empty",
            ParsingError::MissingValue => "no value",
            ParsingError::InvalidValue => "invalid value",
            ParsingError::MissingMultiple => "no multiple",
            ParsingError::InvalidMultiple => "invalid multiple",
        }
    }
}
