use super::*;

#[test]
fn size_into_u64() {
    let tests = vec![
		(Size::new(1, Multiple::Byte), 1),

        (Size::new(1, Multiple::Kilobyte), 1_000),
        (Size::new(23, Multiple::Kilobyte), 23_000),
        (Size::new(65, Multiple::Megabyte), 65_000_000),
        (Size::new(123, Multiple::Gigabyte), 123_000_000_000),
        (Size::new(2, Multiple::Petabyte), 2_000_000_000_000_000),

        (Size::new(10, Multiple::Mebibyte), 10_485_760),
        (Size::new(1000, Multiple::Gigibyte), 1_073_741_824_000),
        (Size::new(1, Multiple::Pebibyte), 1_125_899_906_842_624),
        (Size::new(2, Multiple::Pebibyte), 2_251_799_813_685_248),
    ];

    for test in tests {
        let got: u64 = test.0.clone().into();
        let want = test.1;
        assert_eq!(got, want, "input: {:?}", test.0);
    }
}

#[test]
fn multiple_into_u64() {
    let tests = vec![
		(Multiple::Byte, 1),

		(Multiple::Kilobyte, 1_000),
		(Multiple::Megabyte, 1_000_000),
		(Multiple::Gigabyte, 1_000_000_000),
		(Multiple::Terabyte, 1_000_000_000_000),
		(Multiple::Petabyte, 1_000_000_000_000_000),

		(Multiple::Kibibyte, 1024),
		(Multiple::Mebibyte, 1_048_576),
		(Multiple::Gigibyte, 1_073_741_824),
		(Multiple::Tebibyte, 1_099_511_627_776),
		(Multiple::Pebibyte, 1_125_899_906_842_624),
    ];

    for test in tests {
        let got: u64 = test.0.into();
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
