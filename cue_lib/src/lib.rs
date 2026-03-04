#![no_std]

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "alloc")]
pub mod parse;

#[cfg(feature = "metadata")]
pub mod metadata;

#[cfg(feature = "serde")]
mod serde;

mod internal;

pub mod core;
pub mod discid;
pub mod error;
pub mod probe;
