use cue_lib::core::{CueTimestamp, Frame, Second, error::TimestampParseErrorKind};
use std::str::FromStr;

macro_rules! test_timestamp {
  ($test_name:ident, $timestamp:literal, milli_seconds = $ms:expr ) => {
    #[test]
    fn $test_name() {
      match CueTimestamp::from_str($timestamp) {
        Ok(timestamp) => {
          assert_eq!(timestamp.as_millis(), $ms);
        }
        Err(err) => {
          assert!(false, "timestamp parsing should've succeed, {:?}", err);
        }
      }
    }
  };

  ($test_name:ident, $timestamp:literal, expects_err = $err:expr ) => {
    #[test]
    fn $test_name() {
      match CueTimestamp::from_str($timestamp) {
        Ok(_) => {
          assert!(
            false,
            "timestamp parsing should've failed, input: {}",
            $timestamp
          );
        }
        Err(err) => {
          assert_eq!(err.kind(), $err);
        }
      }
    }
  };
}

#[test]
fn duration_conversion() {
  let timestamp = CueTimestamp::new(12, Second::new(34).unwrap(), Frame::new(74).unwrap());

  assert_eq!(timestamp.as_duration().as_millis(), timestamp.as_millis());
  assert_eq!(timestamp.to_string(), "12:34:74");
}

test_timestamp!(timestamp_00_00_00, "00:00:00", milli_seconds = 0);
test_timestamp!(timestamp_01_00_00, "01:00:00", milli_seconds = 60_000);
test_timestamp!(timestamp_00_01_00, "00:01:00", milli_seconds = 1_000);
test_timestamp!(timestamp_00_00_01, "00:00:01", milli_seconds = 1000 / 75);

test_timestamp!(
  timestamp_6_32_00,
  "6:32:00",
  milli_seconds = ((6 * 60) + 32) * 1000
);

test_timestamp!(
  timestamp_06_32_00,
  "06:32:00",
  milli_seconds = ((6 * 60) + 32) * 1000
);

test_timestamp!(
  timestamp_123_59_74,
  "123:59:74",
  milli_seconds = ((123 * 60) + 59) * 1000 + (1000 / 75) * 74
);

test_timestamp!(
  empty,
  "",
  expects_err = TimestampParseErrorKind::InvalidLength
);
test_timestamp!(
  invalid_length,
  "0:0:00",
  expects_err = TimestampParseErrorKind::InvalidLength
);

test_timestamp!(
  invalid_second_60,
  "00:60:00",
  expects_err = TimestampParseErrorKind::InvalidSecond
);

test_timestamp!(
  invalid_frame_75,
  "00:00:75",
  expects_err = TimestampParseErrorKind::InvalidFrame
);

test_timestamp!(
  invalid_second_99,
  "00:99:00",
  expects_err = TimestampParseErrorKind::InvalidSecond
);

test_timestamp!(
  invalid_frame_99,
  "00:00:99",
  expects_err = TimestampParseErrorKind::InvalidFrame
);

test_timestamp!(
  invalid_second_ab,
  "00:ab:00",
  expects_err = TimestampParseErrorKind::InvalidSecond
);

test_timestamp!(
  invalid_frame_ab,
  "00:00:ab",
  expects_err = TimestampParseErrorKind::InvalidFrame
);

test_timestamp!(
  invalid_minute_abc,
  "abc:00:00",
  expects_err = TimestampParseErrorKind::InvalidMinute
);

test_timestamp!(
  invalid_character,
  "00_00:00",
  expects_err = TimestampParseErrorKind::InvalidCharacter
);

test_timestamp!(
  invalid_characters,
  "000_00_00",
  expects_err = TimestampParseErrorKind::InvalidCharacter
);
