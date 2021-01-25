use crate::*;
use parser::ical::component::IcalEvent;
use property::Property;

#[allow(dead_code)]
pub const ICAL_DATE_FORMAT: &str = "%Y%m%dT%H%M%S";

pub struct IcalEventBuilder {
    tzid: String,
    event: IcalEvent,
}
pub struct Uid(IcalEventBuilder);
pub struct DtStamp(IcalEventBuilder);
pub struct DtStart(IcalEventBuilder);
pub struct DtEnd(IcalEventBuilder);
pub struct DtEndDate(IcalEventBuilder);
pub struct Finalizer(IcalEventBuilder);

/// Builds a new Ical-Event.
/// https://tools.ietf.org/html/rfc5545#section-3.6.1
impl IcalEventBuilder {
    pub fn tzid<S: Into<String>>(timezone: S) -> Uid {
        Uid(Self {
            tzid: timezone.into(),
            event: IcalEvent::new(),
        })
    }
}

impl Uid {
    /// Sets the `UID` of the event. Needs to be unique and stable across recreation of
    /// calendars.
    pub fn uid<S: Into<String>>(mut self, uid: S) -> DtStamp {
        self.0.event.properties.push(ical_property!("UID", uid));
        DtStamp(self.0)
    }
}

impl DtStamp {
    /// Sets the `DTSTAMP` of the event. Signals the date of the last change in global TZ.
    /// Example: "20201231T000000"
    ///          `chrono::Local::now().format("%Y%m%dT%H%M%S").to_string()`
    ///
    pub fn changed<S: Into<String>>(mut self, dtstamp: S) -> DtStart {
        self.0.event.properties.push(ical_property!(
            "DTSTAMP",
            dtstamp.into(),
            ical_param!("TZID", &self.0.tzid)
        ));
        DtStart(self.0)
    }

    /// Sets the `DTSTAMP` of the event. Signals the date of the last change in UTC.
    pub fn changed_utc<S: Into<String>>(mut self, dtstamp: S) -> DtStart {
        self.0
            .event
            .properties
            .push(ical_property!("DTSTAMP", dtstamp.into()));
        DtStart(self.0)
    }
}

impl DtStart {
    /// Sets the `DTSTART` of the event. Signals the date of the begin of the event.
    pub fn start<S: Into<String>>(mut self, dtstamp: S) -> DtEnd {
        self.0.event.properties.push(ical_property!(
            "DTSTART",
            dtstamp.into(),
            ical_param!("TZID", &self.0.tzid)
        ));
        DtEnd(self.0)
    }

    pub fn start_day<S: Into<String>>(mut self, dtstamp: S) -> DtEndDate {
        self.0.event.properties.push(ical_property!(
            "DTSTART",
            dtstamp.into(),
            ical_param!("VALUE", "DATE")
        ));
        DtEndDate(self.0)
    }

    pub fn one_day<S: Into<String>>(mut self, dtstamp: S) -> Finalizer {
        self.0.event.properties.push(ical_property!(
            "DTSTART",
            dtstamp.into(),
            ical_param!("VALUE", "DATE")
        ));
        Finalizer(self.0)
    }
}

impl DtEndDate {
    /// Sets the `DTEND` of the event. Since a event from 9:00 - 10:00 has stoppt
    /// at 10 and a new one can start. The `end_day` has to be the next day. This
    /// `value` is **not inclusive**.     
    pub fn end_day<S: Into<String>>(mut self, value: S) -> Finalizer {
        self.0.event.properties.push(ical_property!(
            "DTEND",
            value.into(),
            ical_param!("VALUE", "DATE")
        ));
        Finalizer(self.0)
    }

    /// Rule for the repeating occurrence.
    pub fn repeat_rule<S: Into<String>>(mut self, value: S) -> Finalizer {
        self.0
            .event
            .properties
            .push(ical_property!("RRULE", value.into()));
        Finalizer(self.0)
    }
}

impl DtEnd {
    /// Sets the `DTEND` of the event.
    pub fn end<S: Into<String>>(mut self, value: S) -> Finalizer {
        self.0.event.properties.push(ical_property!(
            "DTEND",
            value.into(),
            ical_param!("TZID", &self.0.tzid)
        ));
        Finalizer(self.0)
    }

    /// Sets the `DURATION` of the event.
    ///  https://tools.ietf.org/html/rfc5545#section-3.8.2.5
    /// `value` starts with `PT` + duration eg. PT45M
    pub fn duration<S: Into<String>>(mut self, value: S) -> Finalizer {
        self.0
            .event
            .properties
            .push(ical_property!("DURATION", value.into()));
        Finalizer(self.0)
    }
}

impl Finalizer {
    pub fn build(self) -> IcalEvent {
        self.0.event
    }

    pub fn set(mut self, property: Property) -> Self {
        self.0.event.properties.push(property);
        self
    }
}

#[allow(unused)]
mod should {
    use crate::*;
    use generator::event_builder::IcalEventBuilder;
    use property::Property;

    #[test]
    fn build_minimal_ical_event() {
        use generator::Emitter;
        let ev = IcalEventBuilder::tzid("Europe/Berlin")
            .uid("UID_@_test")
            .changed("20201201T120423")
            .start("20201206T170000")
            .duration("PT2H45M0S")
            .0
            .event;
        let e = Emitter::generate(&ev);
        //let e = start.format(ICAL_DATE_FORMAT).to_string();
        assert_eq!(
            e,
            "BEGIN:VEVENT\nUID:UID_@_test\nDTSTAMP;TZID=Europe/Berlin:20201201T120423\n\
            DTSTART;TZID=Europe/Berlin:20201206T170000\n\
            DURATION:PT2H45M0S\nEND:VEVENT\n"
        );
    }

    #[test]
    fn build_whole_day_event() {
        use generator::Emitter;
        let expect = "BEGIN:VEVENT\n\
       UID:20070423T123432Z-541111@example.com\n\
       DTSTAMP:20070423T123432Z\n\
       DTSTART;VALUE=DATE:20070628\n\
       DTEND;VALUE=DATE:20070709\n\
       SUMMARY:Festival International de Jazz de Montreal\n\
       TRANSP:TRANSPARENT\n\
       END:VEVENT\n\
      ";
        let event = IcalEventBuilder::tzid("America/Montreal")
            .uid("20070423T123432Z-541111@example.com")
            .changed_utc("20070423T123432Z")
            .start_day("20070628")
            .end_day("20070709")
            .set(ical_property!(
                "SUMMARY",
                "Festival International de Jazz de Montreal"
            ))
            .set(ical_property!("TRANSP", "TRANSPARENT"))
            .build();

        assert_eq!(expect, event.generate());
    }

    #[test]
    fn build_frequent_ical_event() {
        use generator::Emitter;
        let expect = "BEGIN:VEVENT\n\
       UID:19970901T130000Z-123403@example.com\n\
       DTSTAMP:19970901T130000Z\n\
       DTSTART;VALUE=DATE:19971102\n\
       RRULE:FREQ=YEARLY\n\
       SUMMARY:Our Blissful Anniversary\n\
       TRANSP:TRANSPARENT\n\
       CLASS:CONFIDENTIAL\n\
       CATEGORIES:ANNIVERSARY,PERSONAL,SPECIAL OCCASION\n\
       END:VEVENT\n\
      ";
        let event = IcalEventBuilder::tzid("America/Montreal")
            .uid("19970901T130000Z-123403@example.com")
            .changed_utc("19970901T130000Z")
            .start_day("19971102")
            .repeat_rule("FREQ=YEARLY")
            .set(ical_property!("SUMMARY", "Our Blissful Anniversary"))
            .set(ical_property!("TRANSP", "TRANSPARENT"))
            .set(ical_property!("CLASS", "CONFIDENTIAL"))
            .set(ical_property!(
                "CATEGORIES",
                "ANNIVERSARY,PERSONAL,SPECIAL OCCASION"
            ))
            .build();
        assert_eq!(expect, event.generate());
    }
}
