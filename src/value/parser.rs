

use super::{Value, ValueError};

pub fn from_text(input: &str) -> Result<Value, ValueError> {
    Ok(Value::Text(input.to_string()))
}

pub fn from_uri(input: &str) -> Result<Value, ValueError> {
    Ok(Value::Uri(input.to_string()))
}

pub fn from_adr(input: &str) -> Result<Value, ValueError> {
    Ok(Value::Adr(input.to_string()))
}

pub fn from_date(input: &str) -> Result<Value, ValueError> {
    Ok(Value::Date(input.to_string()))
}

pub fn from_date_time(input: &str) -> Result<Value, ValueError> {
    Ok(Value::Date(input.to_string()))
}

pub fn from_date_and_or_time(input: &str) -> Result<Value, ValueError> {
    Ok(Value::Date(input.to_string()))
}

pub fn from_timestamp(input: &str) -> Result<Value, ValueError> {
    Ok(Value::Date(input.to_string()))
}

pub fn from_clientpidmap(input: &str) -> Result<Value, ValueError> {
    Ok(Value::ClientPidMap(input.to_string()))
}

pub fn from_gender(input: &str) -> Result<Value, ValueError> {
    Ok(Value::Gender(input.to_string()))
}

pub fn from_languagetag(input: &str) -> Result<Value, ValueError> {
    Ok(Value::LanguageTag(input.to_string()))
}

pub fn from_n(input: &str) -> Result<Value, ValueError> {
    Ok(Value::N(input.to_string()))
}

pub fn from_nickname(input: &str) -> Result<Value, ValueError> {
    Ok(Value::Nickname(input.to_string()))
}

pub fn from_org(input: &str) -> Result<Value, ValueError> {
    Ok(Value::Org(input.to_string()))
}

pub fn from_utcoffset(input: &str) -> Result<Value, ValueError> {
    Ok(Value::Date(input.to_string()))
}
