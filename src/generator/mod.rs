//! Emits an ICAL calendar.
//!
//! Wraps the result of the `PropertyParser` into components.
//!
//! Each component contains properties (ie: `Property`) or sub-components.
//!
//!

mod calendar_builder;
mod event_builder;
mod ical;
mod vcard_builder;

#[cfg(feature = "ical")]
pub use self::calendar_builder::*;
#[cfg(feature = "ical")]
pub use self::event_builder::*;
#[cfg(any(feature = "ical", feature = "vcard"))]
pub use self::ical::*;
#[cfg(feature = "vcard")]
pub use self::vcard_builder::*;
#[cfg(feature = "ical")]
pub use crate::parser::ical::component::{IcalCalendar, IcalEvent};
#[cfg(feature = "vcard")]
pub use crate::parser::vcard::component::VcardContact;
pub use crate::property::Property;

mod helper {

    /// Creates a param for a `Property`.
    ///
    /// # Example
    /// ```
    /// # #[macro_use] extern crate ical;
    ///
    /// # fn main() {
    /// let param : (String, Vec<String>) = ical_param!("param2", "pvalue1", "pvalue2");
    /// assert_eq!(format!("{:?}", param), "(\"param2\", [\"pvalue1\", \"pvalue2\"])");
    /// # }
    /// ```
    #[macro_export]
    macro_rules! ical_param {
        ($key:literal, $($prop:expr),+) => {
            (String::from($key), {
                let mut v: Vec<String> = Vec::new();
                $(v.push(String::from($prop));)+
                v
            })
        };
    }

    /// Creates a `Property` for `IcalCalendar`, `IcalEvent`, `IcalTodo`, `IcalJournal` ...
    ///
    /// # Example
    /// ```
    /// # #[macro_use] extern crate ical;
    /// # use ical::property::Property;
    ///
    /// # fn main() {
    /// print!("{:?}", ical_property!(
    ///             "NAME",
    ///             "value",
    ///             ical_param!("param2", "pvalue1", "pvalue2"),
    ///             ical_param!("param2", "pvalue3")
    ///         ));
    /// # }
    /// ```
    #[macro_export]
    macro_rules! ical_property {
        ($name:literal, $value:expr) => {
            Property {
                name: String::from($name),
                value: Some($value.into()),
                params: None,
            }
        };
        ($name:literal, $value:expr, $($params:expr),+) => {
            Property {
                name: String::from($name),
                value: Some(String::from($value)),
                params: Some({
                    let mut v: Vec<(String, Vec<String>)> = Vec::new();
                    $(v.push($params);)+
                    v
                })
            }
        };
    }
}
