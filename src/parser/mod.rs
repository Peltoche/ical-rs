//! Wrapper around `LineParser`
//!
//! #### Warning
//!   The parsers (VcardParser / IcalParser) only parse the content and set to uppercase
//!   the case-insensitive fields.  No checks are made on the fields validity.
//!
//!

pub mod ical;
pub mod vcard;

// Sys mods
use std::io::BufRead;
use std::cell::RefCell;

// Internal mods
use line::parser;
use ::errors::*;

/// An interface for an Ical/Vcard component.
///
/// It take a `LineParser` and fill the component with. It's also able to create
/// sub-component used by event and alarms.
pub trait Component {
    /// Add the givent sub component.
    fn add_sub_component<B: BufRead>(&mut self,
                                     value: &str,
                                     line_parser: &RefCell<parser::LineParser<B>>)
                                     -> Result<()>;

    /// Add the givent property.
    fn add_property(&mut self, property: parser::LineParsed);

    /// Parse the content from `line_parser` and fill the component with.
    fn parse<B: BufRead>(&mut self,
                         line_parser: &RefCell<parser::LineParser<B>>)
                         -> Result<()> {

        loop {
            let line: parser::LineParsed;

            {
                line = match line_parser.borrow_mut().next() {
                    Some(val) => val,
                    None => return Err(ErrorKind::NotComplete.into()),
                }?;
            }

            match line.name.as_str() {
                "END" => break,
                "BEGIN" => {
                    match line.value {
                        Some(v) => self.add_sub_component(v.as_str(), line_parser)?,
                        None => return Err(ErrorKind::NotComplete.into()),
                    }
                }

                _ => self.add_property(line),
            };
        }

        Ok(())
    }
}
