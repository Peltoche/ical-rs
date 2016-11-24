
use std::fs::File;
use std::path::Path;
use std::io::{BufReader, BufRead, Read};

use ::{PARAM_DELIMITER, VALUE_DELIMITER};
use ::{ParseError, ErrorKind};
use ::property;
use ::value;
use ::param;


#[derive(Debug)]
/// Main struct returning the parsed content of a line.
pub struct Property {
    pub name: property::Type,
    pub params: param::Container,
    pub value: value::Value,
}

#[cfg(feature = "rustc-serialize")]
mod rustc_serialize {
    use rustc_serialize::json::{ToJson, Json, Object};
    use super::Property;

    impl ToJson for Property {
        fn to_json(&self) -> Json {
            let mut obj = Object::new();

            obj.insert("name".to_string(), self.name.to_json());
            obj.insert("params".to_string(), self.params.to_json());
            obj.insert("value".to_string(), self.value.to_json());

            Json::Object(obj)
        }
    }
}


#[derive(Debug)]
pub enum Protocol {
    Vcard,
    Ical,
}

impl Protocol {
    fn from_str(input: &str) -> Result<Protocol, ParseError> {
        match input.to_lowercase().as_str() {
            "vcard" => Ok(Protocol::Vcard),
            "Ical" => Ok(Protocol::Ical),
            _ => Err(ParseError::new(ErrorKind::InvalidProtocol)),
        }
    }
}


#[derive(Debug)]
pub enum Version {
    Four,
    Three,
}

impl Version {
    fn from_str(input: &str) -> Result<Version, ParseError> {
        match input {
            "4.0" => Ok(Version::Four),
            "3.0" => Ok(Version::Three),
            _ => Err(ParseError::new(ErrorKind::InvalidVersion)),
        }
    }
}

/// Parser is the main parser struct. It handle the parsing of all the filetypes.
#[allow(dead_code)]
pub struct Parser {
    reader: BufReader<File>,

    // An attribute can be on several lines. Once the first line of an
    // attribute is retrieved, the line after nned to be retrieved too in
    // order to check if it's a single or multiline attribute. As the reader
    // work with a stream it's impossible to read twice the same line so if
    // the next line is the start of a new attribute it must be cached.
    next_start: Option<String>,

    protocol: Protocol,
    version: Version,

    property_design: property::Design,
    value_design: value::Design,
}



impl Iterator for Parser {
    type Item = Result<Property, ParseError>;

    // A property can be split over mutliple lines.
    //
    // ```text
    // ADR;TYPE=home;LABEL="Heidestraße 17\n51147 Köln\nDeutschland"
    //  :;;Heidestraße 17;Köln;;51147;Germany
    // ```
    //
    // Note the additional space at the second line.
    // This method takes a `BufReader` and merge every lines of a property
    // into one.
    fn next(&mut self) -> Option<Result<Property, ParseError>> {

        match self.fetch_line() {
            Some(line) => Some(self.parse_line(line.as_str())),
            None => None,
        }
    }
}


impl Parser {
    /// parse_vcard_file take a `Path` to a VCard file and parse the content
    /// into a vector of contact.
    pub fn from_path(path: &Path) -> Result<Parser, ParseError> {
        let file = match File::open(path) {
            Ok(file) => file,
            Err(err) => return Err(ParseError::new(ErrorKind::File(err))),
        };

        let mut reader = BufReader::new(file);

        let (protocol, version) = retrieve_specs(&mut reader)?;

        let parser = Parser {
            reader: reader,
            next_start: None,
            property_design: property::get_vcard_design(),
            value_design: value::get_vcard_design(),
            protocol: protocol,
            version: version,
        };

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

            } else {
                // This is a new attribute so it need to be saved it for
                // the next iteration.
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


    fn parse_line(&mut self, line: &str) -> Result<Property, ParseError> {

        let name: property::Type;
        let params: param::Container;
        let value: value::Value;


        let (value_position, param_position) = split_line(line);

        // There is some parameters, handle them
        if let Some(param_position) = param_position {
            // The use is safe because the param_position come from the
            // 'find' method.
            unsafe {
                name = property::Type::from_str(line.slice_unchecked(0, param_position))?;
            }

            params = param::parse(line, param_position)?;

        } else if value_position.is_some() {
            // Line without parameters (BEGIN:VCARD, CLASS:PUBLIC)
            params = param::Container::None;

            unsafe {
                name = property::Type::from_str(line.slice_unchecked(0, value_position.unwrap()))?;
            }

        } else {
            // Missing VALUE_DELIMITER, the line is invalid.
            return Err(ParseError::new(ErrorKind::InvalidLineFormat));
        }

        let value_str;

        unsafe {
            value_str = line.slice_unchecked(value_position.unwrap() + 1, line.len());
        }

        if let Some(property) = self.property_design.get(&name) {
            let mut v_type = property.value_type;

            if let Some(type_str) = params.get(&param::Type::Value) {
                if let Some(ref allowed) = property.allowed_types {
                    let param_type = value::Type::from_str(type_str)?;
                    println!("param: {:?}", param_type);

                    if allowed.contains(&param_type) {
                        v_type = param_type;
                    }
                } else {
                    return Err(ParseError::new(ErrorKind::UnacceptedType));
                }
            }


            value = match self.value_design.get(&v_type) {
                Some(design) => (design.parse_str)(value_str)?,
                None => return Err(ParseError::new(ErrorKind::NotImplemented)),
            };

        } else {
            return Err(ParseError::new(ErrorKind::InvalidProperty));
        }


        // let value = parse_value(value, multi_value, ptype);
        // println!("value: {:?}", value);

        Ok(Property {
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


fn retrieve_specs(reader: &mut BufReader<File>) -> Result<(Protocol, Version), ParseError> {
    let protocol: Protocol;
    let version: Version;

    let mut line: String = String::new();
    if reader.read_line(&mut line).is_err() {
        return Err(ParseError::new(ErrorKind::InvalidLineFormat));
    }

    let (key, value) = retrieve_key_value_line(&line)?;
    if key == "begin" {
        protocol = Protocol::from_str(value.as_str())?;
    } else {
        return Err(ParseError::new(ErrorKind::InvalidLineFormat));
    }


    let mut line: String = String::new();
    if reader.read_line(&mut line).is_err() {
        return Err(ParseError::new(ErrorKind::InvalidLineFormat));
    }

    let (key, value) = retrieve_key_value_line(&line)?;
    if key == "version" {
        version = Version::from_str(value.as_str())?;
    } else {
        return Err(ParseError::new(ErrorKind::InvalidLineFormat));
    }

    Ok((protocol, version))
}

fn retrieve_key_value_line(line: &String) -> Result<(String, String), ParseError> {
    let mut elems = line.splitn(2, VALUE_DELIMITER);

    let key = match elems.next() {
        Some(val) => val,
        None => return Err(ParseError::new(ErrorKind::InvalidLineFormat)),
    };

    let value = match elems.next() {
        Some(val) => val.trim_right(),
        None => return Err(ParseError::new(ErrorKind::InvalidLineFormat)),
    };

    Ok((key.to_lowercase(), value.to_lowercase()))
}




/// Identical to `find` but will only match values when they are not
/// preceded by a backslash character.
pub fn unescaped_find(buffer: &str, start: usize, pat: char) -> Option<usize> {

    // let res = buf_chars
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
        .and_then(|(index, _)| Some(index))

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
