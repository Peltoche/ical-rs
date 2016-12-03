
mod component;

use std::io::BufRead;
use std::error::Error;
use std::cell::RefCell;
use std::fmt;

use super::{parser, line};
use self::component::{Component, IcalCalendar};

/// Reader returning Ical object from a `BufRead`.
pub struct IcalParser<B> {
    line_parser: RefCell<parser::LineParser<B>>,
}

impl<B: BufRead> IcalParser<B> {
    pub fn new(reader: B) -> IcalParser<B> {
        let line_reader = line::LineReader::new(reader);
        let line_parser = parser::LineParser::new(line_reader);

        IcalParser { line_parser: RefCell::new(line_parser) }
    }

    /// Read the next line and check if it's a valid VCALENDAR start.
    fn check_header(&mut self) -> Result<Option<()>, IcalError> {
        let line = match self.line_parser.borrow_mut().next() {
            Some(val) => val?,
            None => return Ok(None),
        };

        if line.name != "BEGIN" || line.value != "VCALENDAR" || line.params != None {
            return Err(IcalError::MissingCalendarHeader);
        }

        Ok(Some(()))
    }
}

impl<B: BufRead> Iterator for IcalParser<B> {
    type Item = Result<IcalCalendar, IcalError>;

    fn next(&mut self) -> Option<Result<IcalCalendar, IcalError>> {
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

#[derive(Debug)]
pub enum IcalError {
    Parse(parser::ParseError),
    EndOfFile,
    MissingCalendarHeader,
    NotImplemented,
    NotComplete,
    InvalidComponent(String),
}

impl Error for IcalError {
    fn description(&self) -> &str {
        match *self {
            IcalError::MissingCalendarHeader => "Missing VCALENDAR header.",
            IcalError::EndOfFile => "End of file.",
            IcalError::NotImplemented => "Element parsing not implemented yet.",
            IcalError::NotComplete => "Calendar component is not complete.",
            IcalError::InvalidComponent(_) => "Contains an invalid component.",
            IcalError::Parse(ref err) => err.description(),
        }
    }

    fn cause(&self) -> Option<&Error> {
        match *self {
            IcalError::Parse(ref err) => Some(err),
            _ => None,
        }
    }
}

impl fmt::Display for IcalError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}

impl From<parser::ParseError> for IcalError {
    fn from(err: parser::ParseError) -> IcalError {
        IcalError::Parse(err)
    }
}
