

use rustc_serialize::json::{ToJson, Json, Object};
use std::collections::HashMap;
use std::fmt;
use std::error::Error;

use ::value;
use ::param;
use ::property;

#[derive(Debug)]
/// Main struct returning the parsed content of a line.
pub struct Property {
    pub name:   property::PropertyType,
    pub params: param::ParamSet,
    pub value:  value::ValueContainer,
}

impl ToJson for Property {
    fn to_json(&self) -> Json {
        let mut obj = Object::new();

        obj.insert("name".to_string(), self.name.to_json());
        obj.insert("params".to_string(), self.params.to_json());
        obj.insert("value".to_string(), self.value.to_json());

        Json::Object(obj)
    }
}

/// All the accepted attributes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PropertyType {
    Adr,
    Anniversary,
    Bday,
    Caladruri,
    Caluri,
    Clientpidmap,
    Email,
    Fburl,
    Fn,
    Gender,
    Geo,
    Impp,
    Key,
    Kind,
    Lang,
    Logo,
    Member,
    N,
    Nickname,
    Note,
    Org,
    Photo,
    Related,
    Rev,
    Role,
    Sound,
    Source,
    Tel,
    Title,
    Tz,
    Xml,
}

impl ToJson for PropertyType {
    fn to_json(&self) -> Json {
       Json::String(self.to_string())
    }
}


impl PropertyType {
    /// Match a string an return the  corresponding `PropertyType`.
    pub fn from_str(input: &str) -> Result<PropertyType, PropertyError> {
        match input.to_lowercase().as_str() {
            "adr"           => Ok(PropertyType::Adr),
            "anniversary"   => Ok(PropertyType::Anniversary),
            "bday"          => Ok(PropertyType::Bday),
            "caladruri"     => Ok(PropertyType::Caladruri),
            "caluri"        => Ok(PropertyType::Caluri),
            "clientpidmap"  => Ok(PropertyType::Clientpidmap),
            "email"         => Ok(PropertyType::Email),
            "fburl"         => Ok(PropertyType::Fburl),
            "fn"            => Ok(PropertyType::Fn),
            "gender"        => Ok(PropertyType::Gender),
            "geo"           => Ok(PropertyType::Geo),
            "impp"          => Ok(PropertyType::Impp),
            "key"           => Ok(PropertyType::Key),
            "kind"          => Ok(PropertyType::Kind),
            "lang"          => Ok(PropertyType::Lang),
            "logo"          => Ok(PropertyType::Logo),
            "member"        => Ok(PropertyType::Member),
            "n"             => Ok(PropertyType::N),
            "nickname"      => Ok(PropertyType::Nickname),
            "note"          => Ok(PropertyType::Note),
            "org"           => Ok(PropertyType::Org),
            "photo"         => Ok(PropertyType::Photo),
            "related"       => Ok(PropertyType::Related),
            "rev"           => Ok(PropertyType::Rev),
            "role"          => Ok(PropertyType::Role),
            "sound"         => Ok(PropertyType::Sound),
            "source"        => Ok(PropertyType::Source),
            "tel"           => Ok(PropertyType::Tel),
            "title"         => Ok(PropertyType::Title),
            "tz"            => Ok(PropertyType::Tz),
            "xml"           => Ok(PropertyType::Xml),
            _               => Err(PropertyError::UnknownProperty),
        }
    }

    /// Match a `PropertyType `an return the  corresponding string.
    fn to_string(&self) -> String {
        let res = match self {
           &PropertyType::Adr           => "ADR",
           &PropertyType::Anniversary   => "ANNIVERSARY",
           &PropertyType::Bday          => "BDAY",
           &PropertyType::Caladruri     => "CALADRURI",
           &PropertyType::Caluri        => "CALURI",
           &PropertyType::Clientpidmap  => "CLIENTPIDMAP",
           &PropertyType::Email         => "EMAIL",
           &PropertyType::Fburl         => "FBURL",
           &PropertyType::Fn            => "FN",
           &PropertyType::Gender        => "GENDER",
           &PropertyType::Geo           => "GEO",
           &PropertyType::Impp          => "IMPP",
           &PropertyType::Key           => "KEY",
           &PropertyType::Kind          => "KIND",
           &PropertyType::Lang          => "LANG",
           &PropertyType::Logo          => "LOGO",
           &PropertyType::Member        => "MEMBER",
           &PropertyType::N             => "N",
           &PropertyType::Nickname      => "NICKNAME",
           &PropertyType::Note          => "NOTE",
           &PropertyType::Org           => "ORG",
           &PropertyType::Photo         => "PHOTO",
           &PropertyType::Related       => "RELATED",
           &PropertyType::Rev           => "REV",
           &PropertyType::Role          => "ROLE",
           &PropertyType::Sound         => "SOUND",
           &PropertyType::Source        => "SOURCE",
           &PropertyType::Tel           => "TEL",
           &PropertyType::Title         => "TITLE",
           &PropertyType::Tz            => "TZ",
           &PropertyType::Xml           => "XML",
        };

        res.to_string()
    }
}


pub const DEFAULT_TYPE_TEXT: value::ValueDesignElem = value::ValueDesignElem {
    value_type:         value::ValueType::Text,
    multi_value:        None,
    allowed_types:      None,
    structured_value:   None,
};

pub const DEFAULT_TYPE_TEXT_MULTI: value::ValueDesignElem = value::ValueDesignElem {
    value_type:         value::ValueType::Text,
    multi_value:        Some(','),
    allowed_types:      None,
    structured_value:   None,
};

pub const DEFAULT_TYPE_TEXT_STRUCTURED : value::ValueDesignElem = value::ValueDesignElem {
    value_type:         value::ValueType::Text,
    multi_value:        None,
    allowed_types:      None,
    structured_value:   Some(';'),
};

pub const DEFAULT_TYPE_INTEGER: value::ValueDesignElem = value::ValueDesignElem {
    value_type:       value::ValueType::Integer,
    multi_value:        None,
    allowed_types:      None,
    structured_value:   None,
};

//pub const DEFAULT_TYPE_DATETIME: DesignElem = DesignElem {
    //value_type:       value::ValueType::DateTime,
    //multi_value:        None,
    //allowed_types:      None,
    //structured_value:   None,
//};

pub const DEFAULT_TYPE_URI: value::ValueDesignElem = value::ValueDesignElem {
    value_type:         value::ValueType::Uri,
    multi_value:        None,
    allowed_types:      None,
    structured_value:   None,
};

//pub const DEFAULT_TYPE_UTC_OFFSET: DesignElem = DesignElem {
    //value_type:       value::ValueType::UtcOffset,
    //multi_value:        None,
    //allowed_types:      None,
    //structured_value:   None,
//};



pub fn get_vcard_properties() -> value::ValueDesignSet {
    let mut v_design = HashMap::with_capacity(31);

    v_design.insert(PropertyType::Adr, value::ValueDesignElem {
        value_type:         value::ValueType::Text,
        multi_value:        Some(','),
        allowed_types:      None,
        structured_value:   Some(';'),
    });
    v_design.insert(PropertyType::Anniversary, value::ValueDesignElem {
        value_type:         value::ValueType::DateAndOrTime,
        multi_value:        None,
        allowed_types:      Some(vec![value::ValueType::DateTime, value::ValueType::Date, value::ValueType::Text]),
        structured_value:   None,
    });
    v_design.insert(PropertyType::Bday, value::ValueDesignElem {
        value_type:         value::ValueType::DateAndOrTime,
        multi_value:        None,
        allowed_types:      Some(vec![value::ValueType::DateTime, value::ValueType::Date, value::ValueType::Text]),
        structured_value:   None,
    });
    v_design.insert(PropertyType::Caladruri, DEFAULT_TYPE_URI);
    v_design.insert(PropertyType::Caluri, DEFAULT_TYPE_URI);
    v_design.insert(PropertyType::Clientpidmap, DEFAULT_TYPE_TEXT_STRUCTURED);
    v_design.insert(PropertyType::Email, DEFAULT_TYPE_TEXT);
    v_design.insert(PropertyType::Fburl, DEFAULT_TYPE_URI);
    v_design.insert(PropertyType::Fn, DEFAULT_TYPE_TEXT);
    v_design.insert(PropertyType::Gender, DEFAULT_TYPE_TEXT_STRUCTURED);
    v_design.insert(PropertyType::Geo, DEFAULT_TYPE_URI);
    v_design.insert(PropertyType::Impp, DEFAULT_TYPE_URI);
    v_design.insert(PropertyType::Key, DEFAULT_TYPE_URI);
    v_design.insert(PropertyType::Kind, DEFAULT_TYPE_TEXT);
    v_design.insert(PropertyType::Lang, value::ValueDesignElem{
        value_type:         value::ValueType::LanguageTag,
        multi_value:        None,
        allowed_types:      None,
        structured_value:   None,
    });
    v_design.insert(PropertyType::Logo, DEFAULT_TYPE_URI);
    v_design.insert(PropertyType::Member, DEFAULT_TYPE_URI);
    v_design.insert(PropertyType::N, value::ValueDesignElem{
        value_type:         value::ValueType::Text,
        multi_value:        Some(','),
        allowed_types:      None,
        structured_value:   Some(';'),
    });
    v_design.insert(PropertyType::Nickname, DEFAULT_TYPE_TEXT_MULTI);
    v_design.insert(PropertyType::Note, DEFAULT_TYPE_TEXT);
    v_design.insert(PropertyType::Org, value::ValueDesignElem{
        value_type:         value::ValueType::Text,
        multi_value:        None,
        allowed_types:      None,
        structured_value:   Some(';'),
    });
    v_design.insert(PropertyType::Photo, DEFAULT_TYPE_URI);
    v_design.insert(PropertyType::Related, DEFAULT_TYPE_URI);
    v_design.insert(PropertyType::Rev, value::ValueDesignElem{
        value_type:         value::ValueType::Timestamp,
        multi_value:        None,
        allowed_types:      None,
        structured_value:   None,
    });
    v_design.insert(PropertyType::Role, DEFAULT_TYPE_TEXT);
    v_design.insert(PropertyType::Sound, DEFAULT_TYPE_URI);
    v_design.insert(PropertyType::Source, DEFAULT_TYPE_URI);
    v_design.insert(PropertyType::Tel, value::ValueDesignElem{
        value_type:         value::ValueType::Uri,
        multi_value:        None,
        allowed_types:      Some(vec![value::ValueType::Uri, value::ValueType::Text]),
        structured_value:   None,
    });
    v_design.insert(PropertyType::Title, DEFAULT_TYPE_TEXT);
    v_design.insert(PropertyType::Tz, value::ValueDesignElem{
        value_type:         value::ValueType::Text,
        multi_value:        None,
        allowed_types:      Some(vec![value::ValueType::Uri, value::ValueType::Text, value::ValueType::UtcOffset]),
        structured_value:   None,
    });
    v_design.insert(PropertyType::Xml, DEFAULT_TYPE_TEXT);

    v_design
}



pub fn get_vcard_param_properties() -> param::ParamDesignSet {

    let mut p_design = HashMap::with_capacity(7);

    p_design.insert(param::ParamName::Language, param::ParamDesignElem{
        design:             Some(DEFAULT_TYPE_TEXT),
        allowed_values:     None,
        allow_name:         false,
        allow_iana_token:   false,
    });
    p_design.insert(param::ParamName::Value, param::ParamDesignElem {
        design:             None,
        allowed_values:     Some(
            vec!["text", "uri", "date", "time", "date-time",
            "date-and-or-time", "timestamp", "boolean", "integer", "float",
            "utc-offset", "language-tag"]
        ),
        allow_name: true,
        allow_iana_token: true,
    });
    p_design.insert(param::ParamName::Pref, param::ParamDesignElem {
        design:             Some(DEFAULT_TYPE_INTEGER),
        allowed_values:     None,
        allow_name: true,
        allow_iana_token: true,
    });
    p_design.insert(param::ParamName::AltId, param::ParamDesignElem {
        design:             Some(DEFAULT_TYPE_TEXT),
        allowed_values:     None,
        allow_name: true,
        allow_iana_token: true,
    });
    p_design.insert(param::ParamName::Type, param::ParamDesignElem {
        design:             Some(DEFAULT_TYPE_TEXT_MULTI),
        allowed_values:     None,
        allow_name:         false,
        allow_iana_token:   false,
    });
    p_design.insert(param::ParamName::Mediatype, param::ParamDesignElem {
        design:             Some(DEFAULT_TYPE_TEXT),
        allowed_values:     None,
        allow_name: true,
        allow_iana_token: true,
    });
    p_design.insert(param::ParamName::Calscale, param::ParamDesignElem {
        design:             Some(DEFAULT_TYPE_TEXT),
        allowed_values:     None,
        allow_name: true,
        allow_iana_token: true,
    });



    p_design
}


/// ParamError handler all the param parsing error.
#[derive(Debug)]
pub enum PropertyError {
    UnknownProperty,
}

impl fmt::Display for PropertyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Property error: {}",  self.description())
    }
}

impl Error for PropertyError {
    fn description(&self) -> &str {
        match *self {
            PropertyError::UnknownProperty => "Unknow property.",
        }
    }

    fn cause(&self) -> Option<&Error> {
        None
    }
}
