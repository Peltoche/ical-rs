

use std::collections::HashMap;

use value::ValueType;
use property::PropertyType;


/// A list of `DesignSet`. It list all the possible properties and their
/// format.
pub type DesignSet = HashMap<PropertyType, DesignElem>;


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

/// Identical to `find` but will only match values when they are not
/// preceded by a backslash character.
pub fn unescaped_find(buffer: &str, start: usize, pat: char) -> Option<usize> {
    let buf_chars = buffer.char_indices();

    let res = buf_chars
        .skip(start)
        .find(|&(index, value)| {
            if value == pat {
                if buffer.as_bytes().get(index - 1) != Some(&b'\\') {
                    return true;
                }
            }

            return false;
        });

    if let Some(index_char) = res {
       Some(index_char.0)
    } else {
        None
    }
}
