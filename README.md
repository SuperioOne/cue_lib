# cue_lib

Simple cue sheet parsing library with `no_std` by default.

cue_lib is mainly based on original [CDRWIN](https://web.archive.org/web/20151023011544/http://digitalx.org/cue-sheet/syntax/index.html) 
document with some relaxed requirements:

- The limit for `TRACK` and `INDEX` numbers has been increased to 255 instead of 99 *(wasting bits? in this
economy!?)*
- `POSTGAP` and `PREGAP` commands can appear in any order within a `TRACK`.
- `ISRC` command can appear before, after, in between `INDEX` commands.
- The `FILE` command must appear appear before any `TRACK` command but can appear anywhere
else.
- The `CATALOG` command is not limited to 13-digit UPC/EAN format, any valid
`CueStr` is accepted.
- When `metadata` feature is enabled, `REM` with Vorbis comments can be read as additional
metadata for both album and tracks.
- FLAC is allowed in the `FILE` command as a file type.

> cue_lib does not support EAC's cue sheet with multiple `FILE` commands.


## Feature flags

| Feature Name             | Description                                                                                       |
| ------------------------ | ------------------------------------------------------------------------------------------------- |
| **default**              | Does not enable any additional feature (no_std)                                                   |
| **alloc**                | Enables modules/types that uses dynamic memory allocation (e.g. `parse`)                        |
| **metadata**             | Enables reading remarks as Vorbis comment, and provides helper types for ID3, Vorbis, and libav.  |
| **serde**                | Enables **serde** only for type **serialization**.                                                |
| **ean**                  | Provides `Ean13` utility type to parse 13 digits EAN-13 catalog codes.                            |
| **upc**                  | Provides `UpcA` utility type to parse 12 digits UPC-A catalog codes.                              |

## Example - CuesheetParser

> Requires **`alloc`** feature

```rust
 use cue_lib::error::CueLibError;
 use cue_lib::parse::CuesheetParser;

 let cuesheet = r#"
     PERFORMER "Rick Astley"
     TITLE "Whenever You Need Somebody"
     FILE "rick_astley_whenever_you_need_somebody.flac" WAVE
       TRACK 01 AUDIO
         TITLE "Never Gonna Give You Up"
         PERFORMER "Rick Astley"
         INDEX 00 00:00:00
         INDEX 01 00:03:32
         ISRC GBAYE0100001
       TRACK 02 AUDIO
         TITLE "Never Gonna Let You Down"
         PERFORMER "Rick Astley"
         INDEX 01 03:32:00
         INDEX 02 04:32:00
         ISRC GBAYE0100002
       TRACK 03 AUDIO
         TITLE "Never Gonna Run Around and Desert You"
         PERFORMER "Rick Astley"
         INDEX 01 07:04:00
         ISRC GBAYE0100003
  "#;

 fn print_cue(cuesheet_str: &str) -> Result<(), CueLibError> {
   let cuesheet = CuesheetParser::new().parse(cuesheet_str)?;

   if let Some(title) = cuesheet.album_title {
     println!("Title: {}", title);
   }

   if let Some(performer) = cuesheet.performer {
     println!("Performer: {}", performer);
   }

   for track in cuesheet.tracks.iter() {
     print!("* Track - {}", track.no);

     if let Some(track_title) = track.title {
       println!(" {}", track_title);
     }

     // Calculated track duration, only the last track's duration is `None` since cue_lib
     // does not check FILE duration.
     if let Some(track_duration) = track.time_info.duration {
       println!("    Duration {}s", track_duration.as_duration().as_secs());
     }

     // INDEX 00 (optional) often represent the silence at the start of the song.
     if let Some(index_zero) = track.time_info.pregap_start {
       println!("    Index 00 at {}s", index_zero.as_duration().as_secs());
     }

     // INDEX 01 (required) actual start of the track.
     println!(
       "    Index 01 at {}s ",
       track.time_info.start.as_duration().as_secs()
     );

     // To access other sub-indexes INDEX 02, 03 etc.
     for sub_index in track.sub_indexes.iter() {
       println!(
         "    Index {} starts at {}s",
         sub_index.no,
         sub_index.timestamp.as_duration().as_secs()
       );
     }
   }

   Ok(())
 }

 print_cue(cuesheet);
```

## Example - CuesheetProbe

Low level API without any dynamic memory allocation. It basically attaches to cuesheet
and allows progressively reading its content.

For more detailed explanation see `probe` module.

```rust
  use cue_lib::probe::CuesheetProbe;
  use cue_lib::error::CueLibError;

  let cuesheet = r#"
     PERFORMER "Rick Astley"
     TITLE "Whenever You Need Somebody"
     FILE "rick_astley_whenever_you_need_somebody.flac" WAVE
       TRACK 01 AUDIO
         TITLE "Never Gonna Give You Up"
         PERFORMER "Rick Astley"
         INDEX 00 00:00:00
         INDEX 01 00:03:32
         ISRC GBAYE0100001
       TRACK 02 AUDIO
         TITLE "Never Gonna Let You Down"
         PERFORMER "Rick Astley"
         INDEX 01 03:32:00
         INDEX 02 04:32:00
         ISRC GBAYE0100002
       TRACK 03 AUDIO
         TITLE "Never Gonna Run Around and Desert You"
         PERFORMER "Rick Astley"
         INDEX 01 07:04:00
         ISRC GBAYE0100003
  "#;

  fn print_cue(cuesheet: &str) -> Result<(), CueLibError> {
   let probe = CuesheetProbe::new(cuesheet)?;

   if let Some(title) = probe.album_title() {
     println!("Title: {}", title);
   }

   if let Some(performer) = probe.performer() {
     println!("Performer: {}", performer);
   }

   let mut tracks = probe.tracks();

   while let Some(track) = tracks.next_track()? {
     print!("* Track - {}", track.track_no());

     if let Some(track_title) = track.title() {
       println!(" {}", track_title);
     }

     // INDEX 00 (optional) often represent the silence at the start of the song.
     if let Some(index_zero) = track.pregap_index() {
       println!("    Index 00 at {} sec", index_zero.as_duration().as_secs());
     }

     // INDEX 01 (required) actual start of the track.
     println!(
       "    Index 01 at {} sec",
       track.start_index().as_duration().as_secs()
     );

     // To access other sub-indexes INDEX 02, 03 etc.
     let mut indexes = track.sub_indexes();

     while let Some(sub_index) = indexes.next_index()? {
       println!(
         "    Index {} starts at {} sec",
         sub_index.no,
         sub_index.timestamp.as_duration().as_secs()
       );
     }
   }

    Ok(())
  }

  print_cue(cuesheet);
```
