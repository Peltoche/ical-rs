
// Sys mods
use std::io::BufRead;
use std::cell::RefCell;

// Internal mods
use parser::Component;
use line::parser::{LineParsed, LineParser};
use ::errors::*;

#[derive(Debug, Clone)]
pub struct VcardContact {
    pub properties: Vec<LineParsed>,
}

impl VcardContact {
    pub fn new() -> VcardContact {
        VcardContact { properties: Vec::new() }
    }
}

impl Component for VcardContact {
    fn add_property(&mut self, property: LineParsed) {
        self.properties.push(property);
    }

    fn add_sub_component<B: BufRead>(&mut self,
                                     _: &str,
                                     _: &RefCell<LineParser<B>>)
                                     -> Result<()> {
        return Err(ErrorKind::InvalidComponent.into());
    }
}
