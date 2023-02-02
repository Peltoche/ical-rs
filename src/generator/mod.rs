//! Generates iCal- or vCard-output.
//!
//! A fair knowledge of the iCal/vCard-standards is necessary to create usable files,
//! even so the [IcalEventBuilder](struct.IcalCalendarBuilder.html) and
//! [IcalVcardBuilder](struct.IcalVcardBuilder.html) helps to stick to the
//! formalities.
//!
//! * iCal: <https://tools.ietf.org/html/rfc5545>
//! * vCard: <https://tools.ietf.org/html/rfc2426>
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

    /// Creates a param for a [`Property`](property/struct.Property.html).
    ///
    /// # Example
    /// ```
    /// # #[macro_use] extern crate ical;
    /// let param : (String, Vec<String>) = ical_param!("param2", "pvalue1", "pvalue2");
    /// assert_eq!(format!("{:?}", param), "(\"param2\", [\"pvalue1\", \"pvalue2\"])");
    /// ```
    #[macro_export]
    macro_rules! ical_param {
        ($key:literal, $($prop:expr),+) => {
            (String::from($key), vec![$(String::from($prop),)+])
        };
    }

    /// Creates a [`Property`](property/struct.Property.html) for use with
    /// [IcalCalendarBuilder](generator/struct.IcalCalendarBuilder.html),
    /// [IcalEventBuilder](generator/struct.IcalEventBuilder.html),
    /// [IcalVcardBuilder](generator/struct.IcalVcardBuilder.html),
    /// `IcalTodo`, `IcalJournal` ...
    ///
    /// # Example
    /// ```
    /// # #[macro_use] extern crate ical;
    /// # use ical::property::Property;
    /// let prop = ical_property!(
    ///             "NAME",
    ///             "value",
    ///             ical_param!("param2", "pvalue1", "pvalue2"),
    ///             ical_param!("param3", "pvalue3")
    ///         );
    /// let debug_output = "Property { \
    ///     name: \"NAME\", \
    ///     params: Some([\
    ///         (\"param2\", [\"pvalue1\", \"pvalue2\"]), \
    ///         (\"param3\", [\"pvalue3\"])\
    ///     ]), \
    ///     value: Some(\"value\") \
    /// }";
    /// assert_eq!(debug_output, format!("{:?}", prop));
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
                params: Some(vec![$($params,)+]),
            }
        };
    }
}
