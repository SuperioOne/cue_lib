use cue_lib::discid::{
  error::*,
  isrc::{Alpha, AlphaNumeric, Country, Isrc, Owner, Serial, Year},
};
use std::str::FromStr;

#[test]
fn from_str() {
  match Isrc::from_str("TRCCC2400456") {
    Ok(isrc) => {
      assert_eq!(isrc.country.as_str(), "TR");
      assert_eq!(isrc.owner.as_str(), "CCC");
      assert_eq!(isrc.year.into_inner(), 24);
      assert_eq!(isrc.serial.into_inner(), 456);
    }
    Err(_) => assert!(false),
  }
}

#[test]
fn isrc_to_string() {
  let isrc = Isrc {
    serial: Serial::new(9999).unwrap(),
    year: Year::new(99).unwrap(),
    owner: Owner::new([
      AlphaNumeric::from_u8(b'z').unwrap(),
      AlphaNumeric::from_u8(b'1').unwrap(),
      AlphaNumeric::from_u8(b'z').unwrap(),
    ]),
    country: Country::new([Alpha::from_u8(b't').unwrap(), Alpha::from_u8(b'k').unwrap()]),
  };

  assert_eq!(isrc.country.as_str(), "TK");
  assert_eq!(isrc.owner.as_str(), "Z1Z");
  assert_eq!(isrc.year.into_inner(), 99);
  assert_eq!(isrc.serial.into_inner(), 9999);
  assert_eq!(isrc.to_string(), "TKZ1Z9909999");
}

#[test]
fn isrc_to_ascii_bytes() {
  let isrc = Isrc {
    serial: Serial::new(9999).unwrap(),
    year: Year::new(99).unwrap(),
    owner: Owner::new([
      AlphaNumeric::from_u8(b'z').unwrap(),
      AlphaNumeric::from_u8(b'1').unwrap(),
      AlphaNumeric::from_u8(b'Z').unwrap(),
    ]),
    country: Country::new([Alpha::from_u8(b't').unwrap(), Alpha::from_u8(b'k').unwrap()]),
  };

  assert_eq!(isrc.country.as_str(), "TK");
  assert_eq!(isrc.owner.as_str(), "Z1Z");
  assert_eq!(isrc.year.into_inner(), 99);
  assert_eq!(isrc.serial.into_inner(), 9999);
  assert_eq!(isrc.as_ascii_bytes().as_slice(), b"TKZ1Z9909999");
}

#[test]
fn invalid_length() {
  // lower range
  for len in 0..12 {
    let input = "A".to_owned().repeat(len);

    match Isrc::from_str(&input) {
      Ok(_) => assert!(false, "parsing should've failed, len: {}", len),
      Err(err) if err.kind() == IsrcParseErrorKind::InvalidLength => assert!(true),
      Err(err) => assert!(false, "error type is not correct, err {:?}", err),
    }
  }

  // upper range
  for len in 13..128 {
    let input = "A".to_owned().repeat(len);

    match Isrc::from_str(&input) {
      Ok(_) => assert!(false, "parsing should've failed, len: {}", len),
      Err(err) if err.kind() == IsrcParseErrorKind::InvalidLength => assert!(true),
      Err(err) => assert!(false, "error type is not correct, err {:?}", err),
    }
  }
}

#[test]
fn invalid_country_code() {
  for char_byte in 0u8..127u8 {
    match char_byte {
      b'A'..=b'Z' | b'a'..=b'z' => {
        continue;
      }
      _ => {
        let isrc = format!("{country}{country}ABC2500001", country = char_byte as char);

        match Isrc::from_str(&isrc) {
          Ok(_) => assert!(false, "parsing should've failed, isrc: {}", isrc),
          Err(err) if err.kind() == IsrcParseErrorKind::InvalidCountryCode => assert!(true),
          Err(err) => assert!(
            false,
            "error type is not correct, isrc: {}, err {:?}",
            isrc, err
          ),
        }
      }
    }
  }
}

#[test]
fn invalid_owner() {
  for char_byte in 0u8..127u8 {
    match char_byte {
      b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' => {
        continue;
      }
      _ => {
        let isrc = format!("IT{owner}{owner}{owner}2500001", owner = char_byte as char);

        match Isrc::from_str(&isrc) {
          Ok(_) => assert!(false, "parsing should've failed, isrc: {}", isrc),
          Err(err) if err.kind() == IsrcParseErrorKind::InvalidOwner => assert!(true),
          Err(err) => assert!(
            false,
            "error type is not correct, isrc: {}, err {:?}",
            isrc, err
          ),
        }
      }
    }
  }
}

#[test]
fn invalid_year() {
  for char_byte in 0u8..127u8 {
    match char_byte {
      b'0'..=b'9' => {
        continue;
      }
      _ => {
        let isrc = format!("ITC1T{year}{year}00001", year = char_byte as char);

        match Isrc::from_str(&isrc) {
          Ok(_) => assert!(false, "parsing should've failed, isrc: {}", isrc),
          Err(err) if err.kind() == IsrcParseErrorKind::InvalidYear => assert!(true),
          Err(err) => assert!(
            false,
            "error type is not correct, isrc: {}, err {:?}",
            isrc, err
          ),
        }
      }
    }
  }
}

#[test]
fn invalid_serial() {
  for char_byte in 0u8..127u8 {
    match char_byte {
      b'0'..=b'9' => {
        continue;
      }
      _ => {
        let isrc = format!(
          "ITC1T26{serial}",
          serial = (char_byte as char).to_string().repeat(5)
        );

        match Isrc::from_str(&isrc) {
          Ok(_) => assert!(false, "parsing should've failed, isrc: {}", isrc),
          Err(err) if err.kind() == IsrcParseErrorKind::InvalidSerial => assert!(true),
          Err(err) => assert!(
            false,
            "error type is not correct, isrc: {}, err {:?}",
            isrc, err
          ),
        }
      }
    }
  }
}
