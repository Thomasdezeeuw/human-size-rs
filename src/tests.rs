use super::*;

#[test]
fn multiple_into_u64() {
    let tests = vec![
		(Multiple::Byte, 1),

		(Multiple::Kilobyte, 1000^1),
		(Multiple::Megabyte, 1000^2),
		(Multiple::Gigabyte, 1000^3),
		(Multiple::Terabyte, 1000^4),
		(Multiple::Petabyte, 1000^5),

		(Multiple::Kibibyte, 1024^1),
		(Multiple::Mebibyte, 1024^2),
		(Multiple::Gigibyte, 1024^3),
		(Multiple::Tebibyte, 1024^4),
		(Multiple::Pebibyte, 1024^5),
    ];

    for test in tests {
        let got: u64 = test.0.into();
        let want = test.1;
        assert_eq!(got, want, "input: {:?}", test.0);
    }
}
