
use rustc_serialize::json::{ToJson, Json, Array};
use std::collections::HashMap;

use ::property;
use ::parser;

/// A list of `DesignSet`. It list all the possible properties and their
/// format.
pub type Design = HashMap<property::Type, DesignElem>;


/// A element of the HashMap `DesignSet`. It represent a the properties of an
/// attribute.
#[derive(Debug)]
pub struct DesignElem {
    /// The default `Type` for the attribute.
    pub value_type:         Type,

    /// An attribute can have several values. If this is the case `multi_value`
    /// contain the char used to split the elements.
    pub multi_value:        Option<char>,

    /// An attribute can accept several `Type`. In the case allowed_types
    /// take a list of all allowed elements. The value_type attribute will be
    /// tested first and can be listed in `allowed_types`.
    pub allowed_types:      Option<Vec<Type>>,

    /// An attribute value can be structured on several 'sub-values'.
    /// `structured_value` contain the char used to split this elements.
    pub structured_value:   Option<char>,
}

impl DesignElem {
    /// Parse a value string.
    pub fn parse(&self, buffer: &str) -> Container {
        let mut value_str: &str;
        let mut result = Vec::new();

        // If this is a multi value string.
        if let Some(delimiter) = self.multi_value {
            let mut last_pos = 0;

            // Split each pieces.
            while let Some(pos) = parser::unescaped_find(buffer, last_pos, delimiter) {
                // Save use of slice_unchecked. last_pos and pos come from the
                // buffer find method.
                unsafe {
                    value_str = buffer.slice_unchecked(last_pos, pos);
                }

                if let Some(value) = Value::from_str(value_str, self.value_type) {
                    result.push(value);
                }

                last_pos = pos + 1;
            }

            // On the last piece take the rest of the string.
            value_str = buffer.split_at(last_pos).1;
        } else {
            value_str = buffer;
        }

        if let Some(value) = Value::from_str(value_str, self.value_type) {
            result.push(value);
        }

        match result.len() {
            0   => Container::None,
            1   => Container::Single(result.pop().unwrap()),
            _   => Container::Multi(result),
        }
    }
}


#[derive(Debug, PartialEq, Eq)]
pub enum Container {
    Single(Value),
    Multi(Vec<Value>),
    None,
}

impl ToJson for Container {
    fn to_json(&self) -> Json {
        match self {
            &Container::None               => Json::Null,
            &Container::Single(ref val)    => val.to_json(),
            &Container::Multi(ref list)    => {
                let mut res = Array::new();

                for elem in list {
                    res.push(elem.to_json());
                }

                Json::Array(res)
            }
        }
    }
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

impl Value {
    fn from_str(value: &str, value_type: Type) -> Option<Value> {
        if value.len() == 0 {
            return None
        }


        match value_type {
            Type::Text     => Some(Value::Text(value.to_string())),
            Type::Uri      => Some(Value::Uri(value.to_string())),
            _              => None,
        }
    }
}

#[derive(Debug, Clone, Copy)]
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
}
