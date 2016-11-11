

use rustc_serialize::json::{ToJson, Json, Object};
use std::collections::HashMap;

use ::value::{ValueType, ValueContainer};
use ::design::{DesignSet, DesignElem};
use ::param::{ParamDesignSet, ParamDesignElem, ParamName, ParamSet};
use ::parser::ParserError;

#[derive(Debug)]
/// Main struct returning the parsed content of a line.
pub struct Property {
    pub name:   PropertyType,
    pub params: ParamSet,
    pub value:  ValueContainer,
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
    pub fn from_str(input: &str) -> Result<PropertyType, ParserError> {
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
            _               => Err(ParserError::new(
                    format!("Unknow property type: {}", input)
                    )),
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

//pub const MULTIVALUE_DELIMITER: char = ',';
pub const VALUE_DELIMITER: char = ':';
pub const PARAM_DELIMITER: char = ';';
pub const PARAM_NAME_DELIMITER: char = '=';


pub const DEFAULT_TYPE_TEXT: DesignElem = DesignElem {
    value_type:         ValueType::Text,
    multi_value:        None,
    allowed_types:      None,
    structured_value:   None,
};

pub const DEFAULT_TYPE_TEXT_MULTI: DesignElem = DesignElem {
    value_type:         ValueType::Text,
    multi_value:        Some(','),
    allowed_types:      None,
    structured_value:   None,
};

pub const DEFAULT_TYPE_TEXT_STRUCTURED : DesignElem = DesignElem {
    value_type:         ValueType::Text,
    multi_value:        None,
    allowed_types:      None,
    structured_value:   Some(';'),
};

pub const DEFAULT_TYPE_INTEGER: DesignElem = DesignElem {
    value_type:       ValueType::Integer,
    multi_value:        None,
    allowed_types:      None,
    structured_value:   None,
};

//pub const DEFAULT_TYPE_DATETIME: DesignElem = DesignElem {
    //value_type:       ValueType::DateTime,
    //multi_value:        None,
    //allowed_types:      None,
    //structured_value:   None,
//};

pub const DEFAULT_TYPE_URI: DesignElem = DesignElem {
    value_type:         ValueType::Uri,
    multi_value:        None,
    allowed_types:      None,
    structured_value:   None,
};

//pub const DEFAULT_TYPE_UTC_OFFSET: DesignElem = DesignElem {
    //value_type:       ValueType::UtcOffset,
    //multi_value:        None,
    //allowed_types:      None,
    //structured_value:   None,
//};



pub fn get_vcard_properties() -> DesignSet {
    let mut v_design = HashMap::with_capacity(31);

    v_design.insert(PropertyType::Adr, DesignElem {
        value_type:       ValueType::Text,
        multi_value:        Some(','),
        allowed_types:      None,
        structured_value:   Some(';'),
    });
    v_design.insert(PropertyType::Anniversary, DesignElem {
        value_type:       ValueType::DateAndOrTime,
        multi_value:        None,
        allowed_types:      Some(vec![ValueType::DateTime, ValueType::Date, ValueType::Text]),
        structured_value:   None,
    });
    v_design.insert(PropertyType::Bday, DesignElem {
        value_type:       ValueType::DateAndOrTime,
        multi_value:        None,
        allowed_types:      Some(vec![ValueType::DateTime, ValueType::Date, ValueType::Text]),
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
    v_design.insert(PropertyType::Lang, DesignElem{
        value_type:       ValueType::LanguageTag,
        multi_value:        None,
        allowed_types:      None,
        structured_value:   None,
    });
    v_design.insert(PropertyType::Logo, DEFAULT_TYPE_URI);
    v_design.insert(PropertyType::Member, DEFAULT_TYPE_URI);
    v_design.insert(PropertyType::N, DesignElem{
        value_type:       ValueType::Text,
        multi_value:        Some(','),
        allowed_types:      None,
        structured_value:   Some(';'),
    });
    v_design.insert(PropertyType::Nickname, DEFAULT_TYPE_TEXT_MULTI);
    v_design.insert(PropertyType::Note, DEFAULT_TYPE_TEXT);
    v_design.insert(PropertyType::Org, DesignElem{
        value_type:       ValueType::Text,
        multi_value:        None,
        allowed_types:      None,
        structured_value:   Some(';'),
    });
    v_design.insert(PropertyType::Photo, DEFAULT_TYPE_URI);
    v_design.insert(PropertyType::Related, DEFAULT_TYPE_URI);
    v_design.insert(PropertyType::Rev, DesignElem{
        value_type:       ValueType::Timestamp,
        multi_value:        None,
        allowed_types:      None,
        structured_value:   None,
    });
    v_design.insert(PropertyType::Role, DEFAULT_TYPE_TEXT);
    v_design.insert(PropertyType::Sound, DEFAULT_TYPE_URI);
    v_design.insert(PropertyType::Source, DEFAULT_TYPE_URI);
    v_design.insert(PropertyType::Tel, DesignElem{
        value_type:       ValueType::Uri,
        multi_value:        None,
        allowed_types:      Some(vec![ValueType::Uri, ValueType::Text]),
        structured_value:   None,
    });
    v_design.insert(PropertyType::Title, DEFAULT_TYPE_TEXT);
    v_design.insert(PropertyType::Tz, DesignElem{
        value_type:       ValueType::Text,
        multi_value:        None,
        allowed_types:      Some(vec![ValueType::Uri, ValueType::Text, ValueType::UtcOffset]),
        structured_value:   None,
    });
    v_design.insert(PropertyType::Xml, DEFAULT_TYPE_TEXT);

    v_design
}



pub fn get_vcard_param_properties() -> ParamDesignSet {

    let mut p_design = HashMap::new();

    p_design.insert(ParamName::Language, ParamDesignElem{
        design:             Some(DEFAULT_TYPE_TEXT),
        allowed_values:     None,
        allow_name:         false,
        allow_iana_token:   false,
    });
    p_design.insert(ParamName::Value, ParamDesignElem {
        design:             None,
        allowed_values:     Some(
            vec!["text", "uri", "date", "time", "date-time",
            "date-and-or-time", "timestamp", "boolean", "integer", "float",
            "utc-offset", "language-tag"]
        ),
        allow_name: true,
        allow_iana_token: true,
    });
    p_design.insert(ParamName::Pref, ParamDesignElem {
        design:             Some(DEFAULT_TYPE_INTEGER),
        allowed_values:     None,
        allow_name: true,
        allow_iana_token: true,
    });
    p_design.insert(ParamName::AltId, ParamDesignElem {
        design:             Some(DEFAULT_TYPE_TEXT),
        allowed_values:     None,
        allow_name: true,
        allow_iana_token: true,
    });
    p_design.insert(ParamName::Type, ParamDesignElem {
        design:             Some(DEFAULT_TYPE_TEXT_MULTI),
        allowed_values:     None,
        allow_name:         false,
        allow_iana_token:   false,
    });
    p_design.insert(ParamName::Mediatype, ParamDesignElem {
        design:             Some(DEFAULT_TYPE_TEXT),
        allowed_values:     None,
        allow_name: true,
        allow_iana_token: true,
    });
    p_design.insert(ParamName::Calscale, ParamDesignElem {
        design:             Some(DEFAULT_TYPE_TEXT),
        allowed_values:     None,
        allow_name: true,
        allow_iana_token: true,
    });



    p_design
}
