
use std::collections::HashMap;
use rustc_serialize::json::{ToJson, Json, Object};
use std::fmt;
use std::error::Error;

use ::{PARAM_DELIMITER, VALUE_DELIMITER, PARAM_NAME_DELIMITER};
use ::value;
use ::parser;



/// Regroup all the rules (`DesignElem`) for a type of file (VCard / ICal).
pub type DesignSet = HashMap<Name, DesignElem>;


/// Represent the set of rules for a parameter. It contain the expected format
/// for the value or the list of possible values.
#[derive(Debug)]
pub struct DesignElem {
    /// If it's a 'open' parameter (not closed to a list of predetermined
    /// choises), the values is parsed by a `ValueDesignSet` structur.
    pub design:             Option<value::DesignElem>,

    /// If it's a 'closed' parameter (choices restricted to a predetermined
    /// list), all the possible values a listed her.
    pub allowed_values:     Option<Vec<&'static str>>,


    pub allow_name:         bool,
    pub allow_iana_token:   bool,
}


#[derive(Debug)]
pub enum Container {
    None,
    Some(HashMap<Name, value::Container>)
}


impl ToJson for Container {
    fn to_json(&self) -> Json {
        match self {
            &Container::None    => Json::Null,
            &Container::Some(ref list)    => {
                let mut res = Object::new();

                for (key, val) in list {
                    res.insert(key.to_string(), val.to_json());
                }

                Json::Object(res)
            }
        }
    }
}



/// Regroupe all the possible arguments accepted.
#[derive(Debug, PartialEq, Hash, Eq, Clone, Copy)]
pub enum Name {
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

impl Name {
    /// Match a string an return the corresponding `Name`. The string
    /// is move to lowercase before matching.
    pub fn from_str(input: &str) -> Option<Name> {

        match input.to_lowercase().as_str() {
            "language"  => Some(Name::Language),
            "value"     => Some(Name::Value),
            "atltid"    => Some(Name::AltId),
            "pref"      => Some(Name::Pref),
            "pid"       => Some(Name::Pid),
            "type"      => Some(Name::Type),
            "mediatype" => Some(Name::Mediatype),
            "calscale"  => Some(Name::Calscale),
            "sortas"    => Some(Name::SortAs),
            "geo"       => Some(Name::Geo),
            "tz"        => Some(Name::Tz),
            _           => None,
        }
    }


    fn to_string(&self) -> String {
        match self {
           &Name::Language     => "LANGUAGE",
           &Name::Value        => "VALUE",
           &Name::Pref         => "PREF",
           &Name::AltId        => "ALTID",
           &Name::Pid          => "PID",
           &Name::Type         => "TYPE",
           &Name::Mediatype    => "MEDIATYPE",
           &Name::Calscale     => "CALSCALE",
           &Name::SortAs       => "SORTAS",
           &Name::Geo          => "GEO",
           &Name::Tz           => "TZ",
        }.to_string()
    }
}



/// Parse the parameters from a string to an object. The start
pub fn parse(line: &str, start: usize, p_design: &DesignSet) -> Result<Container, ParamError> {
    let mut params = HashMap::new();
    let mut last_param: usize = start;
    let mut have_params: bool = true;


    // Loop as long as it find a PARAM_NAME_DELIMITER announcing a new parameter
    // key.
    while let Some(mut pos) = parser::unescaped_find(line, last_param + 1, PARAM_NAME_DELIMITER) {

        let p_design_elem: &DesignElem;
        let value_str: &str;
        let value: value::Container;
        let name: Name;

        let value_pos: usize;

        if have_params == false {
            break;
        }


        // Retrieve the param name and parse it.

        // Unsafe slice is secure. last_param and pos come from the find method
        // on line.
        let name_str: &str;
        unsafe {
            // The +1 are used to remove the separator charactere ';'.
            name_str = line.slice_unchecked(last_param + 1, pos);
        }

        if name_str.is_empty() {
            return Err(ParamError::MissingName);
        }

        name = match Name::from_str(name_str) {
            Some(val)   => val,
            None        => return Err(ParamError::UnknownType),
        };

        // Looking for the corresponding set of rules
        p_design_elem = match p_design.get(&name) {
            Some(val)   => val,
            None        => return Err(ParamError::NotForProperty),
        };

        // Retrieve the param value.


        // Check the next letter after the PARAM_NAME_DELIMITER.
        // 1. Find '\"' -> the value is around dquote
        // 2. Find other letter -> this a 'raw' value
        let next_char = match line.bytes().nth(pos + 1) {
            Some(val)   => val,
            None        => return Err(ParamError::InvalidFormat),
        };

        // 1.
        if next_char == b'\"' {
            // Jump the PARAM_NAME_DELIMITER and the close dquote
            value_pos = pos + 2;

            pos = match parser::unescaped_find(line, value_pos, '\"') {
                Some(val)   => val,
                None        => return Err(ParamError::InvalidFormat),
            };


            // Safe her because value_pos and pos are index from some find
            // methods on line.
            unsafe {
                // Retrieve the value section.
                value_str = line.slice_unchecked(value_pos, pos);
            }


            if parser::unescaped_find(line, pos, PARAM_DELIMITER) == None {
                have_params = false;
            };

            // 2.
        } else {
            value_pos = pos + 1;


            let next_param_pos = parser::unescaped_find(line, value_pos, PARAM_DELIMITER);
            let prop_value_pos = parser::unescaped_find(line, value_pos, VALUE_DELIMITER);
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
            value = design.parse(parser::rfc_6868_escape(value_str).as_str());

        } else if let Some(ref allowed_values) = p_design_elem.allowed_values {
            if !allowed_values.contains(&value_str) {
                return Err(ParamError::InvalidValue);
            } else {
                value = value::Container::Single(value::Value::Text(value_str.to_string()));
            }

        } else {
            return Err(ParamError::Internal);
        }

        params.insert(name, value);
    }


    Ok(Container::Some(params))
}


/// ParamError handler all the param parsing error.
#[derive(Debug, Clone, Copy)]
pub enum ParamError {
    MissingName,
    UnknownType,
    NotForProperty,
    InvalidFormat,
    InvalidValue,
    Internal,
}

impl fmt::Display for ParamError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Param error: {}",  self.description())
    }
}

impl Error for ParamError {
    fn description(&self) -> &str {
        match *self {
            ParamError::MissingName => "Missing a name to property parameter.",
            ParamError::UnknownType => "Unknow parameter type.",
            ParamError::NotForProperty => "Parameter not handled by this property.",
            ParamError::InvalidFormat => "Invalid format for.",
            ParamError::InvalidValue => "Invalid value.",
            ParamError::Internal => "Internal error.",
        }
    }

    fn cause(&self) -> Option<&Error> {
        None
    }
}
