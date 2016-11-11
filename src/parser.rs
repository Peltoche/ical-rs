
use std::fs::File;
use std::path::Path;
use std::fmt;
use std::error::Error;

use ::reader::Reader;

/// Parser is the main parser struct. It handle the parsing of all the filetypes.
pub struct Parser;

impl Parser {
    /// parse_vcard_file take a `Path` to a VCard file and parse the content
    /// into a vector of contact.
    pub fn from_file(path: &Path) -> Result<Reader, ParserError>{
        let file = match File::open(path) {
            Ok(file)    => file,
            Err(_)      => {
                return Err(ParserError::new("Invalid file".to_string()));
            }
        };

        Ok(Reader::new(file))
    }
}


/// ParserError handler all the parsing error. It take a `ParserErrorCode`.
#[derive(Debug)]
pub struct ParserError {
    description: String,
}

impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description)
    }
}

impl Error for ParserError {
    fn description(&self) -> &str {
        self.description.as_str()
    }

    fn cause(&self) -> Option<&Error> {
        None
    }
}


impl ParserError {
    pub fn new(description: String) -> ParserError {
        ParserError{
            description: description.clone(),
        }
    }
}
