//! Parse for ical values
use std::collections::HashMap;

use ::{ParseError, ErrorKind};

/// Value Design for a specific protocol and version.
///
/// Each version and protocol have its own implementation. The `Design`
/// type make it generic by implementing all the rules for the valid
/// value types.
pub type Design = HashMap<Type, DesignElem>;


/// Design rules for a specific type of value.
pub struct DesignElem {
    /// Parse a raw value into a typed `Value`.
    pub parse_str: fn(&str) -> Result<Value, ParseError>,
}

/// Parsed values.
#[derive(Debug, PartialEq, Eq)]
pub enum Value {
    // Generics values
    Text(String),
    TextMulti(Vec<String>),
    Uri(String),
    Adr(String),
    Date(String),
    Integer(i32),
    // Boolean,
    // Float,
    // UtcOffset,
    LanguageTag(String),

    // Customs values
    N(String),
}

#[cfg(feature = "rustc-serialize")]
mod rustc_serialize {
    use rustc_serialize::json::{ToJson, Json};
    use super::Value;

    impl ToJson for Value {
        fn to_json(&self) -> Json {
            match *self {
                Value::Text(ref val) => Json::String(val.clone()),
                Value::Uri(ref val) => Json::String(val.clone()),
                Value::Adr(ref val) => Json::String(val.clone()),
                Value::Date(ref val) => Json::String(val.clone()),
                Value::Integer(ref val) => Json::I64(val.clone() as i64),
                Value::LanguageTag(ref val) => Json::String(val.clone()),
                Value::N(ref val) => Json::String(val.clone()),
                Value::TextMulti(ref list) => {
                    let mut res = Vec::new();

                    for elem in list {
                        res.push(Json::String(elem.clone()));
                    }

                    Json::Array(res)
                }
            }
        }
    }
}

/// Type of value.
///
/// It is used by the `property::DesignElem` to specify which type of value
/// is expected for the property.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Type {
    Text,
    TextMulti,
    TextMultiQuote,
    Uri,
    Date,
    // Time,
    DateTime,
    DateAndOrTime,
    Timestamp,
    // Boolean,
    Integer,
    // Float,
    UtcOffset,
    LanguageTag,

    // Custom types
    N,
    Adr,
}

impl Type {
    /// Return the corresponding `value::Type`.
    pub fn from_str(input: &str) -> Result<Type, ParseError> {
        match input.to_lowercase().as_str() {
            "text" => Ok(Type::Text),
            "uri" => Ok(Type::Uri),
            "date" => Ok(Type::Date),
            "datetime" => Ok(Type::DateTime),
            "dateandortime" => Ok(Type::DateAndOrTime),
            "timestamp" => Ok(Type::Timestamp),
            "integer" => Ok(Type::Integer),
            "utcoffset" => Ok(Type::UtcOffset),
            "utc-offset" => Ok(Type::UtcOffset),
            "languagetag" => Ok(Type::LanguageTag),
            _ => Err(ParseError::new(ErrorKind::InvalidValueType)),
        }
    }
}

/// Return the design rules for a vcard file.
pub fn get_vcard_design() -> Design {
    let mut v_design = HashMap::with_capacity(12);


    v_design.insert(Type::Text, DesignElem { parse_str: parse_text });
    v_design.insert(Type::Uri, DesignElem { parse_str: parse_uri });
    v_design.insert(Type::Adr, DesignElem { parse_str: parse_adr });
    v_design.insert(Type::Date, DesignElem { parse_str: parse_date });
    v_design.insert(Type::DateTime, DesignElem { parse_str: parse_date_time });
    v_design.insert(Type::DateAndOrTime,
                    DesignElem { parse_str: parse_date_and_or_time });
    v_design.insert(Type::N, DesignElem { parse_str: parse_n });
    v_design.insert(Type::Timestamp, DesignElem { parse_str: parse_timestamp });
    v_design.insert(Type::UtcOffset, DesignElem { parse_str: parse_utcoffset });
    v_design.insert(Type::TextMulti, DesignElem { parse_str: parse_text_multi });
    v_design.insert(Type::TextMultiQuote,
                    DesignElem { parse_str: parse_text_multi_quote });


    v_design
}

/// Parse a text value.
pub fn parse_text(input: &str) -> Result<Value, ParseError> {
    Ok(Value::Text(input.to_string()))
}

/// Parse a multi text value separated by quote.
pub fn parse_text_multi_quote(input: &str) -> Result<Value, ParseError> {
    parse_multi(input, ',')
}

/// Parse a multi text value.
pub fn parse_text_multi(input: &str) -> Result<Value, ParseError> {
    parse_multi(input, ';')
}

fn parse_multi(input: &str, separator: char) -> Result<Value, ParseError> {
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

/// Parse an Uri value.
pub fn parse_uri(input: &str) -> Result<Value, ParseError> {
    Ok(Value::Uri(input.to_string()))
}

/// Parse an address value.
pub fn parse_adr(input: &str) -> Result<Value, ParseError> {
    Ok(Value::Adr(input.to_string()))
}

/// Parse a date value.
pub fn parse_date(input: &str) -> Result<Value, ParseError> {
    Ok(Value::Date(input.to_string()))
}

/// Parse a date_time value.
pub fn parse_date_time(input: &str) -> Result<Value, ParseError> {
    Ok(Value::Date(input.to_string()))
}

/// Parse a date_and_or_time value.
pub fn parse_date_and_or_time(input: &str) -> Result<Value, ParseError> {
    Ok(Value::Date(input.to_string()))
}

/// Parse a timestamp value.
pub fn parse_timestamp(input: &str) -> Result<Value, ParseError> {
    Ok(Value::Date(input.to_string()))
}

/// Parse a N value.
pub fn parse_n(input: &str) -> Result<Value, ParseError> {
    Ok(Value::N(input.to_string()))
}

/// Parse an utc_offset value.
pub fn parse_utcoffset(input: &str) -> Result<Value, ParseError> {
    Ok(Value::Date(input.to_string()))
}
