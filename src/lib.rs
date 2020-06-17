//! Iterators over text in morse code.
//!
//! ```rust
//! fn wait_for(duration: u8) {
//!     // ...
//! }
//! fn beep_for(duration: u8) {
//!     // ...
//! }
//!
//! for action in morse_code::encode("Hello in morse code!") {
//!     if action.state == morse_code::State::On {
//!         beep_for(action.duration);
//!     } else {
//!         wait_for(action.duration);
//!     }
//! }
//! ```
//!
//! This library is for encoding text into morse code (not the other way around yet).
//!
//! It works without the standard library.

#![cfg_attr(not(test), no_std)]
#![warn(
    missing_docs,
    missing_debug_implementations,
    trivial_casts,
    trivial_numeric_casts,
    unused_import_braces,
    unused_qualifications
)]

#[cfg(test)]
extern crate core;

pub(crate) mod char_map;
mod morse_iter;

pub use morse_iter::{Action, DelayType, MorseIter, State};

/// Creates an iterator over the `Action`s necessary to send the message
pub fn encode(message: &str) -> MorseIter<'_> {
    MorseIter::new(message)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Signal {
    Dot,
    Dash,
}
