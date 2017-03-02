
pub mod ical;
pub mod vcard;

use std::io::BufRead;
use std::cell::RefCell;
use std::error::Error;
use std::fmt;

use line::parser;

pub trait Component {
    fn add_sub_component<B: BufRead>(&mut self,
                                     value: &str,
                                     line_parser: &RefCell<parser::LineParser<B>>)
                                     -> Result<(), ParseError>;
    fn add_property(&mut self, property: parser::LineParsed);

    fn parse<B: BufRead>(&mut self,
                         line_parser: &RefCell<parser::LineParser<B>>)
                         -> Result<(), ParseError> {

        loop {
            let line: parser::LineParsed;

            {
                line = match line_parser.borrow_mut().next() {
                    Some(val) => val,
                    None => return Err(ParseError::NotComplete),
                }?;
            }

            match line.name.as_str() {
                "END" => break,
                "BEGIN" => match line.value {
                    Some(v) => self.add_sub_component(v.as_str(), line_parser)?,
                    None    => self.add_sub_component("", line_parser)?,
                },

                _ => self.add_property(line),
            };
        }

        Ok(())
    }
}


#[derive(Debug)]
pub enum ParseError {
    Parse(parser::ParseError),
    EndOfFile,
    MissingHeader,
    NotImplemented,
    NotComplete,
    InvalidComponent(String),
}

impl Error for ParseError {
    fn description(&self) -> &str {
        match *self {
            ParseError::MissingHeader => "Missing VCALENDAR header.",
            ParseError::EndOfFile => "End of file.",
            ParseError::NotImplemented => "Element parsing not implemented yet.",
            ParseError::NotComplete => "Calendar component is not complete.",
            ParseError::InvalidComponent(_) => "Contains an invalid component.",
            ParseError::Parse(ref err) => err.description(),
        }
    }

    fn cause(&self) -> Option<&Error> {
        match *self {
            ParseError::Parse(ref err) => Some(err),
            _ => None,
        }
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}

impl From<parser::ParseError> for ParseError {
    fn from(err: parser::ParseError) -> ParseError {
        ParseError::Parse(err)
    }
}
