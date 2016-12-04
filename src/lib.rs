
#[cfg(any(feature = "line-parser", feature = "line-reader"))]
pub mod line;

#[cfg(any(feature = "ical-parser", feature = "vcard-parser"))]
pub mod parser;

pub const PARAM_VALUE_DELIMITER: char = ',';
pub const VALUE_DELIMITER: char = ':';
pub const PARAM_DELIMITER: char = ';';
pub const PARAM_NAME_DELIMITER: char = '=';


#[cfg(feature = "ical-parser")]
pub use parser::ical::IcalParser;

#[cfg(feature = "vcard-parser")]
pub use parser::vcard::VcardParser;

#[cfg(feature = "line-parser")]
pub use line::parser::LineParser;

#[cfg(feature = "line-reader")]
pub use line::reader::LineReader;
