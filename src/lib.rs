#![no_std]

//! # cue_lib
//!
//! Simple cue sheet parsing library with `no_std` by default.
//!
//! cue_lib is mainly based on original CDRWIN[^cdrwin] definition with some minor rule changes:
//!
//! - `TRACK` and `INDEX` number limit is increased to 255 instead of 99. (wasting bits? in this
//! economy!?)
//! - `POSTGAP` and `PREGAP` commands can appear in any order on a `TRACK`.
//! - `ISRC` command can appear before/after/in between `INDEX` commands.
//! - `FILE` command can appear after other commands, it only required to appear before
//! any `TRACK` command.
//! - `CATALOG` command is not limited to 13-digit UPC/EAN format, any valid
//! [`CueStr`]( `core::cue_str::CueStr`) is accepted.
//! - When [`metadata`] feature is enabled, `REM` with Vorbis comments can be read as additional
//! metadata for the both album and tracks.
//!
//! <div class="warning">
//!
//! cue_lib does not support EAC's cue sheet with multiple `FILE` commands.
//!
//! </div>
//!
//!
//! [^cdrwin]: <https://web.archive.org/web/20151023011544/http://digitalx.org/cue-sheet/syntax/index.html>
//!
//! ## Feature flags
//!
//! | Feature Name         | Description|
//! |--------------|------------------------------------------------------|
//! | **default**  | Does not enable any additional feature (no_std)|
//! | **alloc**    | Enables modules/types that uses dynamic memory allocation (eg. [`parse`])|
//! | **metadata** | Enables reading remarks as Vorbis comment, and provides helper types for ID3, Vorbis, and libav.|
//! | **serde**    | Enables [serde](https://serde.rs/) only for type **serialization**. |
//!
//!
//! ## Basic examples
//!
//! ### Parse
//!
//! <div class="warning">
//!
//! Requires **`alloc`** feature
//!
//! </div>
//!
//! Parses cuesheet fully and stores it's data in dynamically allocated vectors.
//!
//!
//! ### Probe
//!
//! Low level API without any dynamic memory allocation. It basically attaches to the cuesheet
//! and allows progressively reading its content.
//!
//! For more detailed explanation see [`probe`] module.
//!
//! ```
//!   use cue_lib::probe::CuesheetProbe;
//!   use cue_lib::error::CueLibError;
//!
//!   let cuesheet = r#"
//!      PERFORMER "Rick Astley"
//!      TITLE "Whenever You Need Somebody"
//!      FILE "rick_astley_whenever_you_need_somebody.flac" WAVE
//!        TRACK 01 AUDIO
//!          TITLE "Never Gonna Give You Up"
//!          PERFORMER "Rick Astley"
//!          INDEX 00 00:00:00
//!          INDEX 01 00:03:32
//!          ISRC GBAYE0100001
//!        TRACK 02 AUDIO
//!          TITLE "Never Gonna Let You Down"
//!          PERFORMER "Rick Astley"
//!          INDEX 01 03:32:00
//!          INDEX 02 04:32:00
//!          ISRC GBAYE0100002
//!        TRACK 03 AUDIO
//!          TITLE "Never Gonna Run Around and Desert You"
//!          PERFORMER "Rick Astley"
//!          INDEX 01 07:04:00
//!          ISRC GBAYE0100003
//!   "#;
//!
//!   fn print_cue(cuesheet: &str) -> Result<(), CueLibError> {
//!    let probe = CuesheetProbe::new(cuesheet)?;
//!
//!    if let Some(title) = probe.album_title() {
//!      println!("Title: {}", title);
//!    }
//!
//!    if let Some(performer) = probe.performer() {
//!      println!("Performer: {}", performer);
//!    }
//!
//!    let mut tracks = probe.tracks();
//!
//!    while let Some(track) = tracks.next_track()? {
//!      print!("* Track - {}", track.track_no());
//!
//!      if let Some(track_title) = track.title() {
//!        println!(" {}", track_title);
//!      }
//!
//!      // INDEX 00 (optional) often represent the silence at the start of the song.
//!      if let Some(index_zero) = track.pregap_index() {
//!        println!("    Index 00 at {} sec", index_zero.as_duration().as_secs());
//!      }
//!
//!      // INDEX 01 (required) actual start of the track.
//!      println!(
//!        "    Index 01 at {} sec",
//!        track.start_index().as_duration().as_secs()
//!      );
//!
//!      // To access other sub-indexes INDEX 02, 03 etc.
//!      let mut indexes = track.sub_indexes();
//!
//!      while let Some(sub_index) = indexes.next_index()? {
//!        println!(
//!          "    Index {} starts at {} sec",
//!          sub_index.no,
//!          sub_index.timestamp.as_duration().as_secs()
//!        );
//!      }
//!    }
//!
//!     Ok(())
//!   }
//!
//!   print_cue(cuesheet);
//! ```

#[cfg(feature = "alloc")]
extern crate alloc;

mod internal;

#[cfg(feature = "alloc")]
pub mod parse;

#[cfg(feature = "metadata")]
pub mod metadata;

pub mod core;
pub mod discid;
pub mod error;
pub mod probe;
