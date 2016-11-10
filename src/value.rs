
use rustc_serialize::json::{ToJson, Json, Array};

#[derive(Debug, PartialEq, Eq)]
pub enum ValueContainer {
    Single(Value),
    Multi(Vec<Value>),
    None,
}

impl ToJson for ValueContainer {
    fn to_json(&self) -> Json {
        match self {
            &ValueContainer::None           => Json::Null,
            &ValueContainer::Single(ref val)    => val.to_json(),
            &ValueContainer::Multi(ref list)    => {
                let mut res = Array::new();

                for elem in list {
                    res.push(elem.to_json());
                }

                Json::Array(res)
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Value {
    Text(String),
    //Uri,
    //Date,
    //Time,
    //DateTime,
    //DateAndOrTime,
    //Timestamp,
    //Boolean,
    //Integer,
    //Float,
    //UtcOffset,
    //LanguageTag,
}

impl ToJson for Value {
    fn to_json(&self) -> Json {
        match self {
            &Value::Text(ref val) => Json::String(val.clone()),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum ValueType{
    Text,
    Uri,
    Date,
    Time,
    DateTime,
    DateAndOrTime,
    Timestamp,
    Boolean,
    Integer,
    Float,
    UtcOffset,
    LanguageTag,
}


