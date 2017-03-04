//! ical-rs 0.3.0
//!
//! This is a library to parse the ICalendar format defined in
//! [RFC5545](http://tools.ietf.org/html/rfc5545), as well as similar formats
//! like VCard.
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
//! ical = "0.3.0"
//! ```
//!
//! There is several ways to use Ical depending on the level of parsing you want. Some new
//! wrapper/formater could appeare in the next releases.
//!
//! By default all the features are included but you can choose to include in you
//! project only the needed ones.
//!

#![deny(missing_docs)]

#[macro_use]
extern crate error_chain;


#[cfg(any(feature = "line-parser", feature = "line-reader"))]
pub mod line;

#[cfg(any(feature = "ical-parser", feature = "vcard-parser"))]
pub mod parser;

const PARAM_VALUE_DELIMITER: char = ',';
const VALUE_DELIMITER: char = ':';
const PARAM_DELIMITER: char = ';';
const PARAM_NAME_DELIMITER: char = '=';


#[cfg(feature = "ical-parser")]
pub use parser::ical::IcalParser;

#[cfg(feature = "vcard-parser")]
pub use parser::vcard::VcardParser;

#[cfg(feature = "line-parser")]
pub use line::parser::LineParser;

#[cfg(feature = "line-reader")]
pub use line::reader::LineReader;



mod errors {
    error_chain! {
        types {
            Error, ErrorKind, ResultExt, Result;
        }

        foreign_links {
        }

        errors {
            MissingHeader {
                description("A header is missing.")
                    display("missing header")
            }

            EndOfFile {
                description("The end of file with an unfinished object.")
                    display("end of file")
            }

            NotImplemented {
                description("This feature is not implemented yet.")
                    display("not implemented")
            }

            NotComplete {
                description("The current object is not complete.")
                    display("incomplete object")
            }

            InvalidComponent {
                description("The current component is invalid.")
                    display("invalid component")
            }

            MissingValueDelimiter {
                description("Missing value delimiter.")
                    display("missing value delimiter")
            }

            MissingName {
                description("Missing name.")
                    display("missing name")
            }

            MissingValue {
                description("Missing value.")
                    display("missing value")
            }

            InvalidParamFormat {
                description("Invalid param format.")
                    display("invalid param format")
            }
        }
    }
}
