extern crate human_size;

use human_size::*;

#[test]
fn should_parse_sizes() {
    let tests = vec![
        ("100 B", Ok(Size::new(100, Multiple::Byte))),
        ("0 B", Ok(Size::new(0, Multiple::Byte))),

        ("12 kB", Ok(Size::new(12, Multiple::Kilobyte))),
        ("25 MB", Ok(Size::new(25, Multiple::Megabyte))),
        ("1 GB", Ok(Size::new(1, Multiple::Gigabyte))),
        ("1000 TB", Ok(Size::new(1000, Multiple::Terabyte))),
        ("12 PB", Ok(Size::new(12, Multiple::Petabyte))),
        //("10 EB", Ok(Size::new(10, Multiple::Exabyte))),
        //("12 ZB", Ok(Size::new(12, Multiple::Zettabyte))),
        //("0 YB", Ok(Size::new(0, Multiple::Yottabyte))),

        ("99999 KB", Ok(Size::new(99999, Multiple::Kibibyte))),
        ("1 KiB", Ok(Size::new(1, Multiple::Kibibyte))),
        ("12 MiB", Ok(Size::new(12, Multiple::Mebibyte))),
        ("123 GiB", Ok(Size::new(123, Multiple::Gigibyte))),
        ("129 TiB", Ok(Size::new(129, Multiple::Tebibyte))),
        ("99 PiB", Ok(Size::new(99, Multiple::Pebibyte))),
        //("45 EiB", Ok(Size::new(45, Multiple::Exbibyte))),
        //("12 ZiB", Ok(Size::new(12, Multiple::Zebibyte))),
        //("2 YiB", Ok(Size::new(2, Multiple::Yobibyte))),

        ("", Err(ParsingError::MissingValue)),
        ("10 abc", Err(ParsingError::InvalidMultiple)),
        ("10 B EXTRA", Err(ParsingError::UnknownExtra)),
    ];

    for test in tests {
        let got = test.0.parse();
        let want = match test.1 {
            Ok(size) => Ok(size.unwrap()),
            Err(err) => Err(err),
        };
        assert_eq!(got, want, "input: {:?}", test.0);
    }
}

#[test]
fn displaying_size() {
    let tests = vec![
        (Size::new(100, Multiple::Byte), "100 B"),

        (Size::new(2, Multiple::Kilobyte), "2 kB"),
        (Size::new(25, Multiple::Megabyte), "25 MB"),
        (Size::new(3, Multiple::Gigabyte), "3 GB"),
        (Size::new(38, Multiple::Terabyte), "38 TB"),
        //(Size::new(100, Multiple::Zettabyte), "100 ZB"),

        (Size::new(2, Multiple::Mebibyte), "2 MiB"),
        //(Size::new(3, Multiple::Zebibyte), "3 ZiB"),
        //(Size::new(1000, Multiple::Yobibyte), "1000 YiB"),
        //(Size::new(10.5, Multiple::Yobibyte), "10.5 YiB"),
    ];

    for test in tests {
        let got = test.0.unwrap().to_string();
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
        //(Size::new(1000, Multiple::Zettabyte), Size::new(1, Multiple::Yottabyte), true),
        //(Size::new(1024, Multiple::Zebibyte), Size::new(1, Multiple::Yobibyte), true),

        (Size::new(1024, Multiple::Byte), Size::new(1, Multiple::Kilobyte), false),
        (Size::new(1000, Multiple::Byte), Size::new(1, Multiple::Kibibyte), false),
    ];

    for test in tests {
        let size1 = test.0.unwrap();
        let size2 = test.1.unwrap();
        assert_eq!(size1 == size2, test.2, "input: {:?} and {:?}", size1, size2);
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
