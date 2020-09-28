//! ical-rs 0.4.0
//!
//! This is a library to parse the `ICalendar` format defined in
//! [RFC5545](http://tools.ietf.org/html/rfc5545), as well as similar formats
//! like `VCard`.
//!
//! There are probably some issues to be taken care of, but the library should work for most
//! cases. If you like to help out and
//! would like to discuss any API changes, please [contact me](dev@halium.fr) or create an issue.
//!
//! The initial goal was to make a port from the [ical.js](https://github.com/mozilla-comm/ical.js)
//! library in JavaScript and
//! many code/algorithms was taken from it but in order to but more 'Rusty' a complete rewrite
//! have been made.
//!
//! ## [Documentation](https://peltoche.github.io/ical-rs/ical/)
//!
//! ## Installing
//!
//! Put this in your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! ical = "0.4.*"
//! ```
//!
//! There is several ways to use crate::Ical depending on the level of parsing you want. Some new
//! wrapper/formater could appeare in the next releases.
//!
//! By default all the features are included but you can choose to include in you
//! project only the needed ones.
//!

#[macro_use]
extern crate thiserror;

const PARAM_VALUE_DELIMITER: char = ',';
const VALUE_DELIMITER: char = ':';
const PARAM_DELIMITER: char = ';';
const PARAM_NAME_DELIMITER: char = '=';
const PARAM_QUOTE: char = '"';

#[cfg(any(feature = "ical", feature = "vcard"))]
pub mod parser;

#[cfg(feature = "ical")]
pub use crate::parser::ical::IcalParser;

#[cfg(feature = "vcard")]
pub use crate::parser::vcard::VcardParser;

#[cfg(feature = "property")]
pub mod property;
#[cfg(feature = "property")]
pub use crate::property::PropertyParser;

#[cfg(feature = "line")]
pub mod line;
#[cfg(feature = "line")]
pub use crate::line::LineReader;
