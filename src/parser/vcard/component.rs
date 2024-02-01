// Sys mods
use std::cell::RefCell;
use std::io::BufRead;

#[cfg(feature = "serde-derive")]
extern crate serde;

// Internal mods
use crate::parser::{Component, ParserError};
use crate::property::{Property, PropertyParser};

#[derive(Debug, Clone, Default, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde-derive", derive(serde::Serialize, serde::Deserialize))]
/// A VCARD contact.
pub struct VcardContact {
    pub properties: Vec<Property>,
}

impl VcardContact {
    pub fn new() -> VcardContact {
        VcardContact {
            properties: Vec::new(),
        }
    }
}

impl Component for VcardContact {
    fn add_property(&mut self, property: Property) {
        self.properties.push(property);
    }

    fn get_property<'c>(&'c self, name: &str) -> Option<&'c Property> {
        self.properties.iter().find(|p| p.name == name)
    }

    fn get_property_mut<'c>(&'c mut self, name: &str) -> Option<&'c mut Property> {
        self.properties.iter_mut().find(|p| p.name == name)
    }

    fn add_sub_component<B: BufRead>(
        &mut self,
        _: &str,
        _: &RefCell<PropertyParser<B>>,
    ) -> Result<(), ParserError> {
        Err(ParserError::InvalidComponent)
    }
}
