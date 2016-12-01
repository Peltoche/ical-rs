
mod line;
mod parser;
mod ical;


pub const PARAM_VALUE_DELIMITER: char = ',';
pub const VALUE_DELIMITER: char = ':';
pub const PARAM_DELIMITER: char = ';';
pub const PARAM_NAME_DELIMITER: char = '=';

pub use ical::IcalReader;
