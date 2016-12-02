
use std::io::BufRead;
use std::error::Error;
use std::cell::RefCell;
use std::fmt;

use super::super::super::parser;
use super::super::property;
use super::alarm;

#[derive(Debug)]
pub struct IcalTodo {
    pub properties: Vec<property::Property>,
    pub alarms: Vec<alarm::IcalAlarm>,
}

impl IcalTodo {
    pub fn new() -> IcalTodo {
        IcalTodo {
            properties: Vec::new(),
            alarms: Vec::new()
        }
    }

    pub fn parse<B: BufRead>(line_parser: &RefCell<parser::LineParser<B>>)
                             -> Result<IcalTodo, TodoError> {
        let mut todo = IcalTodo::new();

        loop {
            let line: parser::LineParsed;

            {
                line = match line_parser.borrow_mut().next() {
                    Some(val) => val.clone(),
                    None => return Err(TodoError::NotComplete),
                }?;
            }

            match line.name.as_str() {
                "END" => break,
                "BEGIN" => match line.value.as_str() {
                    "VALARM" => todo.alarms.push((alarm::IcalAlarm::parse(line_parser)?)),
                    _ => return Err(TodoError::InvalidComponent(line.value)),
                },
                _ => todo.properties.push((property::Property::parse(line)?)),
            };
        }

        Ok(todo)
    }
}


#[derive(Debug, PartialEq, Clone)]
pub enum TodoError {
    Parse(parser::ParseError),
    Property(property::PropertyError),
    Alarm(alarm::AlarmError),
    NotComplete,
    InvalidComponent(String),
}

impl Error for TodoError {
    fn description(&self) -> &str {
        match *self {
            TodoError::Parse(ref err) => err.description(),
            TodoError::Property(ref err) => err.description(),
            TodoError::Alarm(ref err) => err.description(),
            TodoError::NotComplete => "Event component is not complete.",
            TodoError::InvalidComponent(_) => "Contain an invalid component."
        }
    }

    fn cause(&self) -> Option<&Error> {
        match *self {
            TodoError::Parse(ref err) => Some(err),
            TodoError::Property(ref err) => Some(err),
            TodoError::Alarm(ref err) => Some(err),
            _ => None,
        }
    }
}

impl fmt::Display for TodoError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}

impl From<parser::ParseError> for TodoError {
    fn from(err: parser::ParseError) -> TodoError {
        TodoError::Parse(err)
    }
}

impl From<property::PropertyError> for TodoError {
    fn from(err: property::PropertyError) -> TodoError {
        TodoError::Property(err)
    }
}

impl From<alarm::AlarmError> for TodoError {
    fn from(err: alarm::AlarmError) -> TodoError {
        TodoError::Alarm(err)
    }
}
