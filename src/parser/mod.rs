//! Wrapper around `PropertyParser`
//!
//! #### Warning
//!   The parsers (`VcardParser` / `IcalParser`) only parse the content and set to uppercase
//!   the case-insensitive fields.  No checks are made on the fields validity.
//!
//!

pub mod ical;
pub mod vcard;

// Sys mods
use std::cell::RefCell;
use std::io::BufRead;

// Internal mods
use crate::property::{Property, PropertyError, PropertyParser};

#[derive(Debug, Error)]
pub enum ParserError {
    #[error("invalid component")]
    InvalidComponent,
    #[error("incomplete object")]
    NotComplete,
    #[error("missing header")]
    MissingHeader,
    #[error("property error")]
    PropertyError(#[from] PropertyError),
}

/// An interface for an Ical/Vcard component.
///
/// It take a `PropertyParser` and fill the component with. It's also able to create
/// sub-component used by event and alarms.
pub trait Component {
    /// Add the givent sub component.
    fn add_sub_component<B: BufRead>(
        &mut self,
        value: &str,
        line_parser: &RefCell<PropertyParser<B>>,
    ) -> Result<(), ParserError>;

    /// Add the givent property.
    fn add_property(&mut self, property: Property);

    /// Parse the content from `line_parser` and fill the component with.
    fn parse<B: BufRead>(
        &mut self,
        line_parser: &RefCell<PropertyParser<B>>,
    ) -> Result<(), ParserError> {
        loop {
            let line: Property;

            {
                line = match line_parser.borrow_mut().next() {
                    Some(val) => val.map_err(|e| ParserError::PropertyError(e))?,
                    None => return Err(ParserError::NotComplete.into()),
                };
            }

            match line.name.as_str() {
                "END" => break,
                "BEGIN" => match line.value {
                    Some(v) => self.add_sub_component(v.as_str(), line_parser)?,
                    None => return Err(ParserError::NotComplete.into()),
                },

                _ => self.add_property(line),
            };
        }

        Ok(())
    }
}
