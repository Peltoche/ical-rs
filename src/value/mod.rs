use rustc_serialize::json::{ToJson, Json};
use std::collections::HashMap;
use std::fmt;
use std::error::Error;
use std::num;

mod parser;

use ::property;

/// A list of `DesignSet`. It list all the possible properties and their
/// format.
pub type Design = HashMap<Type, DesignElem>;


pub struct DesignElem {
    pub from_str:   fn(&str) -> Result<Value, ValueError>,
}


#[derive(Debug, PartialEq, Eq)]
pub enum Value {
    // Generics values
    Text(String),
    Uri(String),
    Adr(String),
    Date(String),
    Integer(i32),
    //Boolean,
    //Float,
    //UtcOffset,
    LanguageTag(String),

    // Customs values
    ClientPidMap(String),
    Gender(String),
    N(String),
    Nickname(String),
    Org(String),
}

impl ToJson for Value {
    fn to_json(&self) -> Json {
        match *self {
            Value::Text(ref val)            => Json::String(val.clone()),
            Value::Uri(ref val)             => Json::String(val.clone()),
            Value::Adr(ref val)             => Json::String(val.clone()),
            Value::Date(ref val)            => Json::String(val.clone()),
            Value::Integer(ref val)         => Json::I64(val.clone() as i64),
            Value::ClientPidMap(ref val)    => Json::String(val.clone()),
            Value::Gender(ref val)          => Json::String(val.clone()),
            Value::LanguageTag(ref val)     => Json::String(val.clone()),
            Value::N(ref val)               => Json::String(val.clone()),
            Value::Nickname(ref val)        => Json::String(val.clone()),
            Value::Org(ref val)             => Json::String(val.clone()),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Type{
    Text,
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
    Nickname,
    N,
    Org,
    Adr,
    Gender,
    ClientPidMap,
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


    v_design.insert(Type::Text, DesignElem{from_str: parser::from_text});
    v_design.insert(Type::Uri, DesignElem{from_str: parser::from_uri});
    v_design.insert(Type::Adr, DesignElem{from_str: parser::from_adr});
    v_design.insert(Type::Date, DesignElem{from_str: parser::from_date});
    v_design.insert(Type::DateTime, DesignElem{from_str: parser::from_date_time});
    v_design.insert(Type::DateAndOrTime, DesignElem{from_str: parser::from_date_and_or_time});
    v_design.insert(Type::ClientPidMap, DesignElem{from_str: parser::from_clientpidmap});
    v_design.insert(Type::Gender, DesignElem{from_str: parser::from_gender});
    v_design.insert(Type::LanguageTag, DesignElem{from_str: parser::from_languagetag});
    v_design.insert(Type::N, DesignElem{from_str: parser::from_n});
    v_design.insert(Type::Nickname, DesignElem{from_str: parser::from_nickname});
    v_design.insert(Type::Org, DesignElem{from_str: parser::from_org});
    v_design.insert(Type::Timestamp, DesignElem{from_str: parser::from_timestamp});
    v_design.insert(Type::UtcOffset, DesignElem{from_str: parser::from_utcoffset});


    v_design
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
