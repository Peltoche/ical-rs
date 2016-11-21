use rustc_serialize::json::{ToJson, Json};
use std::collections::HashMap;
use std::fmt;
use std::error::Error;
use std::num;

use ::property;

/// A list of `DesignSet`. It list all the possible properties and their
/// format.
pub type Design = HashMap<Type, DesignElem>;


pub struct DesignElem {
    pub parse_str:   fn(&str) -> Result<Value, ValueError>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Value {
    // Generics values
    Text(String),
    TextMulti(Vec<String>),
    Uri(String),
    Adr(String),
    Date(String),
    Integer(i32),
    //Boolean,
    //Float,
    //UtcOffset,
    LanguageTag(String),

    // Customs values
    N(String),
}

impl ToJson for Value {
    fn to_json(&self) -> Json {
        match *self {
            Value::Text(ref val)            => Json::String(val.clone()),
            Value::Uri(ref val)             => Json::String(val.clone()),
            Value::Adr(ref val)             => Json::String(val.clone()),
            Value::Date(ref val)            => Json::String(val.clone()),
            Value::Integer(ref val)         => Json::I64(val.clone() as i64),
            Value::LanguageTag(ref val)     => Json::String(val.clone()),
            Value::N(ref val)               => Json::String(val.clone()),
            Value::TextMulti(ref list)       => {
                let mut res = Vec::new();

                for elem in list {
                    res.push(Json::String(elem.clone()));
                }

                Json::Array(res)
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Type{
    Text,
    TextMulti,
    TextMultiQuote,
    Uri,
    Date,
    //Time,
    DateTime,
    DateAndOrTime,
    Timestamp,
    //Boolean,
    Integer,
    //Float,
    UtcOffset,
    LanguageTag,

    // Custom types
    N,
    Adr,
}

impl Type {
    pub fn from_str(input: &str) -> Result<Type, property::PropertyError> {
        match input.to_lowercase().as_str() {
            "text"          => Ok(Type::Text),
            "uri"           => Ok(Type::Uri),
            "date"          => Ok(Type::Date),
            "datetime"      => Ok(Type::DateTime),
            "dateandortime" => Ok(Type::DateAndOrTime),
            "timestamp"     => Ok(Type::Timestamp),
            "integer"       => Ok(Type::Integer),
            "utcoffset"     => Ok(Type::UtcOffset),
            "utc-offset"    => Ok(Type::UtcOffset),
            "languagetag"   => Ok(Type::LanguageTag),
            _               => Err(property::PropertyError::UnknownType),
        }
    }
}

pub fn get_vcard_design() -> Design {
    let mut v_design = HashMap::with_capacity(15);


    v_design.insert(Type::Text, DesignElem{parse_str: parse_text});
    v_design.insert(Type::Uri, DesignElem{parse_str: parse_uri});
    v_design.insert(Type::Adr, DesignElem{parse_str: parse_adr});
    v_design.insert(Type::Date, DesignElem{parse_str: parse_date});
    v_design.insert(Type::DateTime, DesignElem{parse_str: parse_date_time});
    v_design.insert(Type::DateAndOrTime, DesignElem{parse_str: parse_date_and_or_time});
    v_design.insert(Type::N, DesignElem{parse_str: parse_n});
    v_design.insert(Type::Timestamp, DesignElem{parse_str: parse_timestamp});
    v_design.insert(Type::UtcOffset, DesignElem{parse_str: parse_utcoffset});
    v_design.insert(Type::TextMulti, DesignElem{parse_str: parse_text_multi});
    v_design.insert(Type::TextMultiQuote, DesignElem{parse_str: parse_text_multi_quote});


    v_design
}

pub fn parse_text(input: &str) -> Result<Value, ValueError> {
    Ok(Value::Text(input.to_string()))
}

pub fn parse_text_multi_quote(input: &str) -> Result<Value, ValueError> {
    parse_multi(input, ',')
}

pub fn parse_text_multi(input: &str) -> Result<Value, ValueError> {
    parse_multi(input, ';')
}

fn parse_multi(input: &str, separator: char) -> Result<Value, ValueError> {
    let mut res = Vec::new();

    let list = input.split(separator);

    for elem in list {
        res.push(elem.to_string());
    }

    if res.len() == 1 {
        Ok(Value::Text(res.pop().unwrap()))
    } else {
        Ok(Value::TextMulti(res))
    }
}

pub fn parse_uri(input: &str) -> Result<Value, ValueError> {
    Ok(Value::Uri(input.to_string()))
}

pub fn parse_adr(input: &str) -> Result<Value, ValueError> {
    Ok(Value::Adr(input.to_string()))
}

pub fn parse_date(input: &str) -> Result<Value, ValueError> {
    Ok(Value::Date(input.to_string()))
}

pub fn parse_date_time(input: &str) -> Result<Value, ValueError> {
    Ok(Value::Date(input.to_string()))
}

pub fn parse_date_and_or_time(input: &str) -> Result<Value, ValueError> {
    Ok(Value::Date(input.to_string()))
}

pub fn parse_timestamp(input: &str) -> Result<Value, ValueError> {
    Ok(Value::Date(input.to_string()))
}

pub fn parse_n(input: &str) -> Result<Value, ValueError> {
    Ok(Value::N(input.to_string()))
}

pub fn parse_utcoffset(input: &str) -> Result<Value, ValueError> {
    Ok(Value::Date(input.to_string()))
}


/// ValueError handler all the parsing error. It take a `ParserErrorCode`.
#[derive(Debug)]
pub enum ValueError {
    NotImplemented,
    ParseInt(num::ParseIntError),
}

impl fmt::Display for ValueError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Value error: {}",  self.description())
    }
}

impl Error for ValueError {
    fn description(&self) -> &str {
        match *self {
            ValueError::NotImplemented => "The parsing of this type of value \
                                        is not implemented yet.",
            ValueError::ParseInt(_) => "An error during the parsing occured.",
        }
    }

    fn cause(&self) -> Option<&Error> {
        match *self {
            ValueError::NotImplemented  => None,
            ValueError::ParseInt(ref err)   => Some(err),
        }
    }
}

impl From<num::ParseIntError> for ValueError {
    fn from(err: num::ParseIntError) -> ValueError {
        ValueError::ParseInt(err)
    }
}
