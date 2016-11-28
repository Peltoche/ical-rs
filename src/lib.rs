
mod line;
mod parser;


use std::io::BufRead;

pub const VALUE_DELIMITER: char = ':';
pub const PARAM_DELIMITER: char = ';';
pub const PARAM_NAME_DELIMITER: char = '=';

/// Reader returning Ical object from a `BufRead`.
pub struct IcalReader<B> {
    line_parser: parser::LineParser<B>,
}

impl<B: BufRead> IcalReader<B> {
    pub fn new(reader: B) -> IcalReader<B> {
        let line_reader = line::LineReader::new(reader);
        let line_parser = parser::LineParser::new(line_reader);

        IcalReader { line_parser: line_parser }
    }
}

impl<B: BufRead> Iterator for IcalReader<B> {
    type Item = Result<parser::LineParsed, parser::ParseError>;

    fn next(&mut self) -> Option<Result<parser::LineParsed, parser::ParseError>> {
        self.line_parser.next()
    }
}
