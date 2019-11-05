// Sys mods
use std::cell::RefCell;
use std::io::BufRead;

// Internal mods
use crate::parser::{Component, ParserError};
use crate::property::{Property, PropertyParser};

#[derive(Debug, Clone, Default)]
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

    fn add_sub_component<B: BufRead>(
        &mut self,
        _: &str,
        _: &RefCell<PropertyParser<B>>,
    ) -> Result<(), ParserError> {
        Err(ParserError::InvalidComponent.into())
    }
}
