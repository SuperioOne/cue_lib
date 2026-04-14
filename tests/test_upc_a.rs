use cue_lib::{core::digit::Digits, discid::error::UpcParseErrorKind, discid::upc::UpcA};
use std::str::FromStr;

macro_rules! test_upc_a {
  ($test_name:ident, $digits:literal) => {
    #[test]
    fn $test_name() {
      match UpcA::from_str($digits) {
        Ok(upca) => {
          let digits = Digits::from_str(&$digits).unwrap();
          assert_eq!(upca.left_part(), &digits.as_bytes()[1..6]);
          assert_eq!(upca.right_part(), &digits.as_bytes()[6..11]);
          assert_eq!(upca.digit_system(), *digits.get(0).unwrap());
          assert_eq!(&upca.as_bytes(), digits.as_bytes());
          assert_eq!(&upca.to_string(), $digits);
          assert_eq!(&upca.as_ascii_bytes(), $digits.as_bytes());
        }
        Err(err) => assert!(false, "UPC-A parsing should've succeed, {:?}", err),
      }
    }
  };

  ($test_name:ident, $digits:literal, expects_err = $err:expr) => {
    #[test]
    fn $test_name() {
      match UpcA::from_str($digits) {
        Ok(_) => {
          assert!(false, "UPC-A parsing should've failed.")
        }
        Err(err) => assert_eq!(err.kind(), $err),
      }
    }
  };
}

test_upc_a!(from_str_123456789120, "123456789128");
test_upc_a!(from_str_000000000000, "000000000000");
test_upc_a!(from_str_111111111111, "111111111117");
test_upc_a!(from_str_999999999993, "999999999993");
test_upc_a!(from_str_760524681238, "760524681238");
test_upc_a!(from_str_546984834526, "546984834526");

test_upc_a!(
  invalid_cheksum_123456789121,
  "123456789121",
  expects_err = UpcParseErrorKind::ChecksumFail
);

test_upc_a!(
  invalid_char_abc000012345,
  "abc000012345",
  expects_err = UpcParseErrorKind::InvalidCharacter
);

test_upc_a!(
  invalid_checksumdigit_11100001234_,
  "11100001234_",
  expects_err = UpcParseErrorKind::InvalidCharacter
);

test_upc_a!(
  empty_str,
  "",
  expects_err = UpcParseErrorKind::InvalidLength
);

test_upc_a!(
  invalid_length_3,
  "123",
  expects_err = UpcParseErrorKind::InvalidLength
);

test_upc_a!(
  invalid_length_11,
  "54698483452",
  expects_err = UpcParseErrorKind::InvalidLength
);

test_upc_a!(
  invalid_length_13,
  "5469848345212",
  expects_err = UpcParseErrorKind::InvalidLength
);
