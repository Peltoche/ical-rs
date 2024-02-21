use crate::{PARAM_DELIMITER, PARAM_VALUE_DELIMITER, VALUE_DELIMITER};
use parser::ical::component::{
    IcalAlarm, IcalCalendar, IcalEvent, IcalFreeBusy, IcalJournal, IcalTimeZone,
    IcalTimeZoneTransition, IcalTodo,
};
use property::Property;

///
/// Emits the content of the Component in ical-format.
///
pub trait Emitter {
    /// creates a textual-representation of this object and all it's properties
    /// in ical-format.
    fn generate(&self) -> String;
}

fn get_value(value: &Option<String>) -> String {
    VALUE_DELIMITER.to_string() + value.as_ref().unwrap_or(&String::new())
}

pub(crate) fn split_line<T: Into<String>>(str: T) -> String {
    let str = str.into();
    let mut chars = str.chars();
    let mut first = true;
    let sub_string = (0..)
        .map(|_| chars.by_ref().take(if first { first = false; 75 } else { 74 }).collect::<String>())
        .take_while(|s| !s.is_empty())
        .collect::<Vec<_>>();
    sub_string.join("\r\n ")
}

//
// @see: https://tools.ietf.org/html/rfc5545#section-3.3.11
//
// `text = *(TSAFE-CHAR / ":" / DQUOTE / ESCAPED-CHAR)`
//     Folded according to description above
//
// `ESCAPED-CHAR = ("\\" / "\;" / "\," / "\N" / "\n")`
//     \\ encodes \, \N or \n encodes newline
//     \; encodes ;, \, encodes ,
//
// `TSAFE-CHAR = WSP / %x21 / %x23-2B / %x2D-39 / %x3C-5B /
//              %x5D-7E / NON-US-ASCII`
//     Any character except CONTROLs not needed by the current
//     character set, DQUOTE, ";", ":", "\", ","
//
#[allow(clippy::ptr_arg)]
pub(crate) fn protect_params(param: &String) -> String {
    let str = param.as_str();
    let len = str.len() - 1;
    // starts and ends the param with quotes?
    let in_quotes = len > 1 && &str[0..1] == "\"" && &str[len..] == "\"";

    let to_escape: Vec<(usize, char)> = str
        .chars()
        .enumerate()
        .filter(|(_, c)| {
            c == &'\"'
                || c == &'\n'
                || !in_quotes && (c == &';' || c == &':' || c == &',' || c == &'\\')
        })
        .collect();
    let mut ret = param.to_string();
    for (pos, ch) in to_escape.iter().rev() {
        let pos = *pos;
        if ch == &'\n' {
            ret.replace_range(pos..pos + 1, "\\n");
        } else if pos > 0 && pos < len && &str[pos - 1..pos] != "\\" {
            ret.insert(pos, '\\');
        }
    }
    ret + &PARAM_VALUE_DELIMITER.to_string()
}

#[allow(unused)]
mod should {
    use generator::protect_params;
    use generator::split_line;

    #[test]
    fn split_long_line() {
        let text = "The ability to return a type that is only specified by the trait it impleme\r\n \
                     nts is especially useful in the context closures and iterators, which we c\r\n \
                     over in Chapter 13. Closures and iterators create types that only the comp\r\n \
                     iler knows or types that are very long to specify.";
        assert_eq!(text, split_line(text.replace("\r\n ", "")));
    }

    #[test]
    fn split_long_line_multibyte() {
        // the following text includes multibyte characters (UTF-8) at strategic places to ensure
        // split_line would panic if not multibyte aware
        let text = "DESCRIPTION:ABCDEFGHIJ\\n\\nKLMNOPQRSTUVWXYZ123456789üABCDEFGHIJKLMNOPQRS\\n\\n\r\n \
                     TUVWXYZ123456ä7890ABCDEFGHIJKLM\\n\\nNOPQRSTUVWXYZ1234567890ABCDEFGHIJKLMNOP\r\n \
                     QRSTUVWXöYZ1234567890ABCDEFGHIJKLMNOPQRSTUVWX\\n\\nYZ1234567890abcdefghiÜjkl\r\n \
                     m\\nnopqrstuvwx";
        assert_eq!(text, split_line(text.replace("\r\n ", "")));
    }

    #[test]
    fn protect_chars_in_params() {
        assert_eq!(
            protect_params(&String::from("\"value: in quotes;\"")),
            "\"value: in quotes;\","
        );
        assert_eq!(
            protect_params(&String::from("\"value, in quotes\"")),
            "\"value, in quotes\","
        );
        assert_eq!(
            protect_params(&String::from("value, \"with\" something")),
            "value\\, \\\"with\\\" something,"
        );
        assert_eq!(
            protect_params(&String::from("\"Directory; C:\\\\Programme\"")),
            "\"Directory; C:\\\\Programme\","
        );
        assert_eq!(
            protect_params(&String::from("First\nSecond")),
            "First\\nSecond,"
        );
    }
}

fn get_params(params: &Option<Vec<(String, Vec<String>)>>) -> String {
    match params {
        None => String::new(),
        Some(vec) => {
            vec.iter()
                .map(|(name, values)| {
                    let mut value = values.iter().map(protect_params).collect::<String>();
                    value.pop(); // remove last comma
                    format!("{}{}={}", PARAM_DELIMITER, name, value)
                })
                .collect::<String>()
        }
    }
}

impl Emitter for Property {
    fn generate(&self) -> String {
        split_line(self.name.clone() + &get_params(&self.params) + &get_value(&self.value)) + "\r\n"
    }
}

impl Emitter for IcalTimeZoneTransition {
    fn generate(&self) -> String {
        use crate::parser::ical::component::IcalTimeZoneTransitionType::{DAYLIGHT, STANDARD};
        let key = match &self.transition {
            STANDARD => "STANDARD",
            DAYLIGHT => "DAYLIGHT",
        };
        String::from("BEGIN:")
            + key
            + "\r\n"
            + &self
                .properties
                .iter()
                .map(Emitter::generate)
                .collect::<String>()
            + "END:"
            + key
            + "\r\n"
    }
}

macro_rules! generate_emitter {
    ($struct:ident, $key:literal, $($prop:ident),+) => {
        impl Emitter for $struct {
            fn generate(&self) -> String {
                let mut text = String::from("BEGIN:") + $key + "\r\n";
                $(text += &self.$prop
                .iter()
                .map(Emitter::generate)
                .collect::<String>();)+

                text + "END:" + $key + "\r\n"
            }
        }
    };
}

#[cfg(feature = "vcard")]
use parser::vcard::component::VcardContact;

#[cfg(feature = "vcard")]
generate_emitter!(VcardContact, "VCARD", properties);

generate_emitter!(IcalAlarm, "VALARM", properties);
generate_emitter!(IcalFreeBusy, "VFREEBUSY", properties);
generate_emitter!(IcalJournal, "VJOURNAL", properties);
generate_emitter!(IcalEvent, "VEVENT", properties, alarms);
generate_emitter!(IcalTodo, "VTODO", properties, alarms);
generate_emitter!(IcalTimeZone, "VTIMEZONE", properties, transitions);
generate_emitter!(
    IcalCalendar,
    "VCALENDAR",
    properties,
    timezones,
    events,
    alarms,
    todos,
    journals,
    free_busys
);
