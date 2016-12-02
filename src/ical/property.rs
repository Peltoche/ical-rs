
use std::error::Error;
use std::fmt;

use super::super::parser;
use super::param;

#[derive(Debug, Clone)]
pub struct Property {
    name: String,
    params: Vec<param::IcalParam>,
    value: String,
}

impl Property {
    pub fn parse(line: parser::LineParsed) -> Result<Property, PropertyError> {
        let mut params: Vec<param::IcalParam> = Vec::new();

        if let Some(list) = line.params {
            for param in list {
                params.push(param::IcalParam::new(param.0, param.1)?);
            }
        }

        Ok(Property {
            name: line.name,
            params: params,
            value: line.value,
        })
    }
}


#[derive(Debug, PartialEq, Clone, Copy)]
pub enum PropertyError {
    UnknownProperty,
    ParamError(param::ParamError),
}

impl Error for PropertyError {
    fn description(&self) -> &str {
        match *self {
            PropertyError::UnknownProperty => "Unknown property.",
            PropertyError::ParamError(ref err) => err.description(),
        }
    }

    fn cause(&self) -> Option<&Error> {
        None
    }
}

impl fmt::Display for PropertyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}

impl From<param::ParamError> for PropertyError {
    fn from(err: param::ParamError) -> PropertyError {
        PropertyError::ParamError(err)
    }
}
