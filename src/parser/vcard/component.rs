
use std::io::BufRead;
use std::cell::RefCell;

use parser::{Component, ParseError};
use line::parser::{LineParsed, LineParser};

pub struct VcardContact {
    pub properties: Vec<LineParsed>,
}

impl VcardContact {
    pub fn new() -> VcardContact {
        VcardContact {
            properties: Vec::new(),
        }
    }
}

impl Component for VcardContact {
    fn add_property(&mut self, property: LineParsed) {
        self.properties.push(property);
    }

    fn add_sub_component<B: BufRead>(&mut self,
                                     value: &str,
                                     _: &RefCell<LineParser<B>>)
                                     -> Result<(), ParseError> {
        return Err(ParseError::InvalidComponent(value.to_string()));
    }
}
