
use std::fs::File;
use std::io::{BufReader, BufRead, Read};

use ::parser::ParserError;
use ::property::*;
use ::value::{ValueContainer, parse_value};
use ::design::DesignSet;
use ::param::{ParamDesignSet, ParamSet, parse_parameters};


pub struct Reader {
    reader: BufReader<File>,
    /// An attribute can be on several lines. Once the first line of an
    /// attribute is retrieved, the line after nned to be retrieved too in
    /// order to check if it's a single or multiline attribute. As the reader
    /// work with a stream it's impossible to read twice the same line so if
    /// the next line is the start of a new attribute it must be cached.
    next_start: Option<String>,
}

impl Iterator for Reader {
    type Item = Result<Property, ParserError>;

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
    fn next(&mut self) -> Option<Result<Property, ParserError>> {
        let mut new_line = String::new();

        let v_design = get_vcard_properties();
        let p_design = get_vcard_param_properties();

        // If during the last iteration a new line have been saved, start with.
        if let Some(start) = self.next_start.clone() {
            new_line.push_str(start.as_str());
            self.next_start = None;

            // This is the first iteration, net_start isn't been filled yet.
        } else {
            match self.reader.by_ref().read_line(&mut new_line) {
                Ok(_)       => {},
                Err(_)      => return None,
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
            Some(parse_line(new_line.as_str(), &v_design, &p_design))
        }
    }
}

impl Reader {
    pub fn new(file: File) -> Reader {
        let reader = BufReader::new(file);

        Reader{
            reader: reader,
            next_start: None,
        }

        // The first line give us the type
        //let res = vcard_reader.parse_line();
    }
}



fn parse_line(line: &str, v_design: &DesignSet, p_design: &ParamDesignSet) -> Result<Property, ParserError> {

    let name: PropertyType;
    let params: ParamSet;
    let value: ValueContainer;


    let (value_position, param_position) = split_line(line);

    // There is some parameters, handle them
    if let Some(param_position) = param_position {
        // The use is safe because the param_position come from the
        // 'find' method.
        unsafe {
            name = match PropertyType::from_str(line.slice_unchecked(0, param_position)) {
                Ok(val)     => val,
                Err(err)    => return Err(err),
            };
        }

        params = match parse_parameters(line, param_position, p_design) {
            Ok(val)       => val,
            Err(err)    => return Err(err),
        };

    } else if value_position.is_some() {
        // Line without parameters (BEGIN:VCARD, CLASS:PUBLIC)
        params = ParamSet::None;

        unsafe {
            name = match PropertyType::from_str(line.slice_unchecked(0, value_position.unwrap())) {
                Ok(val)     => val,
                Err(err)    => return Err(err),
            };
        }

        // If its not begin/end, then this is a property with an empty value,
        // which should be considered valid.

    // Missing VALUE_DELIMITER, the line is invalid.
    } else {
        return Err(ParserError::new(format!("Invalid line (no token ';' or ':'): {}", line)));
    }

    let value_str;

    unsafe {
        value_str = line.slice_unchecked(value_position.unwrap() + 1, line.len());
    }

    if let Some(value_design_elem) = v_design.get(&name) {
        value = parse_value(value_str, value_design_elem);
    } else {
        return Err(ParserError::new("value: value choosing value not implemented".to_string()))
    }



    //let value = parse_value(value, multi_value, ptype);
    //println!("value: {:?}", value);

    Ok(Property{
        name: name,
        params: params,
        value: value,
    })
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
