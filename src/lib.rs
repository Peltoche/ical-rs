//! # Ical 0.1.0
//!
//! This library is under heavy development. Many features are not finished.
//!
//! Vcard and Ical parser for Rust. It aims to be a feature-complete parser
//! all vcard and ical files.
//! * Ical-rs strictly adheres to rfc6350.
//! * Ical-rs handle Vcard version 3 and 4.
//!
//!
//! The initial goal was to make a porting of the
//! [mozilla parser](https://github.com/mozilla-comm/ical.js) from Javascript
//! to Rust. The main logic come from this codebase, but is adapted to more Rusty.
//!
//!
//!
//! ## Usage
//!
//! Put this in your `Cargo.toml`.
//! ```toml
//! [dependencies]
//! ical-rs = "0.1.0"
//! ```
//!
//! Or, if you want [rustc-serialize](https://github.com/rust-lang-nursery/rustc-serialize) support,
//! include the features like this:
//! ```toml
//! [dependencies]
//! ical-rs = { version = "0.1.0", features = ["rustc-serialize"] }
//! ```
//!
//!
//! Then put this in your crate root:
//!
//! ```rust
//! extern crate ical;
//! ```

extern crate rustc_serialize;

pub mod parser;
mod property;
mod value;
mod param;

use std::fmt;
use std::io;
use std::error::Error;

pub const VALUE_DELIMITER: char = ':';
pub const PARAM_DELIMITER: char = ';';
pub const PARAM_NAME_DELIMITER: char = '=';

/// The list of possible Error.
#[derive(Debug)]
pub enum ErrorKind {
    /// The given file cannot be open or read.
    File(io::Error),
    InvalidLineFormat,
    InvalidParamFormat,
    InvalidProperty,
    InvalidVersion,
    InvalidValueType,
    InvalidProtocol,
    NotImplemented,
    UnacceptedType,
}

/// An error from the parser.
#[derive(Debug)]
pub struct ParseError {
    kind: ErrorKind,
}

impl ParseError {
    pub fn new(kind: ErrorKind) -> ParseError {
        ParseError { kind: kind }
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.kind {
            ErrorKind::File(ref err) => err.fmt(f),
            _ => write!(f, "{}", self.description()),
        }
    }
}

impl Error for ParseError {
    fn description(&self) -> &str {
        match self.kind {
            ErrorKind::File(ref err) => err.description(),
            ErrorKind::InvalidLineFormat => "Invalid line format.",
            ErrorKind::InvalidParamFormat => "Invalid parameter format.",
            ErrorKind::InvalidProperty => "Invalid property.",
            ErrorKind::InvalidVersion => "Invalid version.",
            ErrorKind::InvalidValueType => "Invalid value type.",
            ErrorKind::InvalidProtocol => "Invalid protocol.",
            ErrorKind::NotImplemented => "Element not implemented.",
            ErrorKind::UnacceptedType => "Unaccepted type.",
        }
    }

    fn cause(&self) -> Option<&Error> {
        match self.kind {
            ErrorKind::File(ref err) => Some(err),
            _ => None,
        }
    }
}
