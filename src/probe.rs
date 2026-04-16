//! # How it's works
//!
//! CuesheetProbe progressively parses the cue sheet by logical sections.
//!
//! Initially, it parses only the album portion of the cue sheet, which includes
//! all commands from the beginning of the file up to the first `TRACK` command.
//! The probe itself does not allocate any dynamic memory; it simply keeps substring
//! references from the attached cue sheet.
//!
//! Track blocks can be accessed via `.tracks()`, and the track probe then progressively
//! parses track blocks using `.next_track()` calls until end of file is reached.
//!
//!```markdown, ignore
//!   ┌─ REM GENRE Dance-Pop                                     
//!   │  REM GENRE Pop                                           
//!   │  REM DATE 1987-10-16                                     
//! ┌─┤  PERFORMER "Rick Astley"                                 
//! │ │  TITLE "Whenever You Need Somebody"                      
//! │ └─ FILE "rick_astley_whenever_you_need_somebody.flac" WAVE
//! │   ┌─ TRACK 01 AUDIO                                        
//! │   │    TITLE "Never Gonna Give You Up"                     
//! │ ┌─┤    PERFORMER "Rick Astley"                             
//! │ │ │    INDEX 00 00:00:00                                   
//! │ │ │    INDEX 01 00:03:32                                   
//! │ │ └─   ISRC GBAYE0100001                                   
//! │ │ ┌─ TRACK 02 AUDIO                                        
//! │ │ │    TITLE "Never Gonna Let You Down"                    
//! │ │ │    PERFORMER "Rick Astley"                             
//! │ ├─┤    INDEX 01 03:32:00                                   
//! │ │ │    INDEX 02 04:32:00                                   
//! │ │ └─   ISRC GBAYE0100002                                   
//! │ │ ┌─ TRACK 03 AUDIO                                        
//! │ │ │    TITLE "Never Gonna Run Around and Desert You"       
//! │ ├─│    PERFORMER "Rick Astley"                             
//! │ │ │    INDEX 01 07:04:00                                   
//! │ │ └─   ISRC GBAYE0100003                                   
//! │ │                                                          
//! │ └─────────────────────────────────────► TrackProbes, can be accesed via probe.tracks()
//! └───────────────────────────────────────► CuesheetProbe, all album related commands
//! ```

mod builder;
mod cuesheet;

/// Remark iterator
pub mod remark;
/// Track probe
pub mod track;
/// Vorbis remark iterator
#[cfg(feature = "metadata")]
pub mod vorbis_remark;

pub use cuesheet::CuesheetProbe;
