

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
    pub name:   property::Type,
    pub params: param::Container,
    pub value:  value::Container,
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
pub enum Type {
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

impl ToJson for Type {
    fn to_json(&self) -> Json {
       Json::String(self.to_string())
    }
}


impl Type {
    /// Match a string an return the  corresponding `Type`.
    pub fn from_str(input: &str) -> Result<Type, PropertyError> {
        match input.to_lowercase().as_str() {
            "adr"           => Ok(Type::Adr),
            "anniversary"   => Ok(Type::Anniversary),
            "bday"          => Ok(Type::Bday),
            "caladruri"     => Ok(Type::Caladruri),
            "caluri"        => Ok(Type::Caluri),
            "clientpidmap"  => Ok(Type::Clientpidmap),
            "email"         => Ok(Type::Email),
            "fburl"         => Ok(Type::Fburl),
            "fn"            => Ok(Type::Fn),
            "gender"        => Ok(Type::Gender),
            "geo"           => Ok(Type::Geo),
            "impp"          => Ok(Type::Impp),
            "key"           => Ok(Type::Key),
            "kind"          => Ok(Type::Kind),
            "lang"          => Ok(Type::Lang),
            "logo"          => Ok(Type::Logo),
            "member"        => Ok(Type::Member),
            "n"             => Ok(Type::N),
            "nickname"      => Ok(Type::Nickname),
            "note"          => Ok(Type::Note),
            "org"           => Ok(Type::Org),
            "photo"         => Ok(Type::Photo),
            "related"       => Ok(Type::Related),
            "rev"           => Ok(Type::Rev),
            "role"          => Ok(Type::Role),
            "sound"         => Ok(Type::Sound),
            "source"        => Ok(Type::Source),
            "tel"           => Ok(Type::Tel),
            "title"         => Ok(Type::Title),
            "tz"            => Ok(Type::Tz),
            "xml"           => Ok(Type::Xml),
            _               => Err(PropertyError::UnknownProperty),
        }
    }

    /// Match a `Type `an return the  corresponding string.
    fn to_string(&self) -> String {
        match self {
           &Type::Adr           => "ADR",
           &Type::Anniversary   => "ANNIVERSARY",
           &Type::Bday          => "BDAY",
           &Type::Caladruri     => "CALADRURI",
           &Type::Caluri        => "CALURI",
           &Type::Clientpidmap  => "CLIENTPIDMAP",
           &Type::Email         => "EMAIL",
           &Type::Fburl         => "FBURL",
           &Type::Fn            => "FN",
           &Type::Gender        => "GENDER",
           &Type::Geo           => "GEO",
           &Type::Impp          => "IMPP",
           &Type::Key           => "KEY",
           &Type::Kind          => "KIND",
           &Type::Lang          => "LANG",
           &Type::Logo          => "LOGO",
           &Type::Member        => "MEMBER",
           &Type::N             => "N",
           &Type::Nickname      => "NICKNAME",
           &Type::Note          => "NOTE",
           &Type::Org           => "ORG",
           &Type::Photo         => "PHOTO",
           &Type::Related       => "RELATED",
           &Type::Rev           => "REV",
           &Type::Role          => "ROLE",
           &Type::Sound         => "SOUND",
           &Type::Source        => "SOURCE",
           &Type::Tel           => "TEL",
           &Type::Title         => "TITLE",
           &Type::Tz            => "TZ",
           &Type::Xml           => "XML",
        }.to_string()
    }
}


pub const DEFAULT_TYPE_TEXT: value::DesignElem = value::DesignElem {
    value_type:         value::Type::Text,
    multi_value:        None,
    allowed_types:      None,
    structured_value:   None,
};

pub const DEFAULT_TYPE_TEXT_MULTI: value::DesignElem = value::DesignElem {
    value_type:         value::Type::Text,
    multi_value:        Some(','),
    allowed_types:      None,
    structured_value:   None,
};

pub const DEFAULT_TYPE_TEXT_STRUCTURED : value::DesignElem = value::DesignElem {
    value_type:         value::Type::Text,
    multi_value:        None,
    allowed_types:      None,
    structured_value:   Some(';'),
};

pub const DEFAULT_TYPE_INTEGER: value::DesignElem = value::DesignElem {
    value_type:       value::Type::Integer,
    multi_value:        None,
    allowed_types:      None,
    structured_value:   None,
};

//pub const DEFAULT_TYPE_DATETIME: DesignElem = DesignElem {
    //value_type:       value::Type::DateTime,
    //multi_value:        None,
    //allowed_types:      None,
    //structured_value:   None,
//};

pub const DEFAULT_TYPE_URI: value::DesignElem = value::DesignElem {
    value_type:         value::Type::Uri,
    multi_value:        None,
    allowed_types:      None,
    structured_value:   None,
};

//pub const DEFAULT_TYPE_UTC_OFFSET: DesignElem = DesignElem {
    //value_type:       value::Type::UtcOffset,
    //multi_value:        None,
    //allowed_types:      None,
    //structured_value:   None,
//};



pub fn get_vcard_properties() -> value::Design {
    let mut v_design = HashMap::with_capacity(31);

    v_design.insert(Type::Adr, value::DesignElem {
        value_type:         value::Type::Text,
        multi_value:        Some(','),
        allowed_types:      None,
        structured_value:   Some(';'),
    });
    v_design.insert(Type::Anniversary, value::DesignElem {
        value_type:         value::Type::DateAndOrTime,
        multi_value:        None,
        allowed_types:      Some(vec![value::Type::DateTime, value::Type::Date, value::Type::Text]),
        structured_value:   None,
    });
    v_design.insert(Type::Bday, value::DesignElem {
        value_type:         value::Type::DateAndOrTime,
        multi_value:        None,
        allowed_types:      Some(vec![value::Type::DateTime, value::Type::Date, value::Type::Text]),
        structured_value:   None,
    });
    v_design.insert(Type::Caladruri, DEFAULT_TYPE_URI);
    v_design.insert(Type::Caluri, DEFAULT_TYPE_URI);
    v_design.insert(Type::Clientpidmap, DEFAULT_TYPE_TEXT_STRUCTURED);
    v_design.insert(Type::Email, DEFAULT_TYPE_TEXT);
    v_design.insert(Type::Fburl, DEFAULT_TYPE_URI);
    v_design.insert(Type::Fn, DEFAULT_TYPE_TEXT);
    v_design.insert(Type::Gender, DEFAULT_TYPE_TEXT_STRUCTURED);
    v_design.insert(Type::Geo, DEFAULT_TYPE_URI);
    v_design.insert(Type::Impp, DEFAULT_TYPE_URI);
    v_design.insert(Type::Key, DEFAULT_TYPE_URI);
    v_design.insert(Type::Kind, DEFAULT_TYPE_TEXT);
    v_design.insert(Type::Lang, value::DesignElem{
        value_type:         value::Type::LanguageTag,
        multi_value:        None,
        allowed_types:      None,
        structured_value:   None,
    });
    v_design.insert(Type::Logo, DEFAULT_TYPE_URI);
    v_design.insert(Type::Member, DEFAULT_TYPE_URI);
    v_design.insert(Type::N, value::DesignElem{
        value_type:         value::Type::Text,
        multi_value:        Some(','),
        allowed_types:      None,
        structured_value:   Some(';'),
    });
    v_design.insert(Type::Nickname, DEFAULT_TYPE_TEXT_MULTI);
    v_design.insert(Type::Note, DEFAULT_TYPE_TEXT);
    v_design.insert(Type::Org, value::DesignElem{
        value_type:         value::Type::Text,
        multi_value:        None,
        allowed_types:      None,
        structured_value:   Some(';'),
    });
    v_design.insert(Type::Photo, DEFAULT_TYPE_URI);
    v_design.insert(Type::Related, DEFAULT_TYPE_URI);
    v_design.insert(Type::Rev, value::DesignElem{
        value_type:         value::Type::Timestamp,
        multi_value:        None,
        allowed_types:      None,
        structured_value:   None,
    });
    v_design.insert(Type::Role, DEFAULT_TYPE_TEXT);
    v_design.insert(Type::Sound, DEFAULT_TYPE_URI);
    v_design.insert(Type::Source, DEFAULT_TYPE_URI);
    v_design.insert(Type::Tel, value::DesignElem{
        value_type:         value::Type::Uri,
        multi_value:        None,
        allowed_types:      Some(vec![value::Type::Uri, value::Type::Text]),
        structured_value:   None,
    });
    v_design.insert(Type::Title, DEFAULT_TYPE_TEXT);
    v_design.insert(Type::Tz, value::DesignElem{
        value_type:         value::Type::Text,
        multi_value:        None,
        allowed_types:      Some(vec![value::Type::Uri, value::Type::Text, value::Type::UtcOffset]),
        structured_value:   None,
    });
    v_design.insert(Type::Xml, DEFAULT_TYPE_TEXT);

    v_design
}



pub fn get_vcard_param_properties() -> param::DesignSet {

    let mut p_design = HashMap::with_capacity(7);

    p_design.insert(param::Name::Language, param::DesignElem{
        design:             Some(DEFAULT_TYPE_TEXT),
        allowed_values:     None,
        allow_name:         false,
        allow_iana_token:   false,
    });
    p_design.insert(param::Name::Value, param::DesignElem {
        design:             None,
        allowed_values:     Some(
            vec!["text", "uri", "date", "time", "date-time",
            "date-and-or-time", "timestamp", "boolean", "integer", "float",
            "utc-offset", "language-tag"]
        ),
        allow_name: true,
        allow_iana_token: true,
    });
    p_design.insert(param::Name::Pref, param::DesignElem {
        design:             Some(DEFAULT_TYPE_INTEGER),
        allowed_values:     None,
        allow_name: true,
        allow_iana_token: true,
    });
    p_design.insert(param::Name::AltId, param::DesignElem {
        design:             Some(DEFAULT_TYPE_TEXT),
        allowed_values:     None,
        allow_name: true,
        allow_iana_token: true,
    });
    p_design.insert(param::Name::Type, param::DesignElem {
        design:             Some(DEFAULT_TYPE_TEXT_MULTI),
        allowed_values:     None,
        allow_name:         false,
        allow_iana_token:   false,
    });
    p_design.insert(param::Name::Mediatype, param::DesignElem {
        design:             Some(DEFAULT_TYPE_TEXT),
        allowed_values:     None,
        allow_name: true,
        allow_iana_token: true,
    });
    p_design.insert(param::Name::Calscale, param::DesignElem {
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
