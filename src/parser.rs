//! Return parsed property from a `Line`.

use std::iter::Iterator;
use std::io::BufRead;
use std::error::Error;
use std::fmt;

use ::line;

/// A parsed `Line`.
///
/// It's only split a raw line into the mains elements:
/// - name: Property name.
/// - params: Vector of (key,value) parameter.
/// - value: Property Value.
#[derive(Debug, Clone)]
pub struct LineParsed {
    pub name: String,
    pub params: Option<Vec<(String, String)>>,
    pub value: String,
}

impl LineParsed {
    pub fn new() -> LineParsed {
        LineParsed {
            name: String::new(),
            params: None,
            value: String::new(),
        }
    }

    pub fn set_name(&mut self, name: &str) {
        self.name = name.to_uppercase();
    }

    pub fn set_value(&mut self, value: &str) {
        self.value = value.to_string();
    }

    pub fn set_parameters(&mut self, params: Option<Vec<(String, String)>>) {
        self.params = params;
    }
}

impl fmt::Display for LineParsed {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
               "name: {}\nparams: {:?}\nvalue: {}",
               self.name,
               self.params,
               self.value)
    }
}

/// A splitted `Line`.
#[derive(Debug, Clone)]
pub struct LineParser<B> {
    line_reader: line::LineReader<B>,
}

impl<B: BufRead> LineParser<B> {
    pub fn new(line_reader: line::LineReader<B>) -> LineParser<B> {
        LineParser { line_reader: line_reader }
    }


    fn parse(&self, line: line::Line) -> Result<LineParsed, ParseError> {
        let mut property = LineParsed::new();

        // Parse name.
        let name = self.parse_name(line.as_str())
            .ok_or(ParseError::MissingName)?;
        property.set_name(name);

        // Parse value
        let value = self.parse_value(line.as_str())
            .ok_or(ParseError::MissingValue)?;
        property.set_value(value);

        // Parse parameters.
        let parameters = self.parse_parameters(line.as_str())?;
        property.set_parameters(parameters);

        Ok(property)

    }

    /// Return the name from the given `Line`.
    fn parse_name<'a>(&self, line: &'a str) -> Option<&'a str> {
        let end_name_index;

        let param_index = line.find(::PARAM_DELIMITER).unwrap_or(usize::max_value());
        let value_index = line.find(::VALUE_DELIMITER).unwrap_or(usize::max_value());

        if param_index < value_index {
            end_name_index = param_index;
        } else if value_index != usize::max_value() {
            end_name_index = value_index;
        } else {
            return None;
        }

        Some(line.split_at(end_name_index).0)
    }


    /// Return the value from the given `Line`.
    fn parse_value<'a>(&self, line: &'a str) -> Option<&'a str> {
        let value_index = match line.find(::VALUE_DELIMITER) {
            Some(val) => val + 1, // Jump the VALUE_DELIMITER
            None => return None,
        };

        if value_index < line.len() {
            Some(line.split_at(value_index).1)
        } else {
            None
        }
    }

    /// Return the parameters from the given `Line`.
    fn parse_parameters(&self, line: &str) -> Result<Option<Vec<(String, String)>>, ParseError> {
        let params_str;
        let mut param_list = Vec::new();

        let end_param_index = line.find(::VALUE_DELIMITER)
            .ok_or(ParseError::MissingValueDelimiter)?;

        let start_param_index = match line.find(::PARAM_DELIMITER) {
            Some(val) => val + 1, // Jump the PARAM_DELIMITER sign.
            None => return Ok(None), // there is no params.
        };

        // The first PARAM_DELIMITER is after the VALUE_DELIMITER so it's in the
        // value section and not a PARAM_DELIMITER.
        if start_param_index > end_param_index {
            return Ok(None);
        }

        if start_param_index > line.len() {
            // There is not parameters after PARAM_DELIMITER.
            return Err(ParseError::InvalidParamFormat);
        }

        unsafe {
            params_str = line.slice_unchecked(start_param_index, end_param_index);
        }

        let key_value_list: Vec<&str> = params_str.split(::PARAM_DELIMITER).collect();

        for key_value in key_value_list {

            let mut elem_list = key_value.split(::PARAM_NAME_DELIMITER);

            let key = elem_list.next()
                .and_then(|key| Some(key.to_uppercase()))
                .ok_or(ParseError::InvalidParamFormat)?;

            let value = elem_list.next()
                .and_then(|value| Some(value.to_string()))
                .ok_or(ParseError::InvalidParamFormat)?;

            param_list.push((key, value));

        }
        Ok(Some((param_list)))
    }
}

impl<B: BufRead> Iterator for LineParser<B> {
    type Item = Result<LineParsed, ParseError>;

    fn next(&mut self) -> Option<Result<LineParsed, ParseError>> {
        self.line_reader
            .next()
            .map(|line| self.parse(line))
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ParseError {
    MissingValueDelimiter,
    MissingName,
    MissingValue,
    InvalidParamFormat,
}

impl Error for ParseError {
    fn description(&self) -> &str {
        match *self {
            ParseError::MissingValueDelimiter => "Missing value delimiter.",
            ParseError::MissingName => "Missing name.",
            ParseError::MissingValue => "Missing value.",
            ParseError::InvalidParamFormat => "Invalid parameter format.",
        }
    }

    fn cause(&self) -> Option<&Error> {
        None
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
