
use std::io::BufRead;
use std::error::Error;
use std::cell::RefCell;
use std::fmt;

use super::super::super::parser;
use super::super::property;
use super::alarm;

#[derive(Debug)]
pub struct IcalEvent {
    pub properties: Vec<property::Property>,
    pub alarms: Vec<alarm::IcalAlarm>,
}

impl IcalEvent {
    pub fn new() -> IcalEvent {
        IcalEvent {
            properties: Vec::new(),
            alarms: Vec::new(),
        }
    }

    pub fn parse<B: BufRead>(line_parser: &RefCell<parser::LineParser<B>>)
                             -> Result<IcalEvent, EventError> {
        let mut event = IcalEvent::new();

        loop {
            let line: parser::LineParsed;

            {
                line = match line_parser.borrow_mut().next() {
                    Some(val) => val.clone(),
                    None => return Err(EventError::NotComplete),
                }?;
            }

            match line.name.as_str() {
                "END" => break,
                "BEGIN" => match line.value.as_str() {
                    "VALARM" => event.alarms.push((alarm::IcalAlarm::parse(line_parser)?)),
                    _   => return Err(EventError::InvalidComponent(line.value)),

                },
                _ => event.properties.push((property::Property::parse(line)?)),
            };
        }

        Ok(event)
    }
}


#[derive(Debug, PartialEq, Clone)]
pub enum EventError {
    Parse(parser::ParseError),
    Property(property::PropertyError),
    Alarm(alarm::AlarmError),
    NotComplete,
    InvalidComponent(String),
}

impl Error for EventError {
    fn description(&self) -> &str {
        match *self {
            EventError::Parse(ref err) => err.description(),
            EventError::Property(ref err) => err.description(),
            EventError::Alarm(ref err) => err.description(),
            EventError::NotComplete => "Event component is not complete.",
            EventError::InvalidComponent(_) => "Contain an invalid component."
        }
    }

    fn cause(&self) -> Option<&Error> {
        match *self {
            EventError::Parse(ref err) => Some(err),
            EventError::Property(ref err) => Some(err),
            EventError::Alarm(ref err) => Some(err),
            _ => None,
        }
    }
}

impl fmt::Display for EventError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}

impl From<parser::ParseError> for EventError {
    fn from(err: parser::ParseError) -> EventError {
        EventError::Parse(err)
    }
}

impl From<property::PropertyError> for EventError {
    fn from(err: property::PropertyError) -> EventError {
        EventError::Property(err)
    }
}

impl From<alarm::AlarmError> for EventError {
    fn from(err: alarm::AlarmError) -> EventError {
        EventError::Alarm(err)
    }
}
