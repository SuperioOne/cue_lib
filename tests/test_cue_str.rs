use cue_lib::core::{CueStr, error::CueStrErrorKind};

macro_rules! test_cue_str {
  ($test_name:ident, $str:literal, expects = $cmp:expr) => {
    #[test]
    fn $test_name() {
      match CueStr::try_from_raw_str($str) {
        Ok(cue_str) => {
          assert_eq!(cue_str, $cmp);
          assert_eq!(cue_str.to_string(), $cmp);
        }
        Err(err) => assert!(false, "CueStr convertion should've succeed, {:?}", err),
      }
    }
  };

  ($test_name:ident, $str:literal, expects_err = $err:expr) => {
    #[test]
    fn $test_name() {
      match CueStr::try_from_raw_str($str) {
        Ok(_) => {
          assert!(false, "CueStr should've failed.")
        }
        Err(err) => assert_eq!(err.kind(), $err),
      }
    }
  };
}

#[test]
fn eq_empty_str() {
  let cuestr = CueStr::try_from_raw_str("");
  assert!(cuestr.is_ok());
  assert_eq!(cuestr.unwrap(), "");
}

#[test]
fn eq_quoted_empty_str() {
  let cuestr = CueStr::try_from_raw_str("\"\"");
  assert!(cuestr.is_ok());
  assert_eq!(cuestr.unwrap(), "");
}

#[test]
fn eq_str() {
  let cuestr = CueStr::try_from_raw_str("HelloDarkness,MyOldFriend");
  assert!(cuestr.is_ok());
  assert_eq!(cuestr.unwrap(), "HelloDarkness,MyOldFriend");
}

#[test]
fn eq_quoted_str() {
  let cuestr = CueStr::try_from_raw_str("\" I've come to talk with you again. \"");
  assert!(cuestr.is_ok());
  assert_eq!(cuestr.unwrap(), " I've come to talk with you again. ");
}

#[test]
fn eq_escaped_quoted_str() {
  let cuestr = CueStr::try_from_raw_str("\"\\\"Because\\\" a vision softly creeping\"");
  assert!(cuestr.is_ok());
  assert_eq!(cuestr.unwrap(), "\"Because\" a vision softly creeping");
}

#[test]
fn not_eq_str() {
  let cuestr = CueStr::try_from_raw_str("NeverGonnaGiveYouUP");
  assert!(cuestr.is_ok());
  assert_ne!(cuestr.unwrap(), "NeverGonnaGiveYou");
}

#[test]
fn not_eq_quoted_str() {
  let cuestr = CueStr::try_from_raw_str("\"Never gonna let you down\"");
  assert!(cuestr.is_ok());
  assert_ne!(cuestr.unwrap(), "Never GONNA LET YOU DOWN");
}

#[test]
fn not_eq_escaped_quoted_str() {
  let cuestr =
    CueStr::try_from_raw_str("\"Never gonna \\\\run\\\\ \\\"around\\\" and desert you\"");
  assert!(cuestr.is_ok());
  assert_ne!(cuestr.unwrap(), "Never gonna \\run\\ around and desert you");
}

#[cfg(feature = "alloc")]
#[test]
fn as_cow_escaped() {
  let cuestr = CueStr::try_from_raw_str("\"Never gonna \\\\make\\\\ \\\"you\\\" cry\"");
  assert!(cuestr.is_ok());
  assert_eq!(
    cuestr.unwrap().as_cow_str(),
    "Never gonna \\make\\ \"you\" cry"
  );
}

#[cfg(feature = "alloc")]
#[test]
fn as_cow_quoted() {
  let cuestr = CueStr::try_from_raw_str("\"Never gonna say goodbye\"");

  assert!(cuestr.is_ok());
  assert_eq!(cuestr.unwrap().as_cow_str(), "Never gonna say goodbye");
}

test_cue_str!(empty, "", expects = "");
test_cue_str!(empty_quoted, "\"\"", expects = "");
test_cue_str!(whitespace_quoted, "\"     \"", expects = "     ");
test_cue_str!(non_quoted_ascii, "hello_world", expects = "hello_world");

test_cue_str!(
  non_quoted_utf8,
  "ハローワールド",
  expects = "ハローワールド"
);

test_cue_str!(
  quoted_utf8,
  "\"ハローワールド\"",
  expects = "ハローワールド"
);

test_cue_str!(quoted_ascii, "\"hello_world\"", expects = "hello_world");

test_cue_str!(
  only_with_ending_quote,
  "hello_world\"",
  expects = "hello_world\""
);

test_cue_str!(
  quoted_ascii_with_whitespaces,
  "\" hello  world \"",
  expects = " hello  world "
);

test_cue_str!(
  quoted_utf8_with_whitespaces,
  "\"古池や蛙飛び込む水の音\n\tふるいけやかわずとびこむみずのおと\"",
  expects = "古池や蛙飛び込む水の音\n\tふるいけやかわずとびこむみずのおと"
);

test_cue_str!(
  quoted_with_special_quote,
  "\"hello  \\\"hell\\\"\"",
  expects = "hello  \"hell\""
);

test_cue_str!(
  quoted_with_special_backslash,
  "\"hello  \\\\\\\"hell\\\"\"",
  expects = "hello  \\\"hell\""
);

test_cue_str!(
  missing_ending_quote,
  "\"hell-oh",
  expects_err = CueStrErrorKind::MissingEndingQuote
);

test_cue_str!(
  missing_quotes,
  "Hecatia, my beloved",
  expects_err = CueStrErrorKind::MissingQuotes
);

test_cue_str!(
  unescaped_backslash,
  "\"Hecatia\\, my beloved\"",
  expects_err = CueStrErrorKind::UnescapedSpecialChar
);

test_cue_str!(
  unescaped_quote,
  "\"Hecatia, my\" beloved\"",
  expects_err = CueStrErrorKind::UnescapedSpecialChar
);
