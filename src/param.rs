
use std::collections::HashMap;
use rustc_serialize::json::{ToJson, Json, Object};

use ::{PARAM_DELIMITER, VALUE_DELIMITER, PARAM_NAME_DELIMITER};
use ::{ParseError, ErrorKind};
use ::parser;


#[derive(Debug)]
pub enum Container {
    Some(HashMap<Type, String>),
    None,
}

impl Container {
    pub fn get(&self, key: &Type) -> Option<&String> {
        match *self {
            Container::Some(ref map) => map.get(key),
            Container::None => None,
        }
    }
}

impl ToJson for Container {
    fn to_json(&self) -> Json {
        match self {
            &Container::None => Json::Null,
            &Container::Some(ref list) => {
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
#[derive(Debug, PartialEq, Hash, Eq, Clone)]
pub enum Type {
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
    Any(String),
}

impl Type {
    /// Match a string an return the corresponding `Type`. The string
    /// is move to lowercase before matching.
    pub fn from_str(input: &str) -> Type {

        match input.to_lowercase().as_str() {
            "language" => Type::Language,
            "value" => Type::Value,
            "atltid" => Type::AltId,
            "pref" => Type::Pref,
            "pid" => Type::Pid,
            "type" => Type::Type,
            "mediatype" => Type::Mediatype,
            "calscale" => Type::Calscale,
            "sortas" => Type::SortAs,
            "geo" => Type::Geo,
            "tz" => Type::Tz,
            _ => Type::Any(input.to_uppercase()),
        }
    }


    fn to_string(&self) -> String {
        match *self {
                Type::Language => "LANGUAGE",
                Type::Value => "VALUE",
                Type::Pref => "PREF",
                Type::AltId => "ALTID",
                Type::Pid => "PID",
                Type::Type => "TYPE",
                Type::Mediatype => "MEDIATYPE",
                Type::Calscale => "CALSCALE",
                Type::SortAs => "SORTAS",
                Type::Geo => "GEO",
                Type::Tz => "TZ",
                Type::Any(ref val) => return val.clone(),
            }
            .to_string()
    }
}



/// Parse the parameters from a string to an object. The start
pub fn parse(line: &str, start: usize) -> Result<Container, ParseError> {
    let mut params = HashMap::new();
    let mut last_param: usize = start;
    let mut have_params: bool = true;


    // Loop as long as it find a PARAM_NAME_DELIMITER announcing a new parameter
    // key.
    while let Some(mut pos) = parser::unescaped_find(line, last_param + 1, PARAM_NAME_DELIMITER) {

        // let design_elem: &DesignElem;
        let value: String;
        let name: Type;

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
            return Err(ParseError::new(ErrorKind::InvalidParamFormat));
        }

        name = Type::from_str(name_str);

        // Retrieve the param value.


        // Check the next letter after the PARAM_NAME_DELIMITER.
        // 1. Find '\"' -> the value is around dquote
        // 2. Find other letter -> this a 'raw' value
        let next_char = match line.bytes().nth(pos + 1) {
            Some(val) => val,
            None => return Err(ParseError::new(ErrorKind::InvalidLineFormat)),
        };

        // 1.
        if next_char == b'\"' {
            // Jump the PARAM_NAME_DELIMITER and the close dquote
            value_pos = pos + 2;

            pos = match parser::unescaped_find(line, value_pos, '\"') {
                Some(val) => val,
                None => return Err(ParseError::new(ErrorKind::InvalidParamFormat)),
            };


            // Safe her because value_pos and pos are index from some find
            // methods on line.
            unsafe {
                // Retrieve the value section.
                value = line.slice_unchecked(value_pos, pos).to_string();
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

            if prop_value_pos.is_some() && next_param_pos.is_some() &&
               next_param_pos.unwrap() > prop_value_pos.unwrap() {

                // This is a delimiter in the property value: let's stop here.
                next_pos = prop_value_pos.unwrap();

                have_params = false;

            } else if next_param_pos.is_none() {
                // no ';'
                next_pos = match prop_value_pos {
                    Some(val) => val,
                    None => line.len(),
                };

                have_params = false;
            } else {
                last_param = next_param_pos.unwrap();
                next_pos = last_param;
            }

            unsafe {
                value = line.slice_unchecked(value_pos, next_pos).to_string();
            }
        }


        params.insert(name, value);
    }


    Ok(Container::Some(params))
}
