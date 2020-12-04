use parser::ical::component::{
    IcalAlarm, IcalCalendar, IcalEvent, IcalFreeBusy, IcalJournal, IcalTimeZone,
    IcalTimeZoneTransition, IcalTodo,
};
use property::Property;

pub trait Emitter {
    fn generate(&self) -> String;
}

fn get_value(value: &Option<String>) -> String {
    if value.is_none() {
        return String::from(":");
    }

    // protect comma if needed.
    let str = value.as_ref().unwrap();
    let mut ret = str.clone();
    for (u, _) in str.char_indices().filter(|(_, c)| *c == ',').rev() {
        if u > 0 && str.get(u - 1..u) != Some(&"\\") {
            ret.insert_str(u, "\\");
        }
    }
    String::from(":") + &ret
}

fn split_line<T: Into<String>>(str: T) -> String {
    let mut str = str.into();
    let mut x = 75;
    while x < str.len() {
        str.insert_str(x, "\n ");
        x += 76;
    }
    str
}

fn protect_params(str: &String) -> String {
    if str.contains(',') {
        format!("\"{}\",", str)
    } else {
        format!("{},", str)
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
                    format!(";{}={}", name, value)
                })
                .collect::<String>()
        }
    }
}

impl Emitter for Property {
    fn generate(&self) -> String {
        split_line(self.name.clone() + &get_params(&self.params) + &get_value(&self.value) + "\n")
    }
}

macro_rules! generate_emitter {
    ($struct:ident, $key:literal, $($prop:ident),+) => {
        impl Emitter for $struct {
            fn generate(&self) -> String {
                let mut text = String::from("BEGIN:") + $key + "\n";
                $(text = text + &self.$prop
                .iter()
                .map(|emitter| emitter.generate())
                .collect::<String>();)+

                text + "END:" + $key + "\n"
            }
        }
    };
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
            + "\n"
            + &self
                .properties
                .iter()
                .map(|emitter| emitter.generate())
                .collect::<String>()
            + "END:"
            + key
            + "\n"
    }
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

#[test]
fn split_line_test() {
    let text = "The ability to return a type that is only specified by the trait it impleme\n \
                     nts is especially useful in the context closures and iterators, which we c\n \
                     over in Chapter 13. Closures and iterators create types that only the comp\n \
                     iler knows or types that are very long to specify.";
    assert_eq!(text, split_line(text.replace("\n ", "")));
}
