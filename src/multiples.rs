// Copyright 2017-2018 Thomas de Zeeuw
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT
// or http://opensource.org/licenses/MIT>, at your option. This file may not be
// used, copied, modified, or distributed except according to those terms.

//! Module containing all multiples.
//!
//! All types defined here implement [`Multiple`]. Because all types defined
//! here, expect for `Any`, don't have any fields they are always zero sized.
//! Meaning that for example `SpecificSize<Byte>` has the same size as `f64`
//! (the type used as underlying value).
//!
//! [`Multiple`]: ../trait.Multiple.html

use std::fmt;
use std::str::FromStr;

use super::{SpecificSize, ParsingError, Multiple};

/// Macro used to allow the `concat` macro to be used inside the doc attribute.
///
/// Inspired by the same macro found in the `num` module of Rust's standard
/// library.
macro_rules! doc_comment {
    ($doc:expr, $($tt:tt)*) => {
        #[doc = $doc]
        $($tt)*
    };
}

/// Macro to create a multiple.
///
/// This multiple will be a zero sized struct that implements `Multiple` and
/// `fmt::Display`.
macro_rules! multiple {
    ($name:ident, $size:expr, $str:expr) => {
        multiple!($name, $size, $str, stringify!($name), stringify!($size));
    };
    ($name:ident, $size:expr, $str:expr, $sname:expr, $ssize:expr) => {
        doc_comment! {
            concat!("Multiple representing a ", $sname, ".\n\n",
                    "Represents a size of `value * ", $ssize,
                    "`. When parsing this multiple from text it expects `",
                    $str, "`."),
            #[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
            pub struct $name;
        }

        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                f.pad($str)
            }
        }

        impl Multiple for $name {
            fn from_any(value: f64, multiple: Any) -> SpecificSize<Self> {
                let multiply = multiple.multiple_of_bytes() / $size;
                let value = value * multiply;
                SpecificSize { value, multiple: $name }
            }

            fn into_any(size: SpecificSize<Self>) -> (f64, Any) {
                (size.value, Any::$name)
            }
        }

        impl From<$name> for Any {
            fn from(_multiple: $name) -> Any {
                Any::$name
            }
        }
    };
}

multiple!(Byte, 1_f64, "B");

// Multiples of 1000.
multiple!(Kilobyte,  1000_f64.powi(1), "kB");
multiple!(Megabyte,  1000_f64.powi(2), "MB");
multiple!(Gigabyte,  1000_f64.powi(3), "GB");
multiple!(Terabyte,  1000_f64.powi(4), "TB");
multiple!(Petabyte,  1000_f64.powi(5), "PB");
multiple!(Exabyte,   1000_f64.powi(6), "EB");
multiple!(Zettabyte, 1000_f64.powi(7), "ZB");
multiple!(Yottabyte, 1000_f64.powi(8), "YB");

// Multiples of 1024.
multiple!(Kibibyte, 1024_f64.powi(1), "KiB");
multiple!(Mebibyte, 1024_f64.powi(2), "MiB");
multiple!(Gigibyte, 1024_f64.powi(3), "GiB");
multiple!(Tebibyte, 1024_f64.powi(4), "TiB");
multiple!(Pebibyte, 1024_f64.powi(5), "PiB");
multiple!(Exbibyte, 1024_f64.powi(6), "EiB");
multiple!(Zebibyte, 1024_f64.powi(7), "ZiB");
multiple!(Yobibyte, 1024_f64.powi(8), "YiB");

/// A multiple which can represent all multiples.
///
/// This is mainly used to parse a size from a string, but can also be used when
/// you don't really care about the multiple or want to maintain the multiple
/// from the parsed string.
///
/// For documentation of each variant see the equivalent struct in this module.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[allow(missing_docs)]
pub enum Any {
    Byte,
    Kilobyte,
    Megabyte,
    Gigabyte,
    Terabyte,
    Petabyte,
    Exabyte,
    Zettabyte,
    Yottabyte,
    Kibibyte,
    Mebibyte,
    Gigibyte,
    Tebibyte,
    Pebibyte,
    Exbibyte,
    Zebibyte,
    Yobibyte,

    /// This is not an actual `Multiple`, but allows the enum to be expanded in
    /// the future without breaking match statements that try to match all
    /// types.
    ///
    /// TODO: replace it with the `non_exhaustive` attribute.
    #[doc(hidden)]
    __NonExhaustive,
}

impl Multiple for Any {
    fn from_any(value: f64, multiple: Any) -> SpecificSize<Self> {
        SpecificSize { value, multiple }
    }

    fn into_any(size: SpecificSize<Self>) -> (f64, Any) {
        (size.value, size.multiple)
    }
}

impl Any {
    pub(crate) fn multiple_of_bytes(self) -> f64 {
        match self {
            Any::Byte => 1_f64,

            Any::Kilobyte =>  1000_f64,
            Any::Megabyte =>  1000_f64.powi(2),
            Any::Gigabyte =>  1000_f64.powi(3),
            Any::Terabyte =>  1000_f64.powi(4),
            Any::Petabyte =>  1000_f64.powi(5),
            Any::Exabyte =>   1000_f64.powi(6),
            Any::Zettabyte => 1000_f64.powi(7),
            Any::Yottabyte => 1000_f64.powi(8),

            Any::Kibibyte => 1024_f64,
            Any::Mebibyte => 1024_f64.powi(2),
            Any::Gigibyte => 1024_f64.powi(3),
            Any::Tebibyte => 1024_f64.powi(4),
            Any::Pebibyte => 1024_f64.powi(5),
            Any::Exbibyte => 1024_f64.powi(6),
            Any::Zebibyte => 1024_f64.powi(7),
            Any::Yobibyte => 1024_f64.powi(8),

            Any::__NonExhaustive => unreachable!(),
        }
    }
}

impl FromStr for Any {
    type Err = ParsingError;

    fn from_str(input: &str) -> Result<Any, Self::Err> {
        match input {
            "B" => Ok(Any::Byte),

            "kB" => Ok(Any::Kilobyte),
            "MB" => Ok(Any::Megabyte),
            "GB" => Ok(Any::Gigabyte),
            "TB" => Ok(Any::Terabyte),
            "PB" => Ok(Any::Petabyte),
            "EB" => Ok(Any::Exabyte),
            "ZB" => Ok(Any::Zettabyte),
            "YB" => Ok(Any::Yottabyte),

            "KB" | "KiB" => Ok(Any::Kibibyte),
            "MiB" => Ok(Any::Mebibyte),
            "GiB" => Ok(Any::Gigibyte),
            "TiB" => Ok(Any::Tebibyte),
            "PiB" => Ok(Any::Pebibyte),
            "EiB" => Ok(Any::Exbibyte),
            "ZiB" => Ok(Any::Zebibyte),
            "YiB" => Ok(Any::Yobibyte),

            _ => Err(ParsingError::InvalidMultiple),
        }
    }
}

impl fmt::Display for Any {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let value = match *self {
            Any::Byte => "B",

            Any::Kilobyte => "kB",
            Any::Megabyte => "MB",
            Any::Gigabyte => "GB",
            Any::Terabyte => "TB",
            Any::Petabyte => "PB",
            Any::Exabyte => "EB",
            Any::Zettabyte => "ZB",
            Any::Yottabyte => "YB",

            Any::Kibibyte => "KiB",
            Any::Mebibyte => "MiB",
            Any::Gigibyte => "GiB",
            Any::Tebibyte => "TiB",
            Any::Pebibyte => "PiB",
            Any::Exbibyte => "EiB",
            Any::Zebibyte => "ZiB",
            Any::Yobibyte => "YiB",

            Any::__NonExhaustive => unreachable!(),
        };
        f.pad(value)
    }
}
