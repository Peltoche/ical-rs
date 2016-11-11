
use std::collections::HashMap;
use rustc_serialize::json::{ToJson, Json, Object};

use ::parser::ParserError;
use ::property::*;
use ::design::DesignElem;
use ::value::{ValueContainer, Value, parse_value};
use ::design::{unescaped_find, rfc_6868_escape};

#[derive(Debug)]
pub enum ParamSet {
    None,
    Some(HashMap<ParamName, ValueContainer>)
}

impl ToJson for ParamSet {
    fn to_json(&self) -> Json {
        match self {
            &ParamSet::None    => Json::Null,
            &ParamSet::Some(ref list)    => {
                let mut res = Object::new();

                for (key, val) in list {
                    res.insert(key.to_string(), val.to_json());
                }

                Json::Object(res)
            }
        }
    }
}

/// Regroup all the rules (`ParamDesignElem`) for a type of file (VCard / ICal).
pub type ParamDesignSet = HashMap<ParamName, ParamDesignElem>;


/// Represent the set of rules for a parameter. It contain the expected format
/// for the value or the list of possible values.
#[derive(Debug)]
pub struct ParamDesignElem {
    /// If it's a 'open' parameter (not closed to a list of predetermined
    /// choises), the values is parsed by a `DesignSet` structur.
    pub design:             Option<DesignElem>,

    /// If it's a 'closed' parameter (choices restricted to a predetermined
    /// list), all the possible values a listed her.
    pub allowed_values:     Option<Vec<&'static str>>,


    pub allow_name:         bool,
    pub allow_iana_token:   bool,
}




/// Regroupe all the possible arguments accepted.
#[derive(Debug, PartialEq, Hash, Eq, Clone, Copy)]
pub enum ParamName {
    Language,
    Value,
    Pref,
    AltId,
    Pid,
    Type,
    Mediatype,
    Calscale,
    SortAs,
    Geo,
    Tz,
    //Any(String),
}

impl ParamName {
    /// Match a string an return the corresponding `ParamName`. The string
    /// is move to lowercase before matching.
    pub fn from_str(input: &str) -> Option<ParamName> {

        match input.to_lowercase().as_str() {
            "language"  => Some(ParamName::Language),
            "value"     => Some(ParamName::Value),
            "atltid"    => Some(ParamName::AltId),
            "pref"      => Some(ParamName::Pref),
            "pid"       => Some(ParamName::Pid),
            "type"      => Some(ParamName::Type),
            "mediatype" => Some(ParamName::Mediatype),
            "calscale"  => Some(ParamName::Calscale),
            "sortas"    => Some(ParamName::SortAs),
            "geo"       => Some(ParamName::Geo),
            "tz"        => Some(ParamName::Tz),
            _           => None,
        }
    }


    fn to_string(&self) -> String {

        let res = match self {
           &ParamName::Language     => "LANGUAGE",
           &ParamName::Value        => "VALUE",
           &ParamName::Pref         => "PREF",
           &ParamName::AltId        => "ALTID",
           &ParamName::Pid          => "PID",
           &ParamName::Type         => "TYPE",
           &ParamName::Mediatype    => "MEDIATYPE",
           &ParamName::Calscale     => "CALSCALE",
           &ParamName::SortAs       => "SORTAS",
           &ParamName::Geo          => "GEO",
           &ParamName::Tz           => "TZ",
        };

        res.to_string()
    }
}



/// Parse the parameters from a string to an object. The start
pub fn parse_parameters(line: &str, start: usize, p_design: &ParamDesignSet) -> Result<ParamSet, ParserError> {
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

