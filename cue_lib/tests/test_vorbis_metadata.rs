#![cfg(feature = "metadata")]

use cue_lib::metadata::{MetadataTag, error::InvalidMetadataTagName, vorbis::VorbisComment};

macro_rules! test_vorbis_comment {
  ($test_name:ident, $str:literal, expects_tag = $cmp:expr, expects_value = $value:expr) => {
    #[test]
    fn $test_name() {
      match VorbisComment::try_from_line($str) {
        Ok(vorbis_comment) => {
          assert_eq!(vorbis_comment.tag, $cmp);
          assert_eq!(vorbis_comment.value, $value);
        }
        Err(err) => assert!(false, "VorbisComment parse should've succeed, {:?}", err),
      }
    }
  };

  ($test_name:ident, $str:literal, expects_err = $err:expr) => {
    #[test]
    fn $test_name() {
      match VorbisComment::try_from_line($str) {
        Ok(_) => {
          assert!(false, "VorbisComment should've failed.")
        }
        Err(err) => assert_eq!(err, $err),
      }
    }
  };
}

test_vorbis_comment!(
  artist_test,
  "ARTIST めらみぽっぷ;天舞音叫子;",
  expects_tag = MetadataTag::Artist,
  expects_value = "めらみぽっぷ;天舞音叫子;"
);

test_vorbis_comment!(
  mixed_whitespace,
  "GENRE              \"Rock; MetalCore;\"                 \t\t\t\t\t \n",
  expects_tag = MetadataTag::Genre,
  expects_value = "Rock; MetalCore;"
);

test_vorbis_comment!(
  tab_and_line_feed,
  "ALBUMARTIST\t\"かちかち山\"\n\n",
  expects_tag = MetadataTag::AlbumArtist,
  expects_value = "かちかち山"
);

test_vorbis_comment!(
  arranger_test,
  "ARRANGER \"\\\"ARI2\"",
  expects_tag = MetadataTag::Arranger,
  expects_value = "\"ARI2"
);

test_vorbis_comment!(
  intentional_empty_value,
  "ALBUM \"\"",
  expects_tag = MetadataTag::Album,
  expects_value = ""
);

test_vorbis_comment!(empty_value, "ALBUM", expects_err = InvalidMetadataTagName);

test_vorbis_comment!(
  unknown_tag,
  "UNKNOWNTAG \"Some Value\"",
  expects_err = InvalidMetadataTagName
);

test_vorbis_comment!(
  no_space,
  "TITLE鶏鳴以前の事",
  expects_err = InvalidMetadataTagName
);

test_vorbis_comment!(
  invalid_line_feed,
  "COMMENT\n\"Stack My Beloved ❤️\"",
  expects_err = InvalidMetadataTagName
);

test_vorbis_comment!(empty_line, "", expects_err = InvalidMetadataTagName);

test_vorbis_comment!(
  invalid_quote,
  "ALBUM \"Inevitably Delayed",
  expects_err = InvalidMetadataTagName
);
test_vorbis_comment!(
  unquoted_multi_words,
  "ALBUM Inevitably Delayed",
  expects_err = InvalidMetadataTagName
);

test_vorbis_comment!(
  invalid_escape_sequence,
  "ALBUMARTIST \"\\The Prostitues\"",
  expects_err = InvalidMetadataTagName
);
