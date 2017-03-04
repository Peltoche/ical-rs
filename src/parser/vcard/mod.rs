
mod component;

use std::io::BufRead;
use std::cell::RefCell;

use line::{parser, reader};
use self::component::VcardContact;
use super::{ParseError, Component};

pub struct VcardParser<B> {
    line_parser: RefCell<parser::LineParser<B>>,
}

impl<B: BufRead> VcardParser<B> {
    pub fn new(reader: B) -> VcardParser<B> {
        let line_reader = reader::LineReader::new(reader);
        let line_parser = parser::LineParser::new(line_reader);

        VcardParser { line_parser: RefCell::new(line_parser) }
    }

    /// Read the next line and check if it's a valid VCARD start.
    fn check_header(&mut self) -> Result<Option<()>, ParseError> {
        let line = match self.line_parser.borrow_mut().next() {
            Some(val) => val?,
            None => return Ok(None),
        };

        if line.name != "BEGIN" || line.value.is_none() || line.value.unwrap() != "VCARD" ||
           line.params != None {
            return Err(ParseError::MissingHeader);
        }

        Ok(Some(()))
    }
}


impl<B: BufRead> Iterator for VcardParser<B> {
    type Item = Result<VcardContact, ParseError>;

    fn next(&mut self) -> Option<Result<VcardContact, ParseError>> {
        match self.check_header() {
            Ok(res) => {
                if res == None {
                    return None;
                }
            }
            Err(err) => return Some(Err(err)),
        };

        let mut contact = VcardContact::new();
        let result = match contact.parse(&self.line_parser) {
            Ok(_) => Ok(contact),
            Err(err) => Err(err),
        };

        Some(result)
    }
}
