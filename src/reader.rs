
use std::fs::File;
use std::io::{BufReader, BufRead, Read};
use std::collections::HashMap;

use ::parser::ParserError;
use ::property::*;
use ::value::{Value, ValueType, ValueContainer};
use ::design::{DesignSet, DesignElem};
use ::design::unescaped_find;
use ::param::{ParamName, ParamDesignSet, ParamDesignElem, ParamSet};


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

/// Parse the parameters from a string to an object. The start
fn parse_parameters(line: &str, start: usize, p_design: &ParamDesignSet) -> Result<ParamSet, ParserError> {
    let mut params = HashMap::new();
    let mut last_param: usize = start;
    let mut have_params: bool = true;

    if start > line.len() {
        return Err(ParserError::new(
                format!("Start value out of range -> start: {} / line length: {}", start, line)
                ));
    }

    // Loop as long as it find a PARAM_NAME_DELIMITER announcing a new parameter
    // key.
    while let Some(mut pos) = unescaped_find(line, last_param + 1, PARAM_NAME_DELIMITER) {

        let p_design_elem: &ParamDesignElem;
        let value_str: &str;
        let value: ValueContainer;
        let name: &str;

        let value_pos: usize;

        if have_params == false {
            break;
        }


        // Retrieve the param name and parse it.

        // Unsafe slice is secure. last_param and pos come from the find method
        // on line.
        unsafe {
            // The +1 are used to remove the separator charactere ';'.
            name = line.slice_unchecked(last_param + 1, pos);
        }

        if name.is_empty() {
            return Err(ParserError::new(
                    format!("Empty parameter name in '{}'", line)
                    ));
        }

        let name = match ParamName::from_str(name) {
            Some(val)   => val,
            None        => return Err(ParserError::new(
                    format!("Unknown parameter type: {}", name)
                    )),
        };

        // Looking for the corresponding set of rules
        if let Some(elem) = p_design.get(&name) {
            p_design_elem = elem
        } else {
            return Err(ParserError::new(format!("Invalid parameter `{:?}`: {}", name, line)));
        }


        // Retrieve the param value.


        // Check the next letter after the PARAM_NAME_DELIMITER.
        // 1. Find '\"' -> the value is around dquote
        // 2. Find other letter -> this a 'raw' value
        let next_char = match line.bytes().nth(pos + 1) {
            Some(val)   => val,
            None        => return Err(ParserError::new(
                    format!("Invalid param format in line `{}`", line)
                    )),
        };

        // 1.
        if next_char == b'\"' {
            // Jump the PARAM_NAME_DELIMITER and the close dquote
            value_pos = pos + 2;

            pos = match unescaped_find(line, value_pos, '\"') {
                Some(val)   => val,
                None        => return Err(ParserError::new(
                        format!("Invalid line (no matching double quote): {}", line)
                        )),
            };


            // Safe her because value_pos and pos are index from some find
            // methods on line.
            unsafe {
                // Retrieve the value section.
                value_str = line.slice_unchecked(value_pos, pos);
            }


            match unescaped_find(line, pos, PARAM_DELIMITER) {
                Some(_)     => {},
                None        => have_params = false,
            };

            // 2.
        } else {
            value_pos = pos + 1;


            let next_param_pos = unescaped_find(line, value_pos, PARAM_DELIMITER);
            let prop_value_pos = unescaped_find(line, value_pos, VALUE_DELIMITER);
            let next_pos;

            if prop_value_pos.is_some() && next_param_pos.is_some() && next_param_pos.unwrap() > prop_value_pos.unwrap() {

                // This is a delimiter in the property value: let's stop here.
                next_pos = prop_value_pos.unwrap();

                have_params = false;

            } else if next_param_pos.is_none() {
                // no ';'
                next_pos = match prop_value_pos {
                    Some(val)   => val,
                    None        => line.len(),
                };

                have_params = false;
            } else {
                last_param = next_param_pos.unwrap();
                next_pos = last_param;
            }

            unsafe {
                value_str = line.slice_unchecked(value_pos, next_pos);
            }
        }

        if let Some(ref design) = p_design_elem.design {
            value = parse_value(rfc_6868_escape(value_str).as_str(), &design);

        } else if let Some(ref allowed_values) = p_design_elem.allowed_values {
            if !allowed_values.contains(&value_str) {
                return Err(ParserError::new(
                        format!("Invalid value for parameter {:?} : {}", name, line))
                    );

            } else {
                value = ValueContainer::Single(Value::Text(value_str.to_string()));
            }

        } else {
            return Err(ParserError::new(format!("Invalid design for value (no design set): {}", line)));
        }

        params.insert(name, value);
    }


    Ok(ParamSet::Some(params))
}


/// Internal helper for rfc6868.
fn rfc_6868_escape(input: &str) -> String {
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


/// Parse a value string.
fn parse_value(buffer: &str, design: &DesignElem) -> ValueContainer {
    let mut value: &str;
    let mut result = Vec::new();

    // If this is a multi value string.
    if let Some(delimiter) = design.multi_value {
        let mut last_pos = 0;

        // Split each pieces.
        while let Some(pos) = unescaped_find(buffer, last_pos, delimiter) {
            // Save use of slice_unchecked. last_pos and pos come from the
            // buffer find method.
            unsafe {
                value = buffer.slice_unchecked(last_pos, pos);
            }

            if let Some(res) = value_to_typed(value, design) {
                result.push(res);
            }

            last_pos = pos + 1;
        }

        // On the last piece take the rest of the string.
        value = buffer.split_at(last_pos).1;
    } else {
        value = buffer;
    }

    if let Some(res) = value_to_typed(value, design) {
        result.push(res);
    }

    match result.len() {
        0   => ValueContainer::None,
        1   => ValueContainer::Single(result.pop().unwrap()),
        _   => ValueContainer::Multi(result),
    }
}

fn value_to_typed(value: &str, design: &DesignElem) -> Option<Value> {
    if value.len() == 0 {
        return None
    }


    match design.value_type {
        ValueType::Text     => Some(Value::Text(value.to_string())),
        ValueType::Uri      => Some(Value::Uri(value.to_string())),
        _                   => None,
    }
}
