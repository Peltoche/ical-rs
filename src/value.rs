
use rustc_serialize::json::{ToJson, Json, Array};
use std::collections::HashMap;

use ::property;
use ::parser;

/// A list of `DesignSet`. It list all the possible properties and their
/// format.
pub type ValueDesignSet = HashMap<property::PropertyType, ValueDesignElem>;


/// A element of the HashMap `DesignSet`. It represent a the properties of an
/// attribute.
#[derive(Debug)]
pub struct ValueDesignElem {
    /// The default `ValueType` for the attribute.
    pub value_type:         ValueType,

    /// An attribute can have several values. If this is the case `multi_value`
    /// contain the char used to split the elements.
    pub multi_value:        Option<char>,

    /// An attribute can accept several `ValueType`. In the case allowed_types
    /// take a list of all allowed elements. The value_type attribute will be
    /// tested first and can be listed in `allowed_types`.
    pub allowed_types:      Option<Vec<ValueType>>,

    /// An attribute value can be structured on several 'sub-values'.
    /// `structured_value` contain the char used to split this elements.
    pub structured_value:   Option<char>,
}


#[derive(Debug, PartialEq, Eq)]
pub enum ValueContainer {
    Single(Value),
    Multi(Vec<Value>),
    None,
}

impl ToJson for ValueContainer {
    fn to_json(&self) -> Json {
        match self {
            &ValueContainer::None               => Json::Null,
            &ValueContainer::Single(ref val)    => val.to_json(),
            &ValueContainer::Multi(ref list)    => {
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

#[derive(Debug, Clone, Copy)]
pub enum ValueType{
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




/// Parse a value string.
pub fn parse_value(buffer: &str, design: &ValueDesignElem) -> ValueContainer {
    let mut value: &str;
    let mut result = Vec::new();

    // If this is a multi value string.
    if let Some(delimiter) = design.multi_value {
        let mut last_pos = 0;

        // Split each pieces.
        while let Some(pos) = parser::unescaped_find(buffer, last_pos, delimiter) {
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

fn value_to_typed(value: &str, design: &ValueDesignElem) -> Option<Value> {
    if value.len() == 0 {
        return None
    }


    match design.value_type {
        ValueType::Text     => Some(Value::Text(value.to_string())),
        ValueType::Uri      => Some(Value::Uri(value.to_string())),
        _                   => None,
    }
}
