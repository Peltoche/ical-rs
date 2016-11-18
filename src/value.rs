
use rustc_serialize::json::{ToJson, Json};
use std::collections::HashMap;
use std::fmt;
use std::error::Error;

use ::property;

/// A list of `DesignSet`. It list all the possible properties and their
/// format.
pub type Design = HashMap<Type, DesignElem>;


pub struct DesignElem {
    pub from_str:   fn(&str) -> Value,
}


#[derive(Debug, PartialEq, Eq)]
pub enum Value {
    Text(String),
    Uri(String),
    //Date,
    //Time,
    //DateTime,
    //DateAndOrTime,
    //Timestamp,
    //Boolean,
    //Integer(i32),
    //Float,
    //UtcOffset,
    //LanguageTag,
}

impl ToJson for Value {
    fn to_json(&self) -> Json {
        match self {
            &Value::Text(ref val)       => Json::String(val.clone()),
            &Value::Uri(ref val)        => Json::String(val.clone()),
            //&Value::Integer(ref val)    => Json::String(val as String),
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
    Nickname,
    N,
    Org,
    Adr,
    Gender,
    Clientpidmap,
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
            "languagetag"   => Ok(Type::LanguageTag),
            _               => Err(property::PropertyError::UnknownType),
        }
    }
}

pub fn get_vcard_design() -> Design {
    let mut v_design = HashMap::with_capacity(7);


    v_design.insert(Type::Text, DesignElem{from_str: imple});
    v_design.insert(Type::Uri, DesignElem{from_str: imple});
    v_design.insert(Type::Uri, DesignElem{from_str: imple});


    v_design
}


pub fn imple(input: &str) -> Value {
    println!("Parser not implemented!\nparse: {}", input);

    Value::Text("value".to_string())
}



/// ValueError handler all the parsing error. It take a `ParserErrorCode`.
#[derive(Debug)]
pub enum ValueError {
    NotImplemented
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
        }
    }

    fn cause(&self) -> Option<&Error> {
        None
    }
}
