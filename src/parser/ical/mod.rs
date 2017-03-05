//! Parse an ICAL calendar.
//!
//! Wrap the result of the `PropertyParser` into components.
//!
//! Each component contains properties (ie: Property) or sub-components.
//!
//! * The VcardParser return `IcalCalendar` objects.
//!
//! # Examples
//!
//!
//! Cargo.toml:
//! ```toml
//! [dependencies.ical]
//! version = "0.3.*"
//! default-features = false
//! features = ["ical-parser"]
//! ```
//!
//! ```rust
//! extern crate ical;
//!
//! use std::io::BufReader;
//! use std::fs::File;
//!
//! let buf = BufReader::new(File::open("./tests/ressources/ical_input.ics")
//! .unwrap());
//!
//! let reader = ical::IcalParser::new(buf);
//!
//! for line in reader {
//!     println!("{:?}", line);
//! }
//! ```

mod component;

// Sys mods.
use std::io::BufRead;
use std::cell::RefCell;

// Internal mods
use line::LineReader;
use property::PropertyParser;
use parser::ical::component::IcalCalendar;
use parser::Component;
use parser::errors::*;

/// Reader returning `IcalCalendar` object from a `BufRead`.
pub struct IcalParser<B> {
    line_parser: RefCell<PropertyParser<B>>,
}

impl<B: BufRead> IcalParser<B> {
    /// Return a new `IcalParser` from a `Reader`.
    pub fn new(reader: B) -> IcalParser<B> {
        let line_reader = LineReader::new(reader);
        let line_parser = PropertyParser::new(line_reader);

        IcalParser { line_parser: RefCell::new(line_parser) }
    }

    /// Read the next line and check if it's a valid VCALENDAR start.
    fn check_header(&mut self) -> Result<Option<()>> {
        let line = match self.line_parser.borrow_mut().next() {
            Some(val) => val?,
            None => return Ok(None),
        };

        if line.name != "BEGIN" || line.value.is_none() || line.value.unwrap() != "VCALENDAR" ||
           line.params != None {
            return Err(ErrorKind::MissingHeader.into());
        }

        Ok(Some(()))
    }
}

impl<B: BufRead> Iterator for IcalParser<B> {
    type Item = Result<IcalCalendar>;

    fn next(&mut self) -> Option<Result<IcalCalendar>> {
        match self.check_header() {
            Ok(res) => {
                if res == None {
                    return None;
                }
            }
            Err(err) => return Some(Err(err)),
        };


        let mut calendar = IcalCalendar::new();
        let result = match calendar.parse(&self.line_parser) {
            Ok(_) => Ok(calendar),
            Err(err) => Err(err),
        };

        Some(result)
    }
}
