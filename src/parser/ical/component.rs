// Sys mods
use std::cell::RefCell;
use std::io::BufRead;

// Internal mods
use crate::parser::Component;
use crate::parser::ParserError;
use crate::property::{Property, PropertyParser};

#[derive(Debug, Clone, Default)]
/// An ICAL calendar.
pub struct IcalCalendar {
    pub properties: Vec<Property>,
    pub events: Vec<IcalEvent>,
    pub alarms: Vec<IcalAlarm>,
    pub todos: Vec<IcalTodo>,
    pub journals: Vec<IcalJournal>,
    pub free_busys: Vec<IcalFreeBusy>,
    pub timezones: Vec<IcalTimeZone>,
}

impl IcalCalendar {
    pub fn new() -> IcalCalendar {
        IcalCalendar {
            properties: Vec::new(),
            events: Vec::new(),
            alarms: Vec::new(),
            todos: Vec::new(),
            journals: Vec::new(),
            free_busys: Vec::new(),
            timezones: Vec::new(),
        }
    }
}

impl Component for IcalCalendar {
    fn add_property(&mut self, property: Property) {
        self.properties.push(property);
    }

    fn add_sub_component<B: BufRead>(
        &mut self,
        value: &str,
        line_parser: &RefCell<PropertyParser<B>>,
    ) -> Result<(), ParserError> {
        match value {
            "VALARM" => {
                let mut alarm = IcalAlarm::new();
                alarm.parse(line_parser)?;
                self.alarms.push(alarm);
            }
            "VEVENT" => {
                let mut event = IcalEvent::new();
                event.parse(line_parser)?;
                self.events.push(event);
            }
            "VTODO" => {
                let mut todo = IcalTodo::new();
                todo.parse(line_parser)?;
                self.todos.push(todo);
            }
            "VJOURNAL" => {
                let mut journal = IcalJournal::new();
                journal.parse(line_parser)?;
                self.journals.push(journal);
            }
            "VFREEBUSY" => {
                let mut free_busy = IcalFreeBusy::new();
                free_busy.parse(line_parser)?;
                self.free_busys.push(free_busy);
            }
            "VTIMEZONE" => {
                let mut timezone = IcalTimeZone::new();
                timezone.parse(line_parser)?;
                self.timezones.push(timezone);
            }
            _ => return Err(ParserError::InvalidComponent.into()),
        };

        Ok(())
    }
}

#[derive(Debug, Clone, Default)]
pub struct IcalAlarm {
    pub properties: Vec<Property>,
}

impl IcalAlarm {
    pub fn new() -> IcalAlarm {
        IcalAlarm {
            properties: Vec::new(),
        }
    }
}

impl Component for IcalAlarm {
    fn add_property(&mut self, property: Property) {
        self.properties.push(property);
    }

    fn add_sub_component<B: BufRead>(
        &mut self,
        _: &str,
        _: &RefCell<PropertyParser<B>>,
    ) -> Result<(), ParserError> {
        Err(ParserError::InvalidComponent.into())
    }
}

#[derive(Debug, Clone, Default)]
pub struct IcalEvent {
    pub properties: Vec<Property>,
    pub alarms: Vec<IcalAlarm>,
}

impl IcalEvent {
    pub fn new() -> IcalEvent {
        IcalEvent {
            properties: Vec::new(),
            alarms: Vec::new(),
        }
    }
}

impl Component for IcalEvent {
    fn add_property(&mut self, property: Property) {
        self.properties.push(property);
    }

    fn add_sub_component<B: BufRead>(
        &mut self,
        value: &str,
        line_parser: &RefCell<PropertyParser<B>>,
    ) -> Result<(), ParserError> {
        match value {
            "VALARM" => {
                let mut alarm = IcalAlarm::new();
                alarm.parse(line_parser)?;
                self.alarms.push(alarm);
            }
            _ => return Err(ParserError::InvalidComponent.into()),
        };

        Ok(())
    }
}

#[derive(Debug, Clone, Default)]
pub struct IcalJournal {
    pub properties: Vec<Property>,
}

impl IcalJournal {
    pub fn new() -> IcalJournal {
        IcalJournal {
            properties: Vec::new(),
        }
    }
}

impl Component for IcalJournal {
    fn add_property(&mut self, property: Property) {
        self.properties.push(property);
    }

    fn add_sub_component<B: BufRead>(
        &mut self,
        _: &str,
        _: &RefCell<PropertyParser<B>>,
    ) -> Result<(), ParserError> {
        Err(ParserError::InvalidComponent.into())
    }
}

#[derive(Debug, Clone, Default)]
pub struct IcalTodo {
    pub properties: Vec<Property>,
    pub alarms: Vec<IcalAlarm>,
}

impl IcalTodo {
    pub fn new() -> IcalTodo {
        IcalTodo {
            properties: Vec::new(),
            alarms: Vec::new(),
        }
    }
}

impl Component for IcalTodo {
    fn add_property(&mut self, property: Property) {
        self.properties.push(property);
    }

    fn add_sub_component<B: BufRead>(
        &mut self,
        value: &str,
        line_parser: &RefCell<PropertyParser<B>>,
    ) -> Result<(), ParserError> {
        match value {
            "VALARM" => {
                let mut alarm = IcalAlarm::new();
                alarm.parse(line_parser)?;
                self.alarms.push(alarm);
            }
            _ => return Err(ParserError::InvalidComponent.into()),
        };

        Ok(())
    }
}

#[derive(Debug, Clone, Default)]
pub struct IcalTimeZone {
    pub properties: Vec<Property>,
    pub transitions: Vec<IcalTimeZoneTransition>,
}

impl IcalTimeZone {
    pub fn new() -> IcalTimeZone {
        IcalTimeZone {
            properties: Vec::new(),
            transitions: Vec::new(),
        }
    }
}

impl Component for IcalTimeZone {
    fn add_property(&mut self, property: Property) {
        self.properties.push(property);
    }

    fn add_sub_component<B: BufRead>(
        &mut self,
        value: &str,
        line_parser: &RefCell<PropertyParser<B>>,
    ) -> Result<(), ParserError> {
        match value {
            "STANDARD" | "DAYLIGHT" => {
                let mut transition = IcalTimeZoneTransition::new();
                transition.parse(line_parser)?;
                self.transitions.push(transition);
            }
            _ => return Err(ParserError::InvalidComponent.into()),
        };

        Ok(())
    }
}

#[derive(Debug, Clone, Default)]
pub struct IcalTimeZoneTransition {
    pub properties: Vec<Property>,
}

impl IcalTimeZoneTransition {
    pub fn new() -> IcalTimeZoneTransition {
        IcalTimeZoneTransition {
            properties: Vec::new(),
        }
    }
}

impl Component for IcalTimeZoneTransition {
    fn add_property(&mut self, property: Property) {
        self.properties.push(property);
    }

    fn add_sub_component<B: BufRead>(
        &mut self,
        _: &str,
        _: &RefCell<PropertyParser<B>>,
    ) -> Result<(), ParserError> {
        Err(ParserError::InvalidComponent.into())
    }
}

#[derive(Debug, Clone, Default)]
pub struct IcalFreeBusy {
    pub properties: Vec<Property>,
}

impl IcalFreeBusy {
    pub fn new() -> IcalFreeBusy {
        IcalFreeBusy {
            properties: Vec::new(),
        }
    }
}

impl Component for IcalFreeBusy {
    fn add_property(&mut self, property: Property) {
        self.properties.push(property);
    }

    fn add_sub_component<B: BufRead>(
        &mut self,
        _: &str,
        _: &RefCell<PropertyParser<B>>,
    ) -> Result<(), ParserError> {
        Err(ParserError::InvalidComponent.into())
    }
}
