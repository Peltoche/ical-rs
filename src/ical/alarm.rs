
use std::io::BufRead;
use std::error::Error;
use std::cell::RefCell;
use std::fmt;

use super::super::parser;
use super::property;

#[derive(Debug)]
pub struct IcalAlarm {
    properties: Vec<property::Property>,
}

impl IcalAlarm {
    pub fn new() -> IcalAlarm {
        IcalAlarm { properties: Vec::new() }
    }

    pub fn parse<B: BufRead>(line_parser: &RefCell<parser::LineParser<B>>)
                             -> Result<IcalAlarm, AlarmError> {
        let mut alarm = IcalAlarm::new();

        loop {
            let line: parser::LineParsed;

            {
                line = match line_parser.borrow_mut().next() {
                    Some(val) => val,
                    None => return Err(AlarmError::NotComplete),
                }?;
            }

            match line.name.as_str() {
                "END" => break,
                _ => alarm.add_property(property::Property::parse(line)?),
            };
        }

        Ok(alarm)
    }

    pub fn add_property(&mut self, property: property::Property) {
        self.properties.push(property)
    }
}


#[derive(Debug, PartialEq, Clone, Copy)]
pub enum AlarmError {
    Parse(parser::ParseError),
    Property(property::PropertyError),
    NotComplete,
}

impl Error for AlarmError {
    fn description(&self) -> &str {
        match *self {
            AlarmError::Parse(ref err) => err.description(),
            AlarmError::Property(ref err) => err.description(),
            AlarmError::NotComplete => "Alarm component is not complete.",
        }
    }

    fn cause(&self) -> Option<&Error> {
        match *self {
            AlarmError::Parse(ref err) => Some(err),
            AlarmError::Property(ref err) => Some(err),
            _ => None,
        }
    }
}

impl fmt::Display for AlarmError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}

impl From<property::PropertyError> for AlarmError {
    fn from(err: property::PropertyError) -> AlarmError {
        AlarmError::Property(err)
    }
}

impl From<parser::ParseError> for AlarmError {
    fn from(err: parser::ParseError) -> AlarmError {
        AlarmError::Parse(err)
    }
}
