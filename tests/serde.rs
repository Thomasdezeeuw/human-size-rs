#![cfg(feature = "serde")]

use human_size::{Byte, Kilobyte, Megabyte, Size, SpecificSize, Terabyte};
use serde_test::{assert_de_tokens_error, assert_tokens, Token};

/// Runs both serialize and deserialize.
macro_rules! test {
    ($value: expr, $multiple: expr, $want: expr) => {{
        let size = SpecificSize::new($value, $multiple).unwrap();
        assert_tokens(&size, &[Token::Str($want)]);
    }};
}

#[test]
fn size_serialize() {
    test!(100, Byte, "100 B");
    test!(3000, Kilobyte, "3000 kB");
    test!(132, Terabyte, "132 TB");
    test!(0, Megabyte, "0 MB");
}

macro_rules! serialize_err_test {
    ($input: expr, $want: expr) => {{
        assert_de_tokens_error::<Size>(&[Token::Str($input)], $want);
    }};
}

#[test]
fn size_deserialize_error() {
    serialize_err_test!("", "input is empty");

    serialize_err_test!("B", "no value");
    serialize_err_test!("abc MB", "no value");

    serialize_err_test!("1.0.0 GB", "invalid value");
    serialize_err_test!(". B", "invalid value");

    serialize_err_test!("10", "no multiple");

    serialize_err_test!("10 abc", "invalid multiple");
    serialize_err_test!("10 B extra", "invalid multiple");
}
