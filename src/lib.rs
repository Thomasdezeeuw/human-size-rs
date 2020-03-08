// Copyright 2017-2018 Thomas de Zeeuw
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT
// or http://opensource.org/licenses/MIT>, at your option. This file may not be
// used, copied, modified, or distributed except according to those terms.

#![warn(anonymous_parameters,
        bare_trait_objects,
        missing_debug_implementations,
        missing_docs,
        trivial_casts,
        trivial_numeric_casts,
        unused_extern_crates,
        unused_import_braces,
        unused_qualifications,
        unused_results,
        variant_size_differences,
)]

#![forbid(unsafe_code)]

//! The `human_size` crate represents sizes for humans.
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
//! Below is small example that parses a size from a string and prints it.
//!
//! ```
//! # extern crate human_size;
//! # fn main() {
//! use human_size::{Size, SpecificSize, Kilobyte};
//!
//! let size1: Size = "10000 B".parse().unwrap();
//! assert_eq!(size1.to_string(), "10000 B");
//!
//! // Or using a specific multiple.
//! let size2: SpecificSize<Kilobyte> = "10000 B".parse().unwrap();
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

/// Size with a generic [`Multiple`].
///
/// # Notes
///
/// The size of `Size` is 16 bytes, but using a specific multiple, e.g.
/// `SpecificSize<Byte>`, requires only 8 bytes.
///
/// [`Multiple`]: trait.Multiple.html
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
/// When comparing sizes with one another it is to possible compare different
/// multiples, see the first example above. However due to a lack of precision
/// in floating point numbers equality ignores a difference less then
/// `0.00000001`, after applying the multiple. See the `PartialEq`
/// implementation (via \[src\] to the right) for details.
///
/// The same is true for converting to and from multiples, here again the lack
/// of precision of floating points can be a cause of bugs.
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
    /// Create a new `SpecificSize` with the given value and multiple. If the
    /// `value` is [not normal] this will return an error, however zero is
    /// allowed. If the `value` is normal the result can be safely unwraped.
    ///
    /// ```
    /// # extern crate human_size;
    /// # fn main() {
    /// use std::f64;
    /// use human_size::{SpecificSize, Kilobyte, InvalidValueError};
    ///
    /// let size = SpecificSize::new(100, Kilobyte).unwrap();
    /// assert_eq!(size.to_string(), "100 kB");
    ///
    /// let res = SpecificSize::new(f64::NAN, Kilobyte);
    /// assert_eq!(res, Err(InvalidValueError)); // NAN is not a valid number.
    /// # }
    /// ```
    ///
    /// [not normal]: https://doc.rust-lang.org/nightly/std/primitive.f64.html#method.is_normal
    pub fn new<V>(value: V, multiple: M) -> Result<SpecificSize<M>, InvalidValueError>
        where V: Into<f64>,
    {
        let value = value.into();
        if is_valid_value(value) {
            Ok(SpecificSize { value, multiple })
        } else {
            Err(InvalidValueError)
        }
    }

    /// Conversion between sizes with different multiples.
    ///
    /// This allows a size with one multiple to be converted into a size with
    /// another multiple.
    ///
    /// ```
    /// # extern crate human_size;
    /// # fn main() {
    /// use human_size::{SpecificSize, Byte, Kilobyte};
    ///
    /// let size = SpecificSize::new(1, Kilobyte).unwrap();
    /// let size2: SpecificSize<Byte> = size.into();
    ///
    /// assert_eq!(size, size2);
    /// assert_eq!(size.to_string(), "1 kB");
    /// assert_eq!(size2.to_string(), "1000 B");
    /// # }
    /// ```
    ///
    /// # Notes
    ///
    /// Normally this would be done by implementing the `From` or `Into` trait.
    /// However currently this is not possible due to the blanket implementation
    /// in the standard library. Maybe once specialisation is available this can
    /// be resolved.
    pub fn into<M2>(self) -> SpecificSize<M2>
        where M2: Multiple,
    {
        let (value, any) = M::into_any(self);
        M2::from_any(value, any)
    }

    /// Returns the size in current the multiple.
    ///
    /// ```
    /// # extern crate human_size;
    /// # fn main() {
    /// use human_size::{SpecificSize, Kilobyte};
    ///
    /// let size = SpecificSize::new(1, Kilobyte).unwrap();
    ///
    /// assert_eq!(size.value(), 1.0);
    /// # }
    /// ```
    pub fn value(self) -> f64 {
        self.value
    }

    /// Returns the multiple.
    ///
    /// ```
    /// # extern crate human_size;
    /// # fn main() {
    /// use human_size::{SpecificSize, Any, Kilobyte};
    ///
    /// let size1 = SpecificSize::new(1, Kilobyte).unwrap();
    /// let size2 = SpecificSize::new(1, Any::Kilobyte).unwrap();
    ///
    /// assert_eq!(size1.multiple(), Kilobyte);
    /// assert_eq!(size2.multiple(), Any::Kilobyte);
    /// # }
    /// ```
    pub fn multiple(self) -> M {
        self.multiple
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
        if multiple_index == 0  {
            return Err(ParsingError::MissingValue);
        }

        let (value, multiple) = &input.split_at(multiple_index);
        let value = value.parse().map_err(|_| ParsingError::InvalidValue)?;

        if is_valid_value(value) {
            let multiple = multiple.trim().parse()?;
            Ok(M::from_any(value, multiple))
        } else {
            Err(ParsingError::InvalidValue)
        }
    }
}

/*
TODO: Needs specialisation.
impl<M1: Multiple, M2: Multiple> From<SpecificSize<M2>> for SpecificSize<M1> {
    fn from(size: SpecificSize<M2>) -> Self {
        let (value, any) = M2::into_any(size);
        M1::from_any(value, any)
    }
}
*/

/*
TODO: Enable to specialisation for the same M.
impl<M> PartialEq for SpecificSize<M> {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}
*/

/// The allowed margin to consider two floats still equal, after applying the
/// multiple. Keep in sync with the Notes section of `SpecificSize`.
const CMP_MARGIN: f64 = 0.000_000_01;

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
TODO: Enable to specialisation for the same M.
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
    let multiply = left_multiple.multiple_of_bytes() / right_multiple.multiple_of_bytes();
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

/// Trait to convert a [`SpecificSize`] to and from different multiples.
///
/// [`SpecificSize`]: struct.SpecificSize.html
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
        ParsingError::InvalidValue.fmt(f)
    }
}

impl Error for InvalidValueError {}

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
        f.pad(match *self {
            ParsingError::EmptyInput => "input is empty",
            ParsingError::MissingValue => "no value",
            ParsingError::InvalidValue => "invalid value",
            ParsingError::MissingMultiple => "no multiple",
            ParsingError::InvalidMultiple => "invalid multiple",
        })
    }
}

impl Error for ParsingError {}
