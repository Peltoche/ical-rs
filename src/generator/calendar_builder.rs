use crate::*;
use parser::ical::component::{IcalCalendar, IcalEvent, IcalTimeZone};
use property::Property;

pub struct IcalCalendarBuilder {
    cal: IcalCalendar,
}
pub struct ProdId(IcalCalendarBuilder);
pub struct Finalizer(IcalCalendarBuilder);

/// Builds a new Ical-Calendar.
/// https://tools.ietf.org/html/rfc5545#section-3.6.1
impl IcalCalendarBuilder {
    pub fn version<S: Into<String>>(version: S) -> ProdId {
        let mut e = ProdId(Self {
            cal: IcalCalendar::new(),
        });
        e.0.cal.properties.push(ical_property!("VERSION", version));
        e
    }
}

impl ProdId {
    /// Sets the `PRODID` of the calendar.
    pub fn prodid<S: Into<String>>(mut self, prodid: S) -> Finalizer {
        self.0.cal.properties.push(ical_property!("PRODID", prodid));

        Finalizer(self.0)
    }
}

impl Finalizer {
    /// creates a complete IcalCalendar-object.
    pub fn build(self) -> IcalCalendar {
        self.0.cal
    }

    pub fn set(mut self, property: Property) -> Self {
        self.0.cal.properties.push(property);
        self
    }

    pub fn add_event(mut self, ev: IcalEvent) -> Self {
        self.0.cal.events.push(ev);
        self
    }

    pub fn add_tz(mut self, tz: IcalTimeZone) -> Self {
        self.0.cal.timezones.push(tz);
        self
    }
}
