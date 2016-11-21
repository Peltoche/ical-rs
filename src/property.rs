
use std::collections::HashMap;
use rustc_serialize::json::{ToJson, Json};
use std::fmt;
use std::error::Error;

use ::value;

/// Regroup all the rules (`DesignElem`) for a type of file (VCard / ICal).
pub type Design = HashMap<Type, DesignElem>;

/// A element of the HashMap `DesignSet`. It represent a the properties of an
/// attribute.
#[derive(Debug)]
pub struct DesignElem {
    /// The default `Type` for the attribute.
    pub value_type:         value::Type,

    /// An attribute can accept several `Type`. In the case allowed_types
    /// take a list of all allowed elements. The value_type attribute will be
    /// tested first and can be listed in `allowed_types`.
    pub allowed_types:      Option<Vec<value::Type>>,
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
    Fburl, Fn,
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
        match *self {
           Type::Adr           => "ADR",
           Type::Anniversary   => "ANNIVERSARY",
           Type::Bday          => "BDAY",
           Type::Caladruri     => "CALADRURI",
           Type::Caluri        => "CALURI",
           Type::Clientpidmap  => "CLIENTPIDMAP",
           Type::Email         => "EMAIL",
           Type::Fburl         => "FBURL",
           Type::Fn            => "FN",
           Type::Gender        => "GENDER",
           Type::Geo           => "GEO",
           Type::Impp          => "IMPP",
           Type::Key           => "KEY",
           Type::Kind          => "KIND",
           Type::Lang          => "LANG",
           Type::Logo          => "LOGO",
           Type::Member        => "MEMBER",
           Type::N             => "N",
           Type::Nickname      => "NICKNAME",
           Type::Note          => "NOTE",
           Type::Org           => "ORG",
           Type::Photo         => "PHOTO",
           Type::Related       => "RELATED",
           Type::Rev           => "REV",
           Type::Role          => "ROLE",
           Type::Sound         => "SOUND",
           Type::Source        => "SOURCE",
           Type::Tel           => "TEL",
           Type::Title         => "TITLE",
           Type::Tz            => "TZ",
           Type::Xml           => "XML",
        }.to_string()
    }
}




/// ParamError handler all the param parsing error.
#[derive(Debug)]
pub enum PropertyError {
    UnacceptedType,
    UnknownType,
    UnknownProperty,
    NotHandled,
}

impl fmt::Display for PropertyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Property error: {}",  self.description())
    }
}

impl Error for PropertyError {
    fn description(&self) -> &str {
        match *self {
            PropertyError::UnknownProperty => "Unknown property.",
            PropertyError::NotHandled => "This property is not handled by this\
                                       protocol or version.",
            PropertyError::UnacceptedType => "The property doesn't accept \
            this type of value.",
            PropertyError::UnknownType  => "Unknown type."
        }
    }

    fn cause(&self) -> Option<&Error> {
        None
    }
}


pub const DEFAULT_TYPE_TEXT: DesignElem = DesignElem {
    value_type:         value::Type::Text,
    allowed_types:      None,
};

pub const DEFAULT_TYPE_TEXT_MULTI: DesignElem = DesignElem {
    value_type:         value::Type::TextMulti,
    allowed_types:      None,
};


//pub const DEFAULT_TYPE_INTEGER: DesignElem = DesignElem {
    //value_type:       value::Type::Integer,
    //allowed_types:      None,
//};

//pub const DEFAULT_TYPE_DATETIME: DesignElem = DesignElem {
    //value_type:       value::Type::DateTime,
    //multi_value:        None,
    //allowed_types:      None,
//};

pub const DEFAULT_TYPE_URI: DesignElem = DesignElem {
    value_type:         value::Type::Uri,
    allowed_types:      None,
};

//pub const DEFAULT_TYPE_UTC_OFFSET: DesignElem = DesignElem {
    //value_type:       value::Type::UtcOffset,
    //multi_value:        None,
    //allowed_types:      None,
//};



pub fn get_vcard_design() -> Design {
    let mut v_design = HashMap::with_capacity(31);

    v_design.insert(Type::Adr, DesignElem {
        value_type:         value::Type::Adr,
        allowed_types:      None,
    });
    v_design.insert(Type::Anniversary, DesignElem {
        value_type:         value::Type::DateAndOrTime,
        allowed_types:      Some(vec![value::Type::DateTime, value::Type::Date, value::Type::Text]),
    });
    v_design.insert(Type::Bday, DesignElem {
        value_type:         value::Type::DateAndOrTime,
        allowed_types:      Some(vec![value::Type::DateTime, value::Type::Date, value::Type::Text]),
    });
    v_design.insert(Type::Caladruri, DEFAULT_TYPE_URI);
    v_design.insert(Type::Caluri, DEFAULT_TYPE_URI);
    v_design.insert(Type::Clientpidmap, DEFAULT_TYPE_TEXT_MULTI);
    v_design.insert(Type::Email, DEFAULT_TYPE_TEXT);
    v_design.insert(Type::Fburl, DEFAULT_TYPE_URI);
    v_design.insert(Type::Fn, DEFAULT_TYPE_TEXT);
    v_design.insert(Type::Gender, DEFAULT_TYPE_TEXT_MULTI);
    v_design.insert(Type::Geo, DEFAULT_TYPE_URI);
    v_design.insert(Type::Impp, DEFAULT_TYPE_URI);
    v_design.insert(Type::Key, DEFAULT_TYPE_URI);
    v_design.insert(Type::Kind, DEFAULT_TYPE_TEXT);
    v_design.insert(Type::Lang, DEFAULT_TYPE_TEXT);
    v_design.insert(Type::Logo, DEFAULT_TYPE_URI);
    v_design.insert(Type::Member, DEFAULT_TYPE_URI);
    v_design.insert(Type::N, DesignElem{
        value_type:         value::Type::N,
        allowed_types:      None,
    });
    v_design.insert(Type::Nickname, DesignElem{
        value_type:         value::Type::TextMultiQuote,
        allowed_types:      None,
    });
    v_design.insert(Type::Note, DEFAULT_TYPE_TEXT);
    v_design.insert(Type::Org, DEFAULT_TYPE_TEXT_MULTI);
    v_design.insert(Type::Photo, DEFAULT_TYPE_URI);
    v_design.insert(Type::Related, DEFAULT_TYPE_URI);
    v_design.insert(Type::Rev, DesignElem{
        value_type:         value::Type::Timestamp,
        allowed_types:      None,
    });
    v_design.insert(Type::Role, DEFAULT_TYPE_TEXT);
    v_design.insert(Type::Sound, DEFAULT_TYPE_URI);
    v_design.insert(Type::Source, DEFAULT_TYPE_URI);
    v_design.insert(Type::Tel, DesignElem{
        value_type:         value::Type::Uri,
        allowed_types:      Some(vec![value::Type::Uri, value::Type::Text]),
    });
    v_design.insert(Type::Title, DEFAULT_TYPE_TEXT);
    v_design.insert(Type::Tz, DesignElem{
        value_type:         value::Type::Text,
        allowed_types:      Some(vec![value::Type::Uri, value::Type::Text, value::Type::UtcOffset]),
    });
    v_design.insert(Type::Xml, DEFAULT_TYPE_TEXT);

    v_design
}
