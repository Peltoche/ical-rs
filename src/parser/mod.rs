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
use crate::parser::errors::*;
use crate::property::{Property, PropertyParser};

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
    ) -> Result<()>;

    /// Add the givent property.
    fn add_property(&mut self, property: Property);

    /// Parse the content from `line_parser` and fill the component with.
    fn parse<B: BufRead>(&mut self, line_parser: &RefCell<PropertyParser<B>>) -> Result<()> {
        loop {
            let line: Property;

            {
                line = match line_parser.borrow_mut().next() {
                    Some(val) => val,
                    None => return Err(ErrorKind::NotComplete.into()),
                }?;
            }

            match line.name.as_str() {
                "END" => break,
                "BEGIN" => match line.value {
                    Some(v) => self.add_sub_component(v.as_str(), line_parser)?,
                    None => return Err(ErrorKind::NotComplete.into()),
                },

                _ => self.add_property(line),
            };
        }

        Ok(())
    }
}

#[allow(missing_docs)]
pub mod errors {
    //! The parser errors.

    use crate::property;

    error_chain! {
        types {
            Error, ErrorKind, ResultExt, Result;
        }

        foreign_links {
            Property(property::errors::Error);
        }

        errors {

            /// The current component is invalid.
            InvalidComponent {
                description("The current component is invalid.")
                    display("invalid component")
            }

            /// the current object is not complete.
            NotComplete {
                description("The current object is not complete.")
                    display("incomplete object")
            }

            /// A header is missing.
            MissingHeader {
                description("A header is missing.")
                    display("missing header")
            }
        }
    }
}
