//! Parse the result of `LineReader` into parts.
//!
//! Split the result of `LineReader` into property. A property contains:
//! - A name formated in uppercase.
//! - An optional list of parameters represented by a vector of `(key/value)` tuple . The key is
//! formatted in uppercase and the value stay untouched.
//! - A value stay untouched.
//!
//! It work for both the Vcard and Ical format.
//!
//! #### Warning
//!   The parsers `PropertyParser` only parse the content and set to uppercase the case-insensitive
//!   fields. No checks are made on the fields validity.
//!
//! # Examples
//!
//! ```toml
//! [dependencies.ical]
//! version = "0.3.*"
//! default-features = false
//! features = ["property"]
//! ```
//!
//! ```rust
//! extern crate ical;
//!
//! use std::io::BufReader;
//! use std::fs::File;
//!
//! let buf = BufReader::new(File::open("./tests/ressources/vcard_input.vcf")
//!     .unwrap());
//!
//! let reader = ical::PropertyParser::from_reader(buf);
//!
//! for line in reader {
//!     println!("{:?}", line);
//! }
//! ```

// Sys mods
use std::fmt;
use std::io::BufRead;
use std::iter::Iterator;

// Internal mods
use crate::line::{Line, LineReader};

#[derive(Debug, Error)]
pub enum PropertyError {
    #[error("Line {}: Missing property name.", line)]
    MissingName { line: usize },
    #[error("Line {}: Missing a closing quote.", line)]
    MissingClosingQuote { line: usize },
    #[error("Line {}: Missing a \"{}\" delimiter.", line, delimiter)]
    MissingDelimiter { line: usize, delimiter: char },
    #[error("Line {}: Missing content after \"{}\".", line, letter)]
    MissingContentAfter { line: usize, letter: char },
    #[error("Line {}: Missing a parameter key.", line)]
    MissingParamKey { line: usize },
}

/// A VCARD/ICAL property.
#[derive(Debug, Clone, Default)]
pub struct Property {
    /// Property name.
    pub name: String,
    /// Property list of parameters.
    pub params: Option<Vec<(String, Vec<String>)>>,
    /// Property value.
    pub value: Option<String>,
}

impl Property {
    /// Return a new `Property` object.
    pub fn new() -> Property {
        Property {
            name: String::new(),
            params: None,
            value: None,
        }
    }
}

impl fmt::Display for Property {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "name: {}\nparams: {:?}\nvalue: {:?}",
            self.name, self.params, self.value
        )
    }
}

/// Take a `LineReader` and return a list of `Property`.
#[derive(Debug, Clone)]
pub struct PropertyParser<B> {
    line_reader: LineReader<B>,
}

impl<B: BufRead> PropertyParser<B> {
    /// Return a new `PropertyParser` from a `LineReader`.
    pub fn new(line_reader: LineReader<B>) -> PropertyParser<B> {
        PropertyParser { line_reader }
    }

    /// Return a new `PropertyParser` from a `Reader`.
    pub fn from_reader(reader: B) -> PropertyParser<B> {
        let line_reader = LineReader::new(reader);

        PropertyParser { line_reader }
    }

    fn parse(&self, line: Line) -> Result<Property, PropertyError> {
        let mut property = Property::new();

        let mut to_parse = line.as_str();

        // Parse name.
        let end_name_index;

        let mut param_index = to_parse
            .find(::PARAM_DELIMITER)
            .unwrap_or(usize::max_value());
        let mut value_index = to_parse
            .find(::VALUE_DELIMITER)
            .unwrap_or(usize::max_value());

        if param_index < value_index && param_index != 0 {
            end_name_index = param_index;
        } else if value_index != usize::max_value() && value_index != 0 {
            end_name_index = value_index;
        } else {
            return Err(PropertyError::MissingName {
                line: line.number(),
            })
            .into();
        }

        {
            let split = to_parse.split_at(end_name_index);
            property.name = split.0.to_string();
            to_parse = split.1;
        }

        // Parse parameters.
        value_index = to_parse
            .find(::VALUE_DELIMITER)
            .unwrap_or(usize::max_value());
        param_index = to_parse
            .find(::PARAM_DELIMITER)
            .unwrap_or(usize::max_value());

        // If there is a PARAM_DELIMITER and it not after the VALUE_DELIMITER
        // there is arguments.
        if param_index != usize::max_value() && value_index > param_index {
            let mut param_list = Vec::new();

            while to_parse.starts_with(::PARAM_DELIMITER) {
                to_parse = to_parse.trim_start_matches(::PARAM_DELIMITER);

                // Split the param key and the rest of the line
                let mut param_elements = to_parse.splitn(2, ::PARAM_NAME_DELIMITER);

                let key = param_elements
                    .next()
                    .and_then(|key| {
                        if key.is_empty() {
                            return None;
                        }

                        Some(key)
                    })
                    .ok_or_else(|| PropertyError::MissingParamKey {
                        line: line.number(),
                    })?;

                to_parse =
                    param_elements
                        .next()
                        .ok_or_else(|| PropertyError::MissingDelimiter {
                            delimiter: ::PARAM_NAME_DELIMITER,
                            line: line.number(),
                        })?;

                let mut values = Vec::new();

                let mut i = 10;

                // Parse parameter value.
                while i > 0 {
                    i -= 1;
                    if to_parse.starts_with('"') {
                        // This is a dquoted value. (NAME:Foo="Bar":value)
                        let mut elements = to_parse.splitn(3, ::PARAM_QUOTE).skip(1);
                        // unwrap is safe here as we have already check above if there is on '"'.
                        values.push(
                            elements
                                .next()
                                .ok_or_else(|| PropertyError::MissingClosingQuote {
                                    line: line.number(),
                                })?
                                .to_string(),
                        );

                        to_parse =
                            elements
                                .next()
                                .ok_or_else(|| PropertyError::MissingClosingQuote {
                                    line: line.number(),
                                })?
                    } else {
                        // This is a 'raw' value. (NAME;Foo=Bar:value)

                        // Try to find the next param separator.
                        let param_delimiter = to_parse
                            .find(::PARAM_DELIMITER)
                            .unwrap_or(usize::max_value());
                        let value_delimiter = to_parse
                            .find(::VALUE_DELIMITER)
                            .unwrap_or(usize::max_value());
                        let param_value_delimiter = to_parse
                            .find(::PARAM_VALUE_DELIMITER)
                            .unwrap_or(usize::max_value());

                        let end_param_value = {
                            if param_value_delimiter < value_delimiter
                                && param_value_delimiter < param_delimiter
                            {
                                Ok(param_value_delimiter)
                            } else if param_delimiter < value_delimiter
                                && param_delimiter < param_value_delimiter
                            {
                                Ok(param_delimiter)
                            } else if value_delimiter != usize::max_value() {
                                Ok(value_delimiter)
                            } else {
                                Err(PropertyError::MissingContentAfter {
                                    letter: ::PARAM_NAME_DELIMITER,
                                    line: line.number(),
                                })
                            }
                        }?;

                        let elements = to_parse.split_at(end_param_value);
                        values.push(elements.0.to_string());
                        to_parse = elements.1;
                    }

                    if !to_parse.starts_with(::PARAM_VALUE_DELIMITER) {
                        break;
                    }

                    to_parse = to_parse.trim_start_matches(::PARAM_VALUE_DELIMITER);
                }

                param_list.push((key.to_uppercase(), values));
            }

            property.params = Some(param_list);
        } else {
            property.params = None;
        }

        // Parse value
        to_parse = to_parse.trim_start_matches(::VALUE_DELIMITER);
        if to_parse.is_empty() {
            property.value = None;
        } else {
            property.value = Some(to_parse.to_string());
        }

        Ok(property)
    }
}

impl<B: BufRead> Iterator for PropertyParser<B> {
    type Item = Result<Property, PropertyError>;

    fn next(&mut self) -> Option<Result<Property, PropertyError>> {
        self.line_reader.next().map(|line| self.parse(line))
    }
}
