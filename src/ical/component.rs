
use std::io::BufRead;
use std::cell::RefCell;

use super::super::parser;
use super::property::Property;
use super::IcalError;

pub trait Component {
    fn add_sub_component<B: BufRead>(&mut self,
                                     value: &str,
                                     line_parser: &RefCell<parser::LineParser<B>>)
                                     -> Result<(), IcalError>;
    fn add_property(&mut self, property: Property);

    fn parse<B: BufRead>(&mut self,
                         line_parser: &RefCell<parser::LineParser<B>>)
                         -> Result<(), IcalError> {

        loop {
            let line: parser::LineParsed;

            {
                line = match line_parser.borrow_mut().next() {
                    Some(val) => val,
                    None => return Err(IcalError::NotComplete),
                }?;
            }

            match line.name.as_str() {
                "END" => break,
                "BEGIN" => self.add_sub_component(line.value.as_str(), line_parser)?,
                _ => self.add_property((Property::parse(line)?)),
            };
        }

        Ok(())
    }
}


/// //////////////////////////////////////////////////////////////////
/// Components
/// //////////////////////////////////////////////////////////////////
#[derive(Debug, Clone)]
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

    fn add_sub_component<B: BufRead>(&mut self,
                                     value: &str,
                                     line_parser: &RefCell<parser::LineParser<B>>)
                                     -> Result<(), IcalError> {

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
            _ => return Err(IcalError::InvalidComponent(value.to_string())),

        };

        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct IcalAlarm {
    pub properties: Vec<Property>,
}


impl IcalAlarm {
    pub fn new() -> IcalAlarm {
        IcalAlarm { properties: Vec::new() }
    }
}

impl Component for IcalAlarm {
    fn add_property(&mut self, property: Property) {
        self.properties.push(property);
    }

    fn add_sub_component<B: BufRead>(&mut self,
                                     value: &str,
                                     _: &RefCell<parser::LineParser<B>>)
                                     -> Result<(), IcalError> {
        return Err(IcalError::InvalidComponent(value.to_string()));
    }
}


#[derive(Debug, Clone)]
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

    fn add_sub_component<B: BufRead>(&mut self,
                                     value: &str,
                                     line_parser: &RefCell<parser::LineParser<B>>)
                                     -> Result<(), IcalError> {

        match value {
            "VALARM" => {
                let mut alarm = IcalAlarm::new();
                alarm.parse(line_parser)?;
                self.alarms.push(alarm);
            }
            _ => return Err(IcalError::InvalidComponent(value.to_string())),
        };

        Ok(())
    }
}


#[derive(Debug, Clone)]
pub struct IcalJournal {
    pub properties: Vec<Property>,
}


impl IcalJournal {
    pub fn new() -> IcalJournal {
        IcalJournal { properties: Vec::new() }
    }
}

impl Component for IcalJournal {
    fn add_property(&mut self, property: Property) {
        self.properties.push(property);
    }

    fn add_sub_component<B: BufRead>(&mut self,
                                     value: &str,
                                     _: &RefCell<parser::LineParser<B>>)
                                     -> Result<(), IcalError> {
        return Err(IcalError::InvalidComponent(value.to_string()));
    }
}


#[derive(Debug, Clone)]
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

    fn add_sub_component<B: BufRead>(&mut self,
                                     value: &str,
                                     line_parser: &RefCell<parser::LineParser<B>>)
                                     -> Result<(), IcalError> {

        match value {
            "VALARM" => {
                let mut alarm = IcalAlarm::new();
                alarm.parse(line_parser)?;
                self.alarms.push(alarm);
            }
            _ => return Err(IcalError::InvalidComponent(value.to_string())),

        };

        Ok(())
    }
}


#[derive(Debug, Clone)]
pub struct IcalTimeZone {
    pub properties: Vec<Property>,
}

impl IcalTimeZone {
    pub fn new() -> IcalTimeZone {
        IcalTimeZone { properties: Vec::new() }
    }
}

impl Component for IcalTimeZone {
    fn add_property(&mut self, property: Property) {
        self.properties.push(property);
    }

    fn add_sub_component<B: BufRead>(&mut self,
                                     value: &str,
                                     _: &RefCell<parser::LineParser<B>>)
                                     -> Result<(), IcalError> {
        return Err(IcalError::InvalidComponent(value.to_string()));
    }
}


#[derive(Debug, Clone)]
pub struct IcalFreeBusy {
    pub properties: Vec<Property>,
}

impl IcalFreeBusy {
    pub fn new() -> IcalFreeBusy {
        IcalFreeBusy { properties: Vec::new() }
    }
}

impl Component for IcalFreeBusy {
    fn add_property(&mut self, property: Property) {
        self.properties.push(property);
    }

    fn add_sub_component<B: BufRead>(&mut self,
                                     value: &str,
                                     _: &RefCell<parser::LineParser<B>>)
                                     -> Result<(), IcalError> {
        return Err(IcalError::InvalidComponent(value.to_string()));
    }
}
