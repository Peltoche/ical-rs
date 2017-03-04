
mod component;

use std::io::BufRead;
use std::cell::RefCell;

use line::{parser, reader};
use self::component::IcalCalendar;
use super::{ParseError, Component};

/// Reader returning Ical object from a `BufRead`.
pub struct IcalParser<B> {
    line_parser: RefCell<parser::LineParser<B>>,
}

impl<B: BufRead> IcalParser<B> {
    pub fn new(reader: B) -> IcalParser<B> {
        let line_reader = reader::LineReader::new(reader);
        let line_parser = parser::LineParser::new(line_reader);

        IcalParser { line_parser: RefCell::new(line_parser) }
    }

    /// Read the next line and check if it's a valid VCALENDAR start.
    fn check_header(&mut self) -> Result<Option<()>, ParseError> {
        let line = match self.line_parser.borrow_mut().next() {
            Some(val) => val?,
            None => return Ok(None),
        };

        if line.name != "BEGIN" || line.value.is_none() || line.value.unwrap() != "VCALENDAR" || line.params != None {
            return Err(ParseError::MissingHeader);
        }

        Ok(Some(()))
    }
}

impl<B: BufRead> Iterator for IcalParser<B> {
    type Item = Result<IcalCalendar, ParseError>;

    fn next(&mut self) -> Option<Result<IcalCalendar, ParseError>> {
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

