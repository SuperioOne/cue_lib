use crate::{
  core::cue_str::CueStr,
  internal::tokenizer::{Token, Tokenizer},
  metadata::error::InvalidMetadataTagName,
};
use core::{cmp::Ordering, str::FromStr as _};

pub struct VorbisComment<'a> {
  pub tag: VorbisTagName,
  pub value: CueStr<'a>,
}

impl<'a> VorbisComment<'a> {
  pub fn try_from_line(line: &'a str) -> Result<Self, InvalidMetadataTagName> {
    if line.is_empty() {
      return Err(InvalidMetadataTagName);
    }

    let mut tokenizer = Tokenizer::new(line.trim());

    macro_rules! next_token {
      () => {
        tokenizer.next_token().map_err(|_| InvalidMetadataTagName)
      };
    }

    let tag_name = next_token!()?;
    let value = next_token!()?;

    // Trailing tokens other than LFs are now allowed
    if let Some(Token::Text { .. }) = next_token!()? {
      return Err(InvalidMetadataTagName);
    }

    match (tag_name, value) {
      (Some(Token::Text { value: tag }), Some(Token::Text { value })) => {
        let tag = match tag {
          CueStr::Text(v) => VorbisTagName::from_str(v),
          _ => Err(InvalidMetadataTagName),
        }?;

        Ok(VorbisComment { tag, value })
      }
      _ => Err(InvalidMetadataTagName),
    }
  }
}

#[inline]
fn cmp_ignore_ascii_case<'a, 'b>(a: &'a str, b: &'b str) -> Ordering {
  let a = a.as_bytes();
  let b = b.as_bytes();
  let mut idx = 0;

  if a.is_empty() && !b.is_empty() {
    return Ordering::Less;
  } else if b.is_empty() && !a.is_empty() {
    return Ordering::Greater;
  }

  loop {
    match (a.get(idx), b.get(idx)) {
      (None, None) => return Ordering::Equal,
      (None, Some(_)) => return Ordering::Less,
      (Some(_), None) => return Ordering::Greater,
      (Some(a), Some(b)) => {
        let lhs = a.to_ascii_lowercase();
        let rhs = b.to_ascii_lowercase();

        if lhs < rhs {
          return Ordering::Less;
        } else if lhs > rhs {
          return Ordering::Greater;
        }
      }
    };

    idx += 1;
  }
}

macro_rules! impl_vorbis_comment {
  ($type:ident, values = [
    $(($str_name:literal, $enum:ident)),+
  ]) => {

    #[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub enum $type {
      $($enum),+
    }

    static LOOKUP_TABLE: &[(&'static str, $type)] = &[$(($str_name, <$type>::$enum)),+];

    impl core::fmt::Display for $type {
      fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(self.as_str())
      }
    }

    impl $type {
      pub const fn as_str(&self) -> &'static str {
        match self {
          $(
            <$type>::$enum => $str_name,
          )+
        }
      }
    }

    impl core::str::FromStr for $type {
      type Err = $crate::metadata::error::InvalidMetadataTagName;

      fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
         return Err($crate::metadata::error::InvalidMetadataTagName);
        }

        match LOOKUP_TABLE.binary_search_by(|(name, _)| cmp_ignore_ascii_case(&name, s)) {
          Ok(idx) => match LOOKUP_TABLE.get(idx) {
            Some((_, value)) => Ok(*value),
            None => Err($crate::metadata::error::InvalidMetadataTagName)
          },
          Err(_) => Err($crate::metadata::error::InvalidMetadataTagName),
        }
      }
    }
  };
}

impl_vorbis_comment!(
  VorbisTagName,
  values = [
    ("ACOUSTID_FINGERPRINT", AcoustidFingerprint),
    ("ACOUSTID_ID", AcoustidId),
    ("ALBUM", Album),
    ("ALBUMARTIST", AlbumArtist),
    ("ALBUMARTISTSORT", AlbumArtistSort),
    ("ALBUMSORT", AlbumSort),
    ("ARRANGER", Arranger),
    ("ARTIST", Artist),
    ("ARTISTS", Artists),
    ("ARTISTSORT", ArtistSort),
    ("ASIN", Asin),
    ("BARCODE", Barcode),
    ("BPM", Bpm),
    ("CATALOGNUMBER", CatalogNumber),
    ("COMMENT", Comment),
    ("COMPILATION", Compilation),
    ("COMPOSER", Composer),
    ("COMPOSERSORT", ComposerSort),
    ("CONDUCTOR", Conductor),
    ("COPYRIGHT", Copyright),
    ("DATE", Date),
    ("DIRECTOR", Director),
    ("DISCNUMBER", DiscNumber),
    ("DISCSUBTITLE", DiscSubtitle),
    ("DISCTOTAL", TotalDiscs),
    ("DJMIXER", DjMixer),
    ("ENCODEDBY", EncodedBy),
    ("ENCODERSETTINGS", EncoderSettings),
    ("ENGINEER", Engineer),
    ("GENRE", Genre),
    ("GROUPING", Grouping),
    ("ISRC", Isrc),
    ("KEY", Key),
    ("LABEL", Label),
    ("LANGUAGE", Language),
    ("LICENSE", License),
    ("LYRICIST", Lyricist),
    ("LYRICS", Lyrics),
    ("MEDIA", Media),
    ("MIXER", Mixer),
    ("MOOD", Mood),
    ("MOVEMENT", MovementNumber),
    ("MOVEMENTNAME", Movement),
    ("MOVEMENTTOTAL", MovementTotal),
    ("ORIGINALDATE", OriginalDate),
    ("ORIGINALFILENAME", OriginalFilename),
    ("ORIGINALYEAR", OriginalYear),
    ("PERFORMER", Performer),
    ("PRODUCER", Producer),
    ("RATING", Rating),
    ("RELEASECOUNTRY", ReleaseCountry),
    ("RELEASESTATUS", ReleaseStatus),
    ("RELEASETYPE", ReleaseType),
    ("REMIXER", Remixer),
    ("REPLAYGAIN_ALBUM_GAIN", ReplaygainAlbumGain),
    ("REPLAYGAIN_ALBUM_PEAK", ReplaygainAlbumPeak),
    ("REPLAYGAIN_ALBUM_RANGE", ReplaygainAlbumRange),
    ("REPLAYGAIN_REFERENCE_LOUDNESS", ReplaygainReferenceLoudness),
    ("REPLAYGAIN_TRACK_GAIN", ReplaygainTrackGain),
    ("REPLAYGAIN_TRACK_PEAK", ReplaygainTrackPeak),
    ("REPLAYGAIN_TRACK_RANGE", ReplaygainTrackRange),
    ("SCRIPT", Script),
    ("SHOWMOVEMENT", ShowMovement),
    ("SUBTITLE", SubTitle),
    ("TITLE", Title),
    ("TITLESORT", TitleSort),
    ("TRACKNUMBER", TrackNumber),
    ("TRACKTOTAL", TotalTracks),
    ("WEBSITE", Website),
    ("WORK", Work),
    ("WRITER", Writer)
  ]
);

#[cfg(test)]
mod test {
  use crate::metadata::vorbis::{LOOKUP_TABLE, cmp_ignore_ascii_case};
  use core::cmp::Ordering;

  #[test]
  fn is_inner_table_sorted() {
    let is_sorted = LOOKUP_TABLE.is_sorted_by(|a, b| match a.0.cmp(b.0) {
      core::cmp::Ordering::Greater => false,
      _ => true,
    });

    assert!(
      is_sorted,
      "make sure the generated table is sorted on the code! I'm not planning to add fancy const time sorting, since tag list itself is not expected to be change."
    );
  }

  #[test]
  fn cmp_empty_strings() {
    assert_eq!(cmp_ignore_ascii_case("", ""), Ordering::Equal);
  }

  #[test]
  fn cmp_empty_string_against_non_empty() {
    assert_eq!(cmp_ignore_ascii_case("", "abc"), Ordering::Less);
  }

  #[test]
  fn cmp_non_empty_string_against_empty() {
    assert_eq!(cmp_ignore_ascii_case("abc", ""), Ordering::Greater);
  }

  #[test]
  fn cmp_equal_strings_case_insensitive() {
    assert_eq!(cmp_ignore_ascii_case("abc", "AbC"), Ordering::Equal);
  }

  #[test]
  fn cmp_greater_string_case_insensitive() {
    assert_eq!(cmp_ignore_ascii_case("abcd", "AbC"), Ordering::Greater);
  }

  #[test]
  fn cmp_less_string_case_insensitive() {
    assert_eq!(
      cmp_ignore_ascii_case("abcd_1234", "AbCF1234"),
      Ordering::Less
    );
  }

  #[test]
  fn cmp_single_char_equal() {
    assert_eq!(cmp_ignore_ascii_case("a", "A"), Ordering::Equal);
  }

  #[test]
  fn cmp_single_char_greater() {
    assert_eq!(cmp_ignore_ascii_case("b", "A"), Ordering::Greater);
  }

  #[test]
  fn cmp_single_char_less() {
    assert_eq!(cmp_ignore_ascii_case("a", "B"), Ordering::Less);
  }

  #[test]
  fn cmp_unicode_ascii_mixed() {
    assert_eq!(cmp_ignore_ascii_case("café", "Café"), Ordering::Equal);
  }

  #[test]
  fn cmp_special_characters() {
    assert_eq!(cmp_ignore_ascii_case("a!@#", "A!@#"), Ordering::Equal);
  }
}
