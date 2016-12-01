
mod event;
mod param;
mod property;
mod alarm;

use std::io::BufRead;
use std::error::Error;
use std::cell::RefCell;
use std::fmt;

use super::parser;
use super::line;

#[derive(Debug)]
pub struct IcalCalendar {
    properties: Vec<property::Property>,
    events: Vec<event::IcalEvent>,
    alarms: Vec<alarm::IcalAlarm>,
}

impl IcalCalendar {
    pub fn new() -> IcalCalendar {
        IcalCalendar {
            properties: Vec::new(),
            events: Vec::new(),
            alarms: Vec::new(),
        }
    }
}

/// Reader returning Ical object from a `BufRead`.
pub struct IcalReader<B> {
    line_parser: RefCell<parser::LineParser<B>>,
}

impl<B: BufRead> IcalReader<B> {
    pub fn new(reader: B) -> IcalReader<B> {
        let line_reader = line::LineReader::new(reader);
        let line_parser = parser::LineParser::new(line_reader);

        IcalReader { line_parser: RefCell::new(line_parser) }
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

    fn read_body(&mut self) -> Result<IcalCalendar, IcalError> {
        let mut calendar = IcalCalendar::new();

        loop {
            let line: parser::LineParsed;

            {
                line = match self.line_parser.borrow_mut().next() {
                    Some(val) => val,
                    None => return Err(IcalError::NotComplete),
                }?;

                if line.name == "END" && line.value == "VCALENDAR" {
                    break;
                }
            }

            if line.name == "BEGIN" {
                match line.value.as_str() {
                    "VEVENT" => calendar.events.push(event::IcalEvent::parse(&self.line_parser)?),
                    "ALARM" => calendar.alarms.push(alarm::IcalAlarm::parse(&self.line_parser)?),
                    _ => return Err(IcalError::NotImplemented),
                };
            } else if line.name == "END" {
                break;
            } else {
                calendar.properties.push(property::Property::parse(line)?);
            }
        }

        Ok(calendar)
    }
}

impl<B: BufRead> Iterator for IcalReader<B> {
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


        Some(self.read_body())
    }
}

#[derive(Debug)]
pub enum IcalError {
    Parse(parser::ParseError),
    Property(property::PropertyError),
    Event(event::EventError),
    Alarm(alarm::AlarmError),
    EndOfFile,
    MissingCalendarHeader,
    NotImplemented,
    NotComplete,
}

impl Error for IcalError {
    fn description(&self) -> &str {
        match *self {
            IcalError::MissingCalendarHeader => "Missing VCALENDAR header.",
            IcalError::EndOfFile => "End of file.",
            IcalError::NotImplemented => "Element parsing not implemented yet.",
            IcalError::NotComplete => "Calendar component is not complete.",
            IcalError::Parse(ref err) => err.description(),
            IcalError::Property(ref err) => err.description(),
            IcalError::Event(ref err) => err.description(),
            IcalError::Alarm(ref err) => err.description(),
        }
    }

    fn cause(&self) -> Option<&Error> {
        match *self {
            IcalError::Parse(ref err) => Some(err),
            IcalError::Property(ref err) => Some(err),
            IcalError::Event(ref err) => Some(err),
            IcalError::Alarm(ref err) => Some(err),
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

impl From<property::PropertyError> for IcalError {
    fn from(err: property::PropertyError) -> IcalError {
        IcalError::Property(err)
    }
}

impl From<event::EventError> for IcalError {
    fn from(err: event::EventError) -> IcalError {
        IcalError::Event(err)
    }
}

impl From<alarm::AlarmError> for IcalError {
    fn from(err: alarm::AlarmError) -> IcalError {
        IcalError::Alarm(err)
    }
}
