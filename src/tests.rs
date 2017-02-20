use super::*;

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
