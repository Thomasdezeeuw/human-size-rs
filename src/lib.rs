// Copyright 2017 Thomas de Zeeuw
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT
// or http://opensource.org/licenses/MIT>, at your option. This file may not be
// used, copied, modified, or distributed except according to those terms.

#![warn(missing_docs)]

// TODO: implement serde.

//! The `human_size` represents sizes for humans. The main type is [`Size`],
//! which (as the name might suggests) represents a size in multiple of bytes.
//!
//! [`Size`]: struct.Size.html
//!
//! # Example
//!
//! Below is small example that parses a size from a string, prints it and get
//! the size in bytes.
//!
//! ```
//! # extern crate human_size;
//! # fn main() {
//! use human_size::Size;
//! let size = "100 KB".parse::<Size>().unwrap();
//! println!("size: {}", size); // 100 KB
//!
//! let bytes = size.into_bytes();
//! println!("size in bytes: {}", bytes); // 102400
//! # }
//! ```

use std::fmt;
use std::error::Error;
use std::str::FromStr;
use std::cmp::Ordering;

/// `Size` represents a size in bytes. `Size` can be created using the `new`
/// function, or parsed from a string using the [`FromStr`] trait.
///
/// When comparing to `Size`s it is done on a byte level, so if one `Size` is
/// created with a [`Multiple`] of `Byte` and another of `Kilobyte` it won't
/// matter.
///
/// ```
/// # extern crate human_size;
/// # fn main() {
/// use human_size::{Size, Multiple};
/// let size1 = Size::new(1000, Multiple::Byte).unwrap();
/// let size2 = Size::new(1, Multiple::Kilobyte).unwrap();
/// println!("equal: {}", size1 == size2); // true
/// # }
/// ```
///
/// [`FromStr`]: https://doc.rust-lang.org/nightly/core/str/trait.FromStr.html
/// [`Multiple`]: enum.Multiple.html
#[derive(Copy, Clone, Debug)]
pub struct Size {
    value: f64,
    multiple: Multiple,
}

impl Size {
    /// Create a new `Size` with the multiple of bytes and the value. If the
    /// `value` is [not normal] this will return an error, zero is allowed. If
    /// the `value` is normal the result can be safely unwraped.
    ///
    /// ```
    /// # extern crate human_size;
    /// # fn main() {
    /// use std::f64;
    /// use human_size::{Size, Multiple};
    ///
    /// let size = Size::new(100, Multiple::Kilobyte).unwrap();
    /// println!("size: {}", size); // 100 kB
    ///
    /// let size = Size::new(f64::NAN, Multiple::Kilobyte);
    /// println!("size: {}", size.is_ok()); // false, NAN is not a valid number.
    /// # }
    /// ```
    ///
    /// [not normal]: https://doc.rust-lang.org/nightly/std/primitive.f64.html#method.is_normal
    pub fn new<V>(value: V, multiple: Multiple) -> Result<Size, ()>
        where V: Into<f64>,
    {
        let value = value.into();
        // Zero is not considered normal, but should be accepted here.
        if !value.is_normal() && value != 0.0 {
            Err(())
        } else {
            Ok(Size {
                value: value,
                multiple,
            })
        }
    }

    /// Convert the `Size` into bytes, be wary of overflows!
    ///
    /// ```
    /// # extern crate human_size;
    /// # fn main() {
    /// use human_size::{Size, Multiple};
    /// let size1 = Size::new(1000, Multiple::Byte).unwrap();
    /// let size2 = Size::new(1, Multiple::Kilobyte).unwrap();
    /// println!("size1 bytes: {}", size1.into_bytes()); // 1000
    /// println!("size2 bytes: {}", size2.into_bytes()); // 1000
    /// println!("equal: {}", size1.into_bytes() == size2.into_bytes()); // true
    /// # }
    /// ```
    pub fn into_bytes(self) -> f64 {
        self.value * (self.multiple.multiple_of_bytes() as f64)
    }
}

impl FromStr for Size {
    type Err = ParsingError;

    fn from_str(input: &str) -> Result<Size, Self::Err> {
        let (index, _) = input
            .char_indices()
            .find(|&(_, c)| !(c.is_numeric() || c == '.'))
            .ok_or(ParsingError::MissingMultiple)?;
        let value_part = &input[0..index];
        if value_part.len() == 0 {
            return Err(ParsingError::MissingValue);
        }
        let multiple_part = input[index..].trim();
        let value = value_part.parse::<f64>().or_else(
            |_| Err(ParsingError::InvalidValue),
        )?;
        let multiple = multiple_part.parse()?;

        let size = Size::new(value, multiple).map_err(
            |_| ParsingError::InvalidValue,
        )?;
        Ok(size)
    }
}

impl Eq for Size {}

impl PartialEq for Size {
    fn eq(&self, other: &Size) -> bool {
        self.into_bytes() == other.into_bytes()
    }
}

impl PartialOrd for Size {
    fn partial_cmp(&self, other: &Size) -> Option<Ordering> {
        self.into_bytes().partial_cmp(&other.into_bytes())
    }
}

impl fmt::Display for Size {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}", self.value, self.multiple)
    }
}

/// A `Multiple` represent a multiple of bytes. This is mainly used to keep track
/// of what multiple [`Size`] uses, so it can display it using the same multiple
/// of bytes.
///
/// [`Size`]: struct.Size.html
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Multiple {
    /// Represents a single byte, value * 1, "B" when parsing text.
    Byte,

    /// A kilobyte, value * 1,000 (1000^1), "kB" in when parsing from text.
    Kilobyte,

    /// A megabyte, value * 1,000,000 (1000^2), "MB" in when parsing from text.
    Megabyte,

    /// A gigabyte, value * 1,000,000,000 (1000^3), "GB" in when parsing from
    /// text.
    Gigabyte,

    /// A terabyte, value * 1,000,000,000,000 (1000^4), "TB" in when parsing
    /// from text.
    Terabyte,

    /// A petabyte, value * 1,000,000,000,000,000 (1000^5), "PB" in when
    /// parsing from text.
    Petabyte,

    /*
    /// A exabyte, value * 1,000,000,000,000,000,000 (1000^6), "EB" in when
    /// parsing from text.
    Exabyte,

    /// A zettabyte, value * 1,000,000,000,000,000,000,000 (1000^7), "ZB" in
    /// when parsing from text.
    Zettabyte,

    /// A yottabyte, value * 1,000,000,000,000,000,000,000,000 (1000^8), "YB"
    /// in when parsing from text.
    Yottabyte,
    */

    /// A kibibyte, value * 1,024 (1024^1), "KiB" or "KB" in when parsing from
    /// text.
    Kibibyte,

    /// A mebibyte, value * 1,048,576 (1024^2), "MiB" in when parsing from text.
    Mebibyte,

    /// A gigibyte, value * 1,073,741,824 (1024^3), "GiB" in when parsing from
    /// text.
    Gigibyte,

    /// A tebibyte, value * 1,099,511,627,776 (1024^4), "TiB" in when parsing
    /// from text.
    Tebibyte,

    /// A pebibyte, value * 1,125,899,906,842,624 (1024^5), "PiB" in when
    /// parsing from text.
    Pebibyte,

    /*
    /// A exbibyte, value * 1,152,921,504,606,846,976 (1024^6), "EiB" in when
    /// parsing from text.
    Exbibyte,

    /// A zebibyte, value * 1,180,591,620,717,411,303,424 (1024^7), "ZiB" in
    /// when parsing from text.
    Zebibyte,

    /// A yobibyte, value * 1,208,925,819,614,629,174,706,176 (1024^8), "YiB"
    /// in when parsing from text.
    Yobibyte,
    */

    /// This is not an actual `Multiple`, but allows the enum to be expanded in
    /// the future without breaking match statements that try to match all
    /// frame types, because shouldn't be possible anymore.
    #[doc(hidden)]
    __NonExhaustive,
}

impl Multiple {
    fn multiple_of_bytes(self) -> u64 {
        match self {
            Multiple::Byte => 1,

            Multiple::Kilobyte => 1000,
            Multiple::Megabyte => 1000u64.pow(2),
            Multiple::Gigabyte => 1000u64.pow(3),
            Multiple::Terabyte => 1000u64.pow(4),
            Multiple::Petabyte => 1000u64.pow(5),
            //Multiple::Exabyte => 1000u64.pow(6),
            //Multiple::Zettabyte => 1000u64.pow(7),
            //Multiple::Yottabyte => 1000u64.pow(8),

            Multiple::Kibibyte => 1024,
            Multiple::Mebibyte => 1024u64.pow(2),
            Multiple::Gigibyte => 1024u64.pow(3),
            Multiple::Tebibyte => 1024u64.pow(4),
            Multiple::Pebibyte => 1024u64.pow(5),
            //Multiple::Exbibyte => 1024u64.pow(6),
            //Multiple::Zebibyte => 1024u64.pow(7),
            //Multiple::Yobibyte => 1024u64.pow(8),

            Multiple::__NonExhaustive => unreachable!(),
        }
    }
}

impl FromStr for Multiple {
    type Err = ParsingError;

    fn from_str(input: &str) -> Result<Multiple, Self::Err> {
        match input {
            "B" => Ok(Multiple::Byte),

            "kB" => Ok(Multiple::Kilobyte),
            "MB" => Ok(Multiple::Megabyte),
            "GB" => Ok(Multiple::Gigabyte),
            "TB" => Ok(Multiple::Terabyte),
            "PB" => Ok(Multiple::Petabyte),
            //"EB" => Ok(Multiple::Exabyte),
            //"ZB" => Ok(Multiple::Zettabyte),
            //"YB" => Ok(Multiple::Yottabyte),

            "KB" | "KiB" => Ok(Multiple::Kibibyte),
            "MiB" => Ok(Multiple::Mebibyte),
            "GiB" => Ok(Multiple::Gigibyte),
            "TiB" => Ok(Multiple::Tebibyte),
            "PiB" => Ok(Multiple::Pebibyte),
            //"EiB" => Ok(Multiple::Exbibyte),
            //"ZiB" => Ok(Multiple::Zebibyte),
            //"YiB" => Ok(Multiple::Yobibyte),

            _ => Err(ParsingError::InvalidMultiple),
        }
    }
}

impl fmt::Display for Multiple {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let value = match *self {
            Multiple::Byte => "B",

            Multiple::Kilobyte => "kB",
            Multiple::Megabyte => "MB",
            Multiple::Gigabyte => "GB",
            Multiple::Terabyte => "TB",
            Multiple::Petabyte => "PB",
            //Multiple::Exabyte => "EB",
            //Multiple::Zettabyte => "ZB",
            //Multiple::Yottabyte => "YB",

            Multiple::Kibibyte => "KiB",
            Multiple::Mebibyte => "MiB",
            Multiple::Gigibyte => "GiB",
            Multiple::Tebibyte => "TiB",
            Multiple::Pebibyte => "PiB",
            //Multiple::Exbibyte => "EiB",
            //Multiple::Zebibyte => "ZiB",
            //Multiple::Yobibyte => "YiB",

            Multiple::__NonExhaustive => unreachable!(),
        };
        f.pad(value)
    }
}

/// The error returned when trying to parse a [`Size`] or [`Mulitple`] from a
/// string, using the [`FromStr`] trait.
///
/// [`Size`]: struct.Size.html
/// [`Mulitple`]: enum.Multiple.html
/// [`FromStr`]: https://doc.rust-lang.org/nightly/core/str/trait.FromStr.html
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ParsingError {
    /// The provided string is missing a value.
    MissingValue,
    /// The value is invalid.
    InvalidValue,
    /// The value is missing the multiple of bytes.
    MissingMultiple,
    /// The multiple in the string is invalid.
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
            ParsingError::MissingValue => "no value",
            ParsingError::InvalidValue => "invalid value",
            ParsingError::MissingMultiple => "no multiple",
            ParsingError::InvalidMultiple => "invalid multiple",
        }
    }
}
