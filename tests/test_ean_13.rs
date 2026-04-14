use cue_lib::{core::digit::Digits, discid::ean::Ean13, discid::error::EanParseErrorKind};
use std::str::FromStr;

macro_rules! test_ean_13 {
  ($test_name:ident, $digits:literal) => {
    #[test]
    fn $test_name() {
      match Ean13::from_str($digits) {
        Ok(ean) => {
          let digits = Digits::from_str(&$digits).unwrap();
          assert_eq!(ean.gs1(), &digits.as_bytes()[0..3]);
          assert_eq!(ean.code(), &digits.as_bytes()[3..12]);
          assert_eq!(&ean.as_bytes(), digits.as_bytes());
          assert_eq!(&ean.to_string(), $digits);
          assert_eq!(&ean.as_ascii_bytes(), $digits.as_bytes());
        }
        Err(err) => assert!(false, "EAN-13 parsing should've succeed, {:?}", err),
      }
    }
  };

  ($test_name:ident, $digits:literal, expects_err = $err:expr) => {
    #[test]
    fn $test_name() {
      match Ean13::from_str($digits) {
        Ok(_) => {
          assert!(false, "EAN-13 parsing should've failed.")
        }
        Err(err) => assert_eq!(err.kind(), $err),
      }
    }
  };
}

test_ean_13!(from_str_1234567891286, "1234567891286");
test_ean_13!(from_str_0000000000000, "0000000000000");
test_ean_13!(from_str_1111111111116, "1111111111116");
test_ean_13!(from_str_9999999999994, "9999999999994");
test_ean_13!(from_str_7605246812382, "7605246812382");
test_ean_13!(from_str_5469848345264, "5469848345264");

test_ean_13!(
  invalid_cheksum_1234567891216,
  "1234567891216",
  expects_err = EanParseErrorKind::ChecksumFail
);

test_ean_13!(
  invalid_char_abc0000123458,
  "abc0000123458",
  expects_err = EanParseErrorKind::InvalidCharacter
);

test_ean_13!(
  invalid_checksumdigit_111000012345_,
  "111000012345_",
  expects_err = EanParseErrorKind::InvalidCharacter
);

test_ean_13!(
  empty_str,
  "",
  expects_err = EanParseErrorKind::InvalidLength
);

test_ean_13!(
  invalid_length_3,
  "123",
  expects_err = EanParseErrorKind::InvalidLength
);

test_ean_13!(
  invalid_length_12,
  "546984834520",
  expects_err = EanParseErrorKind::InvalidLength
);

test_ean_13!(
  invalid_length_14,
  "546984834521298",
  expects_err = EanParseErrorKind::InvalidLength
);
