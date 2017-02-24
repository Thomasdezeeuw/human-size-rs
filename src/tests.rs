use super::*;

#[test]
fn size_try_into_u32() {
    let tests = vec![
        (Size::new(1, Multiple::Byte), Ok(1)),

        (Size::new(1, Multiple::Kilobyte), Ok(1_000)),
        (Size::new(23, Multiple::Kilobyte), Ok(23_000)),
        (Size::new(65, Multiple::Megabyte), Ok(65_000_000)),
        (Size::new(1, Multiple::Gigabyte), Ok(1_000_000_000)),

        (Size::new(8, Multiple::Kibibyte), Ok(8192)),
        (Size::new(1000, Multiple::Mebibyte), Ok(1_048_576_000)),
        (Size::new(10, Multiple::Mebibyte), Ok(10_485_760)),

        (Size::new(10, Multiple::Gigabyte), Err(ConversionError::Overflow)),
        (Size::new(1, Multiple::Terabyte), Err(ConversionError::Overflow)),
    ];

    for test in tests {
        let got: Result<u32, ConversionError> = test.0.clone().try_into();
        let want = test.1;
        assert_eq!(got, want, "input: {:?}", test.0);
    }
}

#[test]
fn size_try_into_u64() {
    let tests = vec![
        (Size::new(1, Multiple::Byte), Ok(1)),

        (Size::new(1, Multiple::Kilobyte), Ok(1_000)),
        (Size::new(23, Multiple::Kilobyte), Ok(23_000)),
        (Size::new(65, Multiple::Megabyte), Ok(65_000_000)),
        (Size::new(123, Multiple::Gigabyte), Ok(123_000_000_000)),
        (Size::new(2, Multiple::Petabyte), Ok(2_000_000_000_000_000)),

        (Size::new(10, Multiple::Mebibyte), Ok(10_485_760)),
        (Size::new(1000, Multiple::Gigibyte), Ok(1_073_741_824_000)),
        (Size::new(1, Multiple::Pebibyte), Ok(1_125_899_906_842_624)),
        (Size::new(2, Multiple::Pebibyte), Ok(2_251_799_813_685_248)),

        (Size::new(1, Multiple::Exabyte), Err(ConversionError::Overflow)),
    ];

    for test in tests {
        let got: Result<u64, ConversionError> = test.0.clone().try_into();
        let want = test.1;
        assert_eq!(got, want, "input: {:?}", test.0);
    }
}

#[test]
fn size_try_into_u128() {
    let tests = vec![
        (Size::new(1, Multiple::Byte), Ok(1)),

        (Size::new(1, Multiple::Kilobyte), Ok(1_000)),
        (Size::new(23, Multiple::Kilobyte), Ok(23_000)),
        (Size::new(65, Multiple::Megabyte), Ok(65_000_000)),
        (Size::new(123, Multiple::Gigabyte), Ok(123_000_000_000)),
        (Size::new(2, Multiple::Petabyte), Ok(2_000_000_000_000_000)),
        (Size::new(25, Multiple::Exabyte), Ok(25_000_000_000_000_000_000)),
        (Size::new(200, Multiple::Zettabyte), Ok(200_000_000_000_000_000_000_000)),
        (Size::new(2, Multiple::Yottabyte), Ok(2_000_000_000_000_000_000_000_000)),

        (Size::new(10, Multiple::Mebibyte), Ok(10_485_760)),
        (Size::new(1000, Multiple::Gigibyte), Ok(1_073_741_824_000)),
        (Size::new(1, Multiple::Pebibyte), Ok(1_125_899_906_842_624)),
        (Size::new(2, Multiple::Pebibyte), Ok(2_251_799_813_685_248)),

        (Size::new(3, Multiple::Exbibyte), Ok(3_458_764_513_820_540_928)),
        (Size::new(2, Multiple::Exbibyte), Ok(2_305_843_009_213_693_952)),
        (Size::new(1, Multiple::Yobibyte), Ok(1_208_925_819_614_629_174_706_176)),
    ];

    for test in tests {
        let got: Result<u128, ConversionError> = test.0.clone().try_into();
        let want = test.1;
        assert_eq!(got, want, "input: {:?}", test.0);
    }
}

#[test]
fn size_from_str() {
    let tests = vec![
        ("100 B", Ok(Size::new(100, Multiple::Byte))),

        ("12 kB", Ok(Size::new(12, Multiple::Kilobyte))),
        ("25 MB", Ok(Size::new(25, Multiple::Megabyte))),
        ("1 GB", Ok(Size::new(1, Multiple::Gigabyte))),
        ("1000 TB", Ok(Size::new(1000, Multiple::Terabyte))),
        ("12 PB", Ok(Size::new(12, Multiple::Petabyte))),
        ("10 EB", Ok(Size::new(10, Multiple::Exabyte))),
        ("12 ZB", Ok(Size::new(12, Multiple::Zettabyte))),
        ("0 YB", Ok(Size::new(0, Multiple::Yottabyte))),

        ("99999 KB", Ok(Size::new(99999, Multiple::Kibibyte))),
        ("1 KiB", Ok(Size::new(1, Multiple::Kibibyte))),
        ("12 MiB", Ok(Size::new(12, Multiple::Mebibyte))),
        ("123 GiB", Ok(Size::new(123, Multiple::Gigibyte))),
        ("129 TiB", Ok(Size::new(129, Multiple::Tebibyte))),
        ("99 PiB", Ok(Size::new(99, Multiple::Pebibyte))),
        ("45 EiB", Ok(Size::new(45, Multiple::Exbibyte))),
        ("12 ZiB", Ok(Size::new(12, Multiple::Zebibyte))),
        ("2 YiB", Ok(Size::new(2, Multiple::Yobibyte))),

        ("", Err(ParsingError::NoValue)),
        ("10 abc", Err(ParsingError::UnknownMultiple)),
        ("10 B EXTRA", Err(ParsingError::UnknownExtra)),
    ];

    for test in tests {
        let got = Size::from_str(test.0);
        let want = test.1;
        assert_eq!(got, want, "input: {:?}", test.0);
    }
}

#[test]
fn size_equivalence() {
    let tests = vec![
        (Size::new(1, Multiple::Byte), Size::new(1, Multiple::Byte), true),
        (Size::new(1000, Multiple::Byte), Size::new(1, Multiple::Kilobyte), true),
        (Size::new(1024, Multiple::Byte), Size::new(1, Multiple::Kibibyte), true),
        (Size::new(1_000_000_000, Multiple::Byte), Size::new(1, Multiple::Gigabyte), true),
        (Size::new(1_073_741_824, Multiple::Byte), Size::new(1, Multiple::Gigibyte), true),

        (Size::new(1024, Multiple::Byte), Size::new(1, Multiple::Kilobyte), false),
        (Size::new(1000, Multiple::Byte), Size::new(1, Multiple::Kibibyte), false),
    ];

    for test in tests {
        let got = test.0 == test.1;
        let want = test.2;
        assert_eq!(got, want, "input: {:?} and {:?}", test.0, test.1);
    }
}

#[test]
fn size_comparing() {
    use std::cmp::Ordering::*;

    let tests = vec![
        (Size::new(1, Multiple::Byte), Size::new(1, Multiple::Byte), Some(Equal)),
        (Size::new(1000, Multiple::Byte), Size::new(1, Multiple::Kilobyte), Some(Equal)),
        (Size::new(1024, Multiple::Byte), Size::new(1, Multiple::Kibibyte), Some(Equal)),
        (Size::new(1_000_000_000, Multiple::Byte), Size::new(1, Multiple::Gigabyte), Some(Equal)),
        (Size::new(1_073_741_824, Multiple::Byte), Size::new(1, Multiple::Gigibyte), Some(Equal)),

        (Size::new(1, Multiple::Byte), Size::new(2, Multiple::Byte), Some(Less)),
        (Size::new(1024, Multiple::Byte), Size::new(1, Multiple::Kilobyte), Some(Greater)),
        (Size::new(1000, Multiple::Byte), Size::new(1, Multiple::Kibibyte), Some(Less)),
    ];

    for test in tests {
        let got = test.0.partial_cmp(&test.1);
        let want = test.2;
        assert_eq!(got, want, "input: {:?} and {:?}", test.0, test.1);
    }
}

#[test]
fn size_to_string() {
    let tests = vec![
        (Size::new(100, Multiple::Byte), "100 B"),

        (Size::new(2, Multiple::Kilobyte), "2 kB"),
        (Size::new(25, Multiple::Megabyte), "25 MB"),
        (Size::new(3, Multiple::Gigabyte), "3 GB"),
        (Size::new(38, Multiple::Terabyte), "38 TB"),
        (Size::new(100, Multiple::Zettabyte), "100 ZB"),

        (Size::new(2, Multiple::Mebibyte), "2 MiB"),
        (Size::new(3, Multiple::Zebibyte), "3 ZiB"),
        (Size::new(1000, Multiple::Yobibyte), "1000 YiB"),
    ];

    for test in tests {
        let got = test.0.to_string();
        let want = test.1;
        assert_eq!(got, want, "input: {:?}", test.0);
    }
}

#[test]
fn multiple_try_into_u32() {
    let tests = vec![
        (Multiple::Byte, Ok(1)),

        (Multiple::Kilobyte, Ok(1_000)),
        (Multiple::Megabyte, Ok(1_000_000)),
        (Multiple::Gigabyte, Ok(1_000_000_000)),

        (Multiple::Kibibyte, Ok(1024)),
        (Multiple::Mebibyte, Ok(1_048_576)),
        (Multiple::Gigibyte, Ok(1_073_741_824)),

        (Multiple::Terabyte, Err(ConversionError::Overflow)),
    ];

    for test in tests {
        let got: Result<u32, ConversionError> = test.0.try_into();
        let want = test.1;
        assert_eq!(got, want, "input: {:?}", test.0);
    }
}

#[test]
fn multiple_try_into_u64() {
    let tests = vec![
        (Multiple::Byte, Ok(1)),

        (Multiple::Kilobyte, Ok(1_000)),
        (Multiple::Megabyte, Ok(1_000_000)),
        (Multiple::Gigabyte, Ok(1_000_000_000)),
        (Multiple::Terabyte, Ok(1_000_000_000_000)),
        (Multiple::Petabyte, Ok(1_000_000_000_000_000)),

        (Multiple::Kibibyte, Ok(1024)),
        (Multiple::Mebibyte, Ok(1_048_576)),
        (Multiple::Gigibyte, Ok(1_073_741_824)),
        (Multiple::Tebibyte, Ok(1_099_511_627_776)),
        (Multiple::Pebibyte, Ok(1_125_899_906_842_624)),

        (Multiple::Exabyte, Err(ConversionError::Overflow)),
    ];

    for test in tests {
        let got: Result<u64, ConversionError> = test.0.try_into();
        let want = test.1;
        assert_eq!(got, want, "input: {:?}", test.0);
    }
}

#[test]
fn multiple_try_into_u128() {
    let tests = vec![
        (Multiple::Byte, Ok(1)),

        (Multiple::Kilobyte, Ok(1_000)),
        (Multiple::Megabyte, Ok(1_000_000)),
        (Multiple::Gigabyte, Ok(1_000_000_000)),
        (Multiple::Terabyte, Ok(1_000_000_000_000)),
        (Multiple::Petabyte, Ok(1_000_000_000_000_000)),
        (Multiple::Exabyte, Ok(1_000_000_000_000_000_000)),
        (Multiple::Zettabyte, Ok(1_000_000_000_000_000_000_000)),
        (Multiple::Yottabyte, Ok(1_000_000_000_000_000_000_000_000)),

        (Multiple::Kibibyte, Ok(1024)),
        (Multiple::Mebibyte, Ok(1_048_576)),
        (Multiple::Gigibyte, Ok(1_073_741_824)),
        (Multiple::Tebibyte, Ok(1_099_511_627_776)),
        (Multiple::Pebibyte, Ok(1_125_899_906_842_624)),
        (Multiple::Exbibyte, Ok(1_152_921_504_606_846_976)),
        (Multiple::Zebibyte, Ok(1_180_591_620_717_411_303_424)),
        (Multiple::Yobibyte, Ok(1_208_925_819_614_629_174_706_176)),
    ];

    for test in tests {
        let got: Result<u128, ConversionError> = test.0.try_into();
        let want = test.1;
        assert_eq!(got, want, "input: {:?}", test.0);
    }
}

#[test]
fn multiple_from_str() {
    let tests = vec![
        ("B", Ok(Multiple::Byte)),

        ("kB", Ok(Multiple::Kilobyte)),
        ("MB", Ok(Multiple::Megabyte)),
        ("GB", Ok(Multiple::Gigabyte)),
        ("TB", Ok(Multiple::Terabyte)),
        ("PB", Ok(Multiple::Petabyte)),
        ("EB", Ok(Multiple::Exabyte)),
        ("ZB", Ok(Multiple::Zettabyte)),
        ("YB", Ok(Multiple::Yottabyte)),

        ("KB", Ok(Multiple::Kibibyte)),
        ("KiB", Ok(Multiple::Kibibyte)),
        ("MiB", Ok(Multiple::Mebibyte)),
        ("GiB", Ok(Multiple::Gigibyte)),
        ("TiB", Ok(Multiple::Tebibyte)),
        ("PiB", Ok(Multiple::Pebibyte)),
        ("EiB", Ok(Multiple::Exbibyte)),
        ("ZiB", Ok(Multiple::Zebibyte)),
        ("YiB", Ok(Multiple::Yobibyte)),

        ("", Err(ParsingError::UnknownMultiple)),
        ("abc", Err(ParsingError::UnknownMultiple)),
    ];

    for test in tests {
        let got = Multiple::from_str(test.0);
        let want = test.1;
        assert_eq!(got, want, "input: {:?}", test.0);
    }
}

#[test]
fn multiple_to_string() {
    let tests = vec![
        (Multiple::Byte, "B"),

        (Multiple::Kilobyte, "kB"),
        (Multiple::Megabyte, "MB"),
        (Multiple::Gigabyte, "GB"),
        (Multiple::Terabyte, "TB"),
        (Multiple::Petabyte, "PB"),
        (Multiple::Exabyte, "EB"),
        (Multiple::Zettabyte, "ZB"),
        (Multiple::Yottabyte, "YB"),

        (Multiple::Kibibyte, "KiB"),
        (Multiple::Mebibyte, "MiB"),
        (Multiple::Gigibyte, "GiB"),
        (Multiple::Tebibyte, "TiB"),
        (Multiple::Pebibyte, "PiB"),
        (Multiple::Exbibyte, "EiB"),
        (Multiple::Zebibyte, "ZiB"),
        (Multiple::Yobibyte, "YiB"),
    ];

    for test in tests {
        let got = test.0.to_string();
        let want = test.1;
        assert_eq!(got, want, "input: {:?}", test.0);
    }
}

#[test]
fn parsing_error() {
    let invalid_value_err = u8::from_str_radix("INVALID", 10).unwrap_err();
    let tests = vec![
        (ParsingError::NoValue, "no value"),
        (ParsingError::InvalidValue(invalid_value_err), "invalid value"),
        (ParsingError::NoMultiple, "no multiple"),
        (ParsingError::UnknownMultiple, "unknown multiple"),
        (ParsingError::UnknownExtra, "unknown extra data"),
    ];

    for test in tests {
        let got = test.0.description();
        let want = test.1;
        assert_eq!(got, want, "input: {:?}", test.0);

        let got = test.0.to_string();
        assert_eq!(got, want, "input: {:?}", test.0);
    }
}

#[test]
fn conversion_error() {
    let tests = vec![
        (ConversionError::Overflow, "size overflows integer"),
    ];

    for test in tests {
        let got = test.0.description();
        let want = test.1;
        assert_eq!(got, want, "input: {:?}", test.0);

        let got = test.0.to_string();
        assert_eq!(got, want, "input: {:?}", test.0);
    }
}
