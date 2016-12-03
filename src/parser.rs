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
    pub params: Option<Vec<(String, Vec<String>)>>,
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

    pub fn from_reader(reader: B) -> LineParser<B> {
        let line_reader = line::LineReader::new(reader);

        LineParser { line_reader: line_reader }
    }


    fn parse(&self, line: line::Line) -> Result<LineParsed, ParseError> {
        let mut property = LineParsed::new();

        // Parse name.
        property.name = self.parse_name(line.as_str())
            .and_then(|name| Some(name.to_string()))
            .ok_or(ParseError::MissingName)?;

        // Parse value
        property.value = self.parse_value(line.as_str())
            .and_then(|value| Some(value.to_string()))
            .ok_or(ParseError::MissingValue)?;

        // Parse parameters.
        property.params = self.parse_parameters(line.as_str())?;

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
    fn parse_parameters(&self, line: &str) -> Result<Option<Vec<(String, Vec<String>)>>, ParseError> {
        let mut param_list = Vec::new();
        let mut params_str;

        let start_value_index = line.find(::VALUE_DELIMITER).unwrap_or(usize::max_value());
        let start_param_index = match line.find(::PARAM_DELIMITER) {
            Some(val) => val,
            None => return Ok(None), // there is no params.
        };

        if start_value_index < start_param_index {
            return Ok(None);
        }

        // Remove the attribue name.
        params_str = line.split_at(start_param_index).1;

        while params_str.starts_with(::PARAM_DELIMITER) {
            let mut elements;
            let name;

            params_str = params_str.trim_left_matches(::PARAM_DELIMITER);

            // Split the first name and the rest of the line
            elements = params_str.splitn(2, ::PARAM_NAME_DELIMITER);
            name = elements.next().ok_or(ParseError::InvalidParamFormat)?;
            params_str = elements.next().ok_or(ParseError::InvalidParamFormat)?;

            let (values, param_tmp) = parse_param_value(vec![], &mut params_str)?;
            params_str = param_tmp;

            param_list.push((name.to_uppercase(), values));

        };

        Ok(Some((param_list)))
    }
}

fn parse_param_value<'a>(mut values: Vec<String>, params_str: &'a str) -> Result<(Vec<String>, &'a str), ParseError> {
    let new_params_str;

    if params_str.starts_with('"') {
        // This is a dquoted value. (NAME:Foo="Bar":value)
        let mut elements = params_str.splitn(3, '"').skip(1);
        values.push(elements.next()
                    .and_then(|value| Some(value.to_string()))
                    .ok_or(ParseError::InvalidParamFormat)?);
        new_params_str = elements.next()
            .ok_or(ParseError::InvalidParamFormat)?;
    } else {
        // This is a 'raw' value. (NAME;Foo=Bar:value)

        // Try to find the next param separator.
        let param_delimiter = params_str.find(::PARAM_DELIMITER).unwrap_or(usize::max_value());
        let value_delimiter = params_str.find(::VALUE_DELIMITER).unwrap_or(usize::max_value());
        let end_param_value = {
            if param_delimiter < value_delimiter {
                Ok(param_delimiter)
            } else if value_delimiter != usize::max_value() {
                Ok(value_delimiter)
            } else {
                Err(ParseError::InvalidParamFormat)
            }
        }?;

        let elements = params_str.split_at(end_param_value);
        values.push(elements.0.to_string());
        new_params_str = elements.1;
    }

    if new_params_str.starts_with(::PARAM_VALUE_DELIMITER) {
        parse_param_value(values, new_params_str.trim_left_matches(::PARAM_VALUE_DELIMITER))
    } else {
        Ok((values, new_params_str))
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
