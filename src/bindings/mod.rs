//! The reverse engineered binding protocol for the 8BitDo Micro.
//!
//! Bindings move in two groups of 8 buttons. [`read`] sends the read commands
//! and parses the notifications that come back; [`write`] patches new key codes
//! into the write commands and sends them, followed by a sync. The raw command
//! bytes live in [`commands`], and the small parsing and patching helpers in
//! [`utils`].

pub mod commands;
pub mod read;
pub mod reset;
pub mod types;
pub mod utils;
pub mod write;
