
use std::collections::HashMap;
use rustc_serialize::json::{ToJson, Json, Object};

use ::design::DesignElem;
use ::value::ValueContainer;

#[derive(Debug)]
pub enum ParamSet {
    None,
    Some(HashMap<ParamName, ValueContainer>)
}

impl ToJson for ParamSet {
    fn to_json(&self) -> Json {
        match self {
            &ParamSet::None    => Json::Null,
            &ParamSet::Some(ref list)    => {
                let mut res = Object::new();

                for (key, val) in list {
                    res.insert(key.to_string(), val.to_json());
                }

                Json::Object(res)
            }
        }
    }
}

/// Regroup all the rules (`ParamDesignElem`) for a type of file (VCard / ICal).
pub type ParamDesignSet = HashMap<ParamName, ParamDesignElem>;


/// Represent the set of rules for a parameter. It contain the expected format
/// for the value or the list of possible values.
#[derive(Debug)]
pub struct ParamDesignElem {
    /// If it's a 'open' parameter (not closed to a list of predetermined
    /// choises), the values is parsed by a `DesignSet` structur.
    pub design:             Option<DesignElem>,

    /// If it's a 'closed' parameter (choices restricted to a predetermined
    /// list), all the possible values a listed her.
    pub allowed_values:     Option<Vec<&'static str>>,


    pub allow_name:         bool,
    pub allow_iana_token:   bool,
}




/// Regroupe all the possible arguments accepted.
#[derive(Debug, PartialEq, Hash, Eq, Clone, Copy)]
pub enum ParamName {
    Language,
    Value,
    Pref,
    Pid,
    Type,
    Geo,
    Tz,
    SortAs,
    CalScale,
    //Any(String),
}

impl ParamName {
    /// Match a string an return the corresponding `ParamName`. The string
    /// is move to lowercase before matching.
    pub fn from_str(input: &str) -> Option<ParamName> {

        match input.to_lowercase().as_str() {
            "language"  => Some(ParamName::Language),
            "value"     => Some(ParamName::Value),
            "pref"      => Some(ParamName::Pref),
            "pid"       => Some(ParamName::Pid),
            "type"      => Some(ParamName::Type),
            "geo"       => Some(ParamName::Geo),
            "tz"        => Some(ParamName::Tz),
            "sortas"    => Some(ParamName::SortAs),
            "calscale"  => Some(ParamName::CalScale),
            _           => None,
        }
    }


    fn to_string(&self) -> String {

        let res = match self {
           &ParamName::Language => "language",
           &ParamName::Value    => "value",
           &ParamName::Pref     => "pref",
           &ParamName::Pid      => "pid",
           &ParamName::Type     => "type",
           &ParamName::Geo      => "geo",
           &ParamName::Tz       => "tz",
           &ParamName::SortAs   => "sortas",
           &ParamName::CalScale => "calscale",
        };

        res.to_string()
    }
}
