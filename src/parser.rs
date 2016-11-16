
use std::fs::File;
use std::path::Path;
use std::fmt;
use std::error::Error;
use std::io::{BufReader, BufRead, Read};

use ::{PARAM_DELIMITER, VALUE_DELIMITER};
use ::VcardIcalError;
use ::property;
use ::value;
use ::param;

/// Parser is the main parser struct. It handle the parsing of all the filetypes.
pub struct Parser {
    reader: BufReader<File>,

    /// An attribute can be on several lines. Once the first line of an
    /// attribute is retrieved, the line after nned to be retrieved too in
    /// order to check if it's a single or multiline attribute. As the reader
    /// work with a stream it's impossible to read twice the same line so if
    /// the next line is the start of a new attribute it must be cached.
    next_start: Option<String>,

    //Protocol: Protocol,
    //Version: Version,

    value_design: value::ValueDesignSet,
    param_design: param::ParamDesignSet,
}



impl Iterator for Parser {
    type Item = Result<property::Property, VcardIcalError>;

    /// A property can be split over mutliple lines.
    ///
    /// ```text
    /// ADR;TYPE=home;LABEL="Heidestraße 17\n51147 Köln\nDeutschland"
    ///  :;;Heidestraße 17;Köln;;51147;Germany
    /// ```
    ///
    /// Note the additional space at the second line.
    /// This method takes a `BufReader` and merge every lines of a property
    /// into one.
    fn next(&mut self) -> Option<Result<property::Property, VcardIcalError>> {

        match self.fetch_line() {
            Some(line)  => Some(self.parse_line(line.as_str())),
            None        => None,
        }
    }
}


impl Parser {
    /// parse_vcard_file take a `Path` to a VCard file and parse the content
    /// into a vector of contact.
    pub fn from_path(path: &Path) -> Result<Parser, VcardIcalError>{
        let file = match File::open(path) {
            Ok(file)    => file,
            Err(err)      => return Err(VcardIcalError::File(err)),
        };

        let reader = BufReader::new(file);

        let parser = Parser{
            reader: reader,
            next_start: None,
            value_design: property::get_vcard_properties(),
            param_design: property::get_vcard_param_properties(),
            //Protocol: Protocol::None,
            //Version: Version::None,

        };

        //parser.retrieve_header_line();
        //parser.retrieve_header_line();

        //println!("proto: {:?} / version: {:?}", parser.Protocol, parser.Version);
        //assert!(false);

        Ok(parser)
    }


    fn fetch_line(&mut self) -> Option<String> {
        let mut new_line = String::new();

        // If during the last iteration a new line have been saved, start with.
        if let Some(start) = self.next_start.clone() {
            new_line.push_str(start.as_str());
            self.next_start = None;

            // This is the first iteration, next_start isn't been filled yet.
        } else {
            if self.reader.by_ref().read_line(&mut new_line).is_err() {
                return None;
            }

            new_line = new_line.trim_right().to_string();
        }

        for line in self.reader.by_ref().lines() {
            let mut line = line.unwrap();

            // This is a multi-lines attribute.
            if line.starts_with(" ") {
                // Remove the ' ' charactere and join with the current line.
                line.remove(0);
                new_line.push_str(line.trim_right())

                    // This is a new attribute so it need to be saved it for
                    // the next iteration.
            } else {
                self.next_start = Some(line.trim().to_string());
                break;
            }
        }

        if new_line.is_empty() {
            None
        } else {
            Some(new_line)
        }
    }


    //fn retrieve_header_line(&mut self) -> Result<(), ParserError> {
        //let key: &str;
        //let value: &str;
        //let line;

        //line = match self.fetch_line() {
            //Some(val)   => val,
            //None        => return Err(ParserError::NoProtocol),
        //};

        //let (_, value_index) = split_line(line.as_str());

        //if value_index.is_none() {
            //return Err(ParserError::InvalidProtocol);
        //}

        //unsafe {
            //key = line.slice_unchecked(0, value_index.unwrap());
            //value = line.slice_unchecked(value_index.unwrap(), line.len());
        //}


        //match key.to_lowercase().as_str() {
            //"begin"     => self.Protocol = Protocol::from_str(value),
            //"version"   => self.Version = Version::from_str(value),
            //_           => {
                //println!("key: {}", key);
                //return Err(ParserError::InvalidProtocol);
            //}
        //};

        //Ok(())
    //}


    fn parse_line(&mut self, line: &str) -> Result<property::Property, VcardIcalError> {

        let name: property::PropertyType;
        let params: param::ParamSet;
        let value: value::ValueContainer;


        let (value_position, param_position) = split_line(line);

        // There is some parameters, handle them
        if let Some(param_position) = param_position {
            // The use is safe because the param_position come from the
            // 'find' method.
            unsafe {
                name = property::PropertyType::from_str(line.slice_unchecked(0, param_position))?;
            }

            params = param::parse_parameters(line, param_position, &self.param_design)?;

        } else if value_position.is_some() {
            // Line without parameters (BEGIN:VCARD, CLASS:PUBLIC)
            params = param::ParamSet::None;

            unsafe {
                name = property::PropertyType::from_str(line.slice_unchecked(0, value_position.unwrap()))?;
            }

        } else {
            // Missing VALUE_DELIMITER, the line is invalid.
            return Err(VcardIcalError::Parser(ParserError::InvalidFormat));
        }

        let value_str;

        unsafe {
            value_str = line.slice_unchecked(value_position.unwrap() + 1, line.len());
        }

        if let Some(value_design_elem) = self.value_design.get(&name) {
            value = value::parse_value(value_str, value_design_elem);
        } else {
            unimplemented!()
        }


        //let value = parse_value(value, multi_value, ptype);
        //println!("value: {:?}", value);

        Ok(property::Property{
            name: name,
            params: params,
            value: value,
        })
    }
}

/// Split the line between the value and the parameters.
///
/// Different property cases
///
/// 1. RRULE:FREQ=foo
///      FREQ= is not a param but the value
///
/// 2. ATTENDEE;ROLE=REQ-PARTICIPANT;
///      ROLE= is a param because ':' has not happend yet
fn split_line(line: &str) -> (Option<usize>, Option<usize>) {
    // Break up the parts of the line.
    let value_position = line.find(VALUE_DELIMITER);
    let mut param_position = line.find(PARAM_DELIMITER);


    if param_position.is_some() && value_position.is_some() {
        // When the parameter delimiter is after the value delimiter then its
        // not a parameter.
        if param_position.unwrap() > value_position.unwrap() {
            param_position = None;
        }
    }

    (value_position, param_position)
}

/// ParserError handler all the parsing error. It take a `ParserErrorCode`.
#[derive(Debug)]
pub enum ParserError {
    InvalidFormat,
    InvalidProtocol,
    NoProtocol,
}

impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Parser error: {}",  self.description())
    }
}

impl Error for ParserError {
    fn description(&self) -> &str {
        match *self {
            ParserError::InvalidFormat => "Invalid line format.",
            ParserError::InvalidProtocol => "Invalid protocol.",
            ParserError::NoProtocol => "No protocol found.",
        }
    }

    fn cause(&self) -> Option<&Error> {
        match *self {
            ParserError::InvalidFormat => None,
            ParserError::InvalidProtocol => None,
            ParserError::NoProtocol => None,
        }
    }
}


/// Identical to `find` but will only match values when they are not
/// preceded by a backslash character.
pub fn unescaped_find(buffer: &str, start: usize, pat: char) -> Option<usize> {

    //let res = buf_chars
    buffer.char_indices()
        .skip(start)
        .find(|&(index, value)| {
            if value == pat {
                if buffer.as_bytes().get(index - 1) != Some(&b'\\') {
                    return true;
                }
            }

            return false;
        })
    .and_then(|(index, _)| {
       Some(index)
    })

}


/// Internal helper for rfc6868.
pub fn rfc_6868_escape(input: &str) -> String {
    let mut s = input.to_string();

    if s.contains("^'") {
        s = s.replace("^'", "\"");
    }

    if s.contains("^n") {
        s = s.replace("^n", "\n");
    }

    if s.contains("^^") {
        s = s.replace("^^", "^");
    }

    s
}
