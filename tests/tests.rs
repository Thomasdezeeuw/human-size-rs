// Copyright 2017-2018 Thomas de Zeeuw
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT
// or http://opensource.org/licenses/MIT>, at your option. This file may not be
// used, copied, modified, or distributed except according to those terms.

extern crate human_size;

use std::mem;

use human_size::*;

/// Assert that type `T` has the `expected` size.
fn assert_size<T>(expected: usize) {
    assert_eq!(mem::size_of::<T>(), expected);
}

#[test]
fn assertions() {
    const F64_SIZE: usize = mem::size_of::<f64>();

    assert_size::<SpecificSize<Byte>>(F64_SIZE);
    assert_size::<SpecificSize<Kilobyte>>(F64_SIZE);
    assert_size::<SpecificSize<Megabyte>>(F64_SIZE);

    assert_size::<SpecificSize<Any>>(F64_SIZE + 8);
    assert_size::<Size>(F64_SIZE + 8);
}

/// Create a new parse test.
macro_rules! parse_test {
    // Ok case.
    ($input:expr, $size:expr, $type:expr) => {
        let input = $input;
        let expected = Ok(SpecificSize::new($size, $type).unwrap());
        let got = input.parse();
        assert_eq!(got, expected, "input: {:?}", input);
    };
    // Error case.
    ($input:expr, $err:expr) => {
        let input = $input;
        let expected = Err($err);
        let got: Result<Size, ParsingError> = input.parse();
        assert_eq!(got, expected, "input: {:?}", input);
    };
}

#[test]
fn simple_size_parsing() {
    parse_test!("0 B", 0, Byte);
    parse_test!("0B", 0, Byte);

    // Multiples of 1000.
    parse_test!("1.0 kB", 1, Kilobyte);
    parse_test!("123.0 MB", 123, Megabyte);
    parse_test!("100 GB", 100, Gigabyte);
    parse_test!("321 TB", 321, Terabyte);
    parse_test!("10 PB", 10, Petabyte);
    parse_test!("12 EB", 12, Exabyte);
    parse_test!("0.100 ZB", 0.1, Zettabyte);
    parse_test!(".512 YB", 0.512, Yottabyte);

    // Multiples of 1024.
    parse_test!("0.0 KB", 0, Kibibyte);
    parse_test!("1. KiB", 1, Kibibyte);
    parse_test!("1.KiB", 1, Kibibyte);
    parse_test!("100 MiB", 100, Mebibyte);
    parse_test!("100 GiB", 100, Gigibyte);
    parse_test!("123 TiB", 123, Tebibyte);
    parse_test!("512 PiB", 512, Pebibyte);
    parse_test!("312 EiB", 312, Exbibyte);
    parse_test!("1 ZiB", 1, Zebibyte);
    parse_test!("2 YiB", 2, Yobibyte);

    // Same as above, but then using `Any`.
    parse_test!("0 B", 0, Any::Byte);

    parse_test!("1.0 kB", 1, Any::Kilobyte);
    parse_test!("123.0 MB", 123, Any::Megabyte);
    parse_test!("100 GB", 100, Any::Gigabyte);
    parse_test!("100GB", 100, Any::Gigabyte);
    parse_test!("321 TB", 321, Any::Terabyte);
    parse_test!("10 PB", 10, Any::Petabyte);
    parse_test!("12 EB", 12, Any::Exabyte);
    parse_test!("0.100 ZB", 0.1, Any::Zettabyte);
    parse_test!(".512 YB", 0.512, Any::Yottabyte);

    parse_test!("0.0 KB", 0, Any::Kibibyte);
    parse_test!("1. KiB", 1, Any::Kibibyte);
    parse_test!("100 MiB", 100, Any::Mebibyte);
    parse_test!("100 GiB", 100, Any::Gigibyte);
    parse_test!("123 TiB", 123, Any::Tebibyte);
    parse_test!("512 PiB", 512, Any::Pebibyte);
    parse_test!("312 EiB", 312, Any::Exbibyte);
    parse_test!("1 ZiB", 1, Any::Zebibyte);
    parse_test!("2 YiB", 2, Any::Yobibyte);

    // Accept some extra white space.
    parse_test!("   100   B   ", 100, Byte);
    parse_test!("12   MiB   ", 12, Mebibyte);
    parse_test!(" \t\t 100 \n\n  B \n  ", 100, Byte);
}

#[test]
fn parsing_size_conversion() {
    parse_test!("1000 B", 1, Kilobyte);
    parse_test!("12 kB", 12000, Byte);
    parse_test!("1 YiB", 1208925819614629174706176.0, Byte);
    parse_test!("1 YB",  1000000000000000000000000.0, Byte);

    // This is where floats lose there precision.
    parse_test!("100 MiB", 104.85759999999999, Megabyte);

    // TODO: Add more conversion tests.
}

#[test]
fn size_parsing_errors() {
    parse_test!("", ParsingError::EmptyInput);

    parse_test!("B", ParsingError::MissingValue);
    parse_test!("abc MB", ParsingError::MissingValue); // TODO: InvalidValue would be better.

    parse_test!("1.0.0 GB", ParsingError::InvalidValue);
    parse_test!(". B", ParsingError::InvalidValue);

    parse_test!("10", ParsingError::MissingMultiple);

    parse_test!("10 abc", ParsingError::InvalidMultiple);
    parse_test!("10 B extra", ParsingError::InvalidMultiple);
}

/// Create a new display test.
macro_rules! display_test {
    ($size:expr, $type:expr, $expected:expr) => {
        let input = SpecificSize::new($size, $type).unwrap();
        assert_eq!(input.to_string(), $expected, "input: {:?}", input);
    };
}

#[test]
fn displaying_size() {
    display_test!(100, Byte, "100 B");

    // Multiples of 1000.
    display_test!(1.5, Kilobyte, "1.5 kB");
    display_test!(123, Megabyte, "123 MB");
    display_test!(100, Gigabyte, "100 GB");
    display_test!(321, Terabyte, "321 TB");
    display_test!(10, Petabyte, "10 PB");
    display_test!(12, Exabyte, "12 EB");
    display_test!(0.1, Zettabyte, "0.1 ZB");
    display_test!(0.512, Yottabyte, "0.512 YB");

    // Multiples of 1024.
    display_test!(0, Kibibyte, "0 KiB");
    display_test!(1.9999, Kibibyte, "1.9999 KiB");
    display_test!(100, Mebibyte, "100 MiB");
    display_test!(100, Gigibyte, "100 GiB");
    display_test!(123, Tebibyte, "123 TiB");
    display_test!(512, Pebibyte, "512 PiB");
    display_test!(312, Exbibyte, "312 EiB");
    display_test!(1, Zebibyte, "1 ZiB");
    display_test!(2, Yobibyte, "2 YiB");

    // Same but for `Any`.
    display_test!(0, Any::Byte, "0 B");

    display_test!(1.5, Any::Kilobyte, "1.5 kB");
    display_test!(123, Any::Megabyte, "123 MB");
    display_test!(100, Any::Gigabyte, "100 GB");
    display_test!(321, Any::Terabyte, "321 TB");
    display_test!(10, Any::Petabyte, "10 PB");
    display_test!(12, Any::Exabyte, "12 EB");
    display_test!(0.1, Any::Zettabyte, "0.1 ZB");
    display_test!(0.512, Any::Yottabyte, "0.512 YB");

    display_test!(0, Any::Kibibyte, "0 KiB");
    display_test!(1.9999, Any::Kibibyte, "1.9999 KiB");
    display_test!(100, Any::Mebibyte, "100 MiB");
    display_test!(100, Any::Gigibyte, "100 GiB");
    display_test!(123, Any::Tebibyte, "123 TiB");
    display_test!(512, Any::Pebibyte, "512 PiB");
    display_test!(312, Any::Exbibyte, "312 EiB");
    display_test!(1, Any::Zebibyte, "1 ZiB");
    display_test!(2, Any::Yobibyte, "2 YiB");

    // Test provided precision.
    let input = SpecificSize::new(1.1234567890, Byte).unwrap();
    assert_eq!(format!("{:.4}", input), "1.1235 B", "input: {:?}", input);
}

/// Create an equivalence test.
macro_rules! equivalence_test {
    ($size_left:expr, $type_left:expr, $size_right:expr, $type_right:expr) => {
        let left = SpecificSize::new($size_left, $type_left).unwrap();
        let right = SpecificSize::new($size_right, $type_right).unwrap();
        assert_eq!(left, right);
    };
}

#[test]
fn equivalence_tests() {
    // TODO: Remove all `Any::` tests and move that to the macro, so that each
    // test creates 4 assertions.
    equivalence_test!(1, Byte, 1, Byte);
    equivalence_test!(1, Byte, 1, Any::Byte);

    equivalence_test!(1000, Byte, 1, Kilobyte);
    equivalence_test!(1000, Byte, 1, Any::Kilobyte);
    equivalence_test!(1000f64.powi(2), Byte, 1, Megabyte);
    equivalence_test!(1000f64.powi(2), Byte, 1, Any::Megabyte);
    equivalence_test!(1000f64.powi(3), Byte, 1, Gigabyte);
    equivalence_test!(1000f64.powi(3), Byte, 1, Any::Gigabyte);
    equivalence_test!(1000f64.powi(4), Byte, 1, Terabyte);
    equivalence_test!(1000f64.powi(4), Byte, 1, Any::Terabyte);
    equivalence_test!(1000f64.powi(5), Byte, 1, Petabyte);
    equivalence_test!(1000f64.powi(5), Byte, 1, Any::Petabyte);
    equivalence_test!(1000f64.powi(6), Byte, 1, Exabyte);
    equivalence_test!(1000f64.powi(6), Byte, 1, Any::Exabyte);
    equivalence_test!(1000f64.powi(7), Byte, 1, Zettabyte);
    equivalence_test!(1000f64.powi(7), Byte, 1, Any::Zettabyte);
    equivalence_test!(1000f64.powi(8), Byte, 1, Yottabyte);
    equivalence_test!(1000f64.powi(8), Byte, 1, Any::Yottabyte);

    equivalence_test!(1024, Byte, 1, Kibibyte);
    equivalence_test!(1024, Byte, 1, Any::Kibibyte);
    equivalence_test!(1024f64.powi(2), Byte, 1, Mebibyte);
    equivalence_test!(1024f64.powi(2), Byte, 1, Any::Mebibyte);
    equivalence_test!(1024f64.powi(3), Byte, 1, Gigibyte);
    equivalence_test!(1024f64.powi(3), Byte, 1, Any::Gigibyte);
    equivalence_test!(1024f64.powi(4), Byte, 1, Tebibyte);
    equivalence_test!(1024f64.powi(4), Byte, 1, Any::Tebibyte);
    equivalence_test!(1024f64.powi(5), Byte, 1, Pebibyte);
    equivalence_test!(1024f64.powi(5), Byte, 1, Any::Pebibyte);
    equivalence_test!(1024f64.powi(6), Byte, 1, Exbibyte);
    equivalence_test!(1024f64.powi(6), Byte, 1, Any::Exbibyte);
    equivalence_test!(1024f64.powi(7), Byte, 1, Zebibyte);
    equivalence_test!(1024f64.powi(7), Byte, 1, Any::Zebibyte);
    equivalence_test!(1024f64.powi(8), Byte, 1, Yobibyte);
    equivalence_test!(1024f64.powi(8), Byte, 1, Any::Yobibyte);

    equivalence_test!(1073.741824f64, Megabyte, 1, Gigibyte);
    equivalence_test!(1073.741824f64, Megabyte, 1, Any::Gigibyte);
    equivalence_test!(1073.741824f64, Any::Megabyte, 1, Gigibyte);
    equivalence_test!(1073.741824f64, Any::Megabyte, 1, Any::Gigibyte);
    equivalence_test!(1, Mebibyte, 1.048576, Megabyte);
    equivalence_test!(1, Mebibyte, 1.048576, Any::Megabyte);
    equivalence_test!(1, Any::Mebibyte, 1.048576, Megabyte);
    equivalence_test!(1, Any::Mebibyte, 1.048576, Any::Megabyte);
}

/// Create an ordering test.
macro_rules! ordering_test {
    ($size_left:expr, $type_left:expr, $cmp:expr, $size_right:expr, $type_right:expr) => {
        let left = SpecificSize::new($size_left, $type_left).unwrap();
        let right = SpecificSize::new($size_right, $type_right).unwrap();
        assert_eq!(left.partial_cmp(&right), Some($cmp));
    };
}

#[test]
fn ordering_tests() {
    use std::cmp::Ordering::*;
    ordering_test!(1, Byte, Equal, 1, Byte);
    ordering_test!(2, Byte, Greater, 1, Byte);
    ordering_test!(1, Byte, Less, 3, Byte);

    ordering_test!(1024, Byte, Equal, 1, Kibibyte);
    ordering_test!(1025, Byte, Greater, 1, Kibibyte);
    ordering_test!(1023, Byte, Less, 1, Kibibyte);

    ordering_test!(1000, Byte, Equal, 1, Kilobyte);
    ordering_test!(1001, Byte, Greater, 1, Kilobyte);
    ordering_test!(0999, Byte, Less, 1, Kilobyte);

    ordering_test!(1, Kibibyte, Greater, 1, Kilobyte);
    ordering_test!(1, Kilobyte, Less, 1, Kibibyte);
}

macro_rules! into_test {
    ($size_left:expr, $type_left:expr, $size_right:expr, $type_right:expr, $tr:ty) => {
        let left = SpecificSize::new($size_left, $type_left).unwrap();
        let converted = left.into::<$tr>();
        let right = SpecificSize::new($size_right, $type_right).unwrap();
        assert_eq!(converted, right);
    };
}

#[test]
fn into_tests() {
    into_test!(1, Byte, 0.001, Kilobyte, Kilobyte);
    into_test!(1000, Byte, 1, Kilobyte, Kilobyte);
    into_test!(1000, Byte, 1, Kilobyte, Kilobyte);
}
