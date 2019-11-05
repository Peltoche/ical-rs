//! Parse a VCARD address book.
//!
//! Wrap the result of the `PropertyParser` into components.
//!
//! Each component contains properties (ie: Property) or sub-components.
//!
//! * The `VcardParser` return `VcardContact` objects.
//!
//! # Examples
//!
//! ```toml
//! [dependencies.ical]
//! version = "0.3.*"
//! default-features = false
//! features = ["vcard-parser"]
//! ```
//!
//! ```rust
//! extern crate ical;
//!
//! use std::io::BufReader;
//! use std::fs::File;
//!
//! let buf = BufReader::new(File::open("./tests/ressources/vcard_input.vcf")
//! .unwrap());
//!
//! let reader = ical::VcardParser::new(buf);
//!
//! for contact in reader {
//!     println!("{:?}", contact);
//! }
//! ```

pub mod component;

// Sys mods
use crate::parser::ParserError;
use std::cell::RefCell;
use std::io::BufRead;

// Internal mods
use crate::line::LineReader;
use crate::parser::Component;
use crate::property::PropertyParser;

/// Reader returning `VcardContact` object from a `BufRead`.
pub struct VcardParser<B> {
    line_parser: RefCell<PropertyParser<B>>,
}

impl<B: BufRead> VcardParser<B> {
    /// Create a new `VcardParser` from a reader.
    pub fn new(reader: B) -> VcardParser<B> {
        let line_reader = LineReader::new(reader);
        let line_parser = PropertyParser::new(line_reader);

        VcardParser {
            line_parser: RefCell::new(line_parser),
        }
    }

    /// Read the next line and check if it's a valid VCARD start.
    fn check_header(&mut self) -> Result<Option<()>, ParserError> {
        let line = match self.line_parser.borrow_mut().next() {
            Some(val) => val.map_err(|e| ParserError::PropertyError(e))?,
            None => return Ok(None),
        };

        if line.name != "BEGIN"
            || line.value.is_none()
            || line.value.unwrap() != "VCARD"
            || line.params != None
        {
            return Err(ParserError::MissingHeader.into());
        }

        Ok(Some(()))
    }
}

impl<B: BufRead> Iterator for VcardParser<B> {
    type Item = Result<component::VcardContact, ParserError>;

    fn next(&mut self) -> Option<Result<component::VcardContact, ParserError>> {
        match self.check_header() {
            Ok(res) => {
                if res == None {
                    return None;
                }
            }
            Err(err) => return Some(Err(err)),
        };

        let mut contact = component::VcardContact::new();
        let result = match contact.parse(&self.line_parser) {
            Ok(_) => Ok(contact),
            Err(err) => Err(err),
        };

        Some(result)
    }
}
