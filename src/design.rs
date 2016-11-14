

use std::collections::HashMap;

use value::ValueType;
use property::PropertyType;
use param::{ParamName};



/// A list of `DesignSet`. It list all the possible properties and their
/// format.
pub type DesignSet = HashMap<PropertyType, DesignElem>;

/// Regroup all the rules (`ParamDesignElem`) for a type of file (VCard / ICal).
pub type ParamDesignSet = HashMap<ParamName, ParamDesignElem>;


/// A element of the HashMap `DesignSet`. It represent a the properties of an
/// attribute.
#[derive(Debug)]
pub struct DesignElem {
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

/// Identical to `find` but will only match values when they are not
/// preceded by a backslash character.
pub fn unescaped_find(buffer: &str, start: usize, pat: char) -> Option<usize> {

    //let res = buf_chars
    buffer.char_indices()
        .skip(start)
        .find(|&(index, value)| {
            if value == pat {
                if buffer.as_bytes().get(index - 1) != Some(&b'\\') {
                    return true;
                }
            }

            return false;
        })
    .and_then(|(index, _)| {
       Some(index)
    })

}


/// Internal helper for rfc6868.
pub fn rfc_6868_escape(input: &str) -> String {
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
