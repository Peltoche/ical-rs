
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct IcalParam {
    name: ParamName,
    values: Vec<String>,
}

impl IcalParam {
    pub fn new(name: String, value: String) -> Result<IcalParam, ParamError> {
        let values = value.split(::PARAM_VALUE_DELIMITER)
            .map(|val| val.to_string())
            .collect();

        Ok(IcalParam {
            name: ParamName::from_str(name.as_str())?,
            values: values,
        })
    }
}

/// Parameter name.
#[derive(Debug)]
pub enum ParamName {
    AltRep,
    Cn,
    CuType,
    DelegatedFrom,
    DelegatedTo,
    Dir,
    Encoding,
    FmType,
    FbType,
    Language,
    Member,
    PartStat,
    Range,
    Related,
    RelType,
    Role,
    Rsvp,
    SentBy,
    TzId,
    Value,
}

impl ParamName {
    pub fn from_str(input: &str) -> Result<ParamName, ParamError> {
        match input {
            "ALTREP" => Ok(ParamName::AltRep),
            "CN" => Ok(ParamName::Cn),
            "CUTYPE" => Ok(ParamName::CuType),
            "DELEGATED-FROM" => Ok(ParamName::DelegatedFrom),
            "DELEGATED-TO" => Ok(ParamName::DelegatedTo),
            "DIR" => Ok(ParamName::Dir),
            "ENCODING" => Ok(ParamName::Encoding),
            "FMTYPE" => Ok(ParamName::FmType),
            "FBTYPE" => Ok(ParamName::FbType),
            "LANGUAGE" => Ok(ParamName::Language),
            "MEMBER" => Ok(ParamName::Member),
            "PARTSTAT" => Ok(ParamName::PartStat),
            "RANGE" => Ok(ParamName::Range),
            "RELATED" => Ok(ParamName::Related),
            "RELTYPE" => Ok(ParamName::RelType),
            "ROLE" => Ok(ParamName::Role),
            "RSVP" => Ok(ParamName::Rsvp),
            "SENTBY" => Ok(ParamName::SentBy),
            "TZiD" => Ok(ParamName::TzId),
            "VALUE" => Ok(ParamName::Value),
            _ => Err(ParamError::UnknownParam),
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ParamError {
    UnknownParam,
}

impl Error for ParamError {
    fn description(&self) -> &str {
        match *self {
            ParamError::UnknownParam => "Unknown parameter.",
        }
    }

    fn cause(&self) -> Option<&Error> {
        None
    }
}

impl fmt::Display for ParamError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
