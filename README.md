[![license](http://img.shields.io/badge/license-Apache%20v2-orange.svg)](https://raw.githubusercontent.com/Peltoche/ical-rs/master/LICENSE)
[![Build Status](https://travis-ci.org/Peltoche/ical-rs.svg?branch=master)](https://travis-ci.org/Peltoche/ical-rs)
[![Latest version](https://img.shields.io/crates/v/ical.svg)](https://crates.io/crates/ical)
[![Documentation](https://docs.rs/ical/badge.svg)](https://docs.rs/ical)

# ical-rs

This library parses the iCalendar format defined in [RFC5545](http://tools.ietf.org/html/rfc5545), as well as similar formats like vCard.

There are probably some issues to be taken care of, but the library should work for most cases. 
If you like to help out and would like to discuss any API changes, please [contact me](dev@halium.fr) or create an issue.

Initially, the goal was to port the JavaScript [ical.js](https://github.com/mozilla-comm/ical.js) library.
Many code/algorithms were taken from it at first; but in order to but more “Rusty”, a complete rewrite was made.

## [Documentation](https://peltoche.github.io/ical-rs/ical/)

## Installing

Put this in your `Cargo.toml`:

```toml
[dependencies]
ical = "0.10"
```


## Overview

There are several ways to use the `ical` crate, depending on the level of parsing you want. 
Some new wrappers/formatters could appear in future releases.

By default all the features are included, but you can include only the features you need in your project.

#### Warning
  The parsers (`PropertyParser` and `IcalParser`) only parse the content and uppercase the case-insensitive fields.
  No checks are made on the fields’ validity.


### `IcalParser` / `VcardParser`

Wraps the result of the `PropertyParser` into components.

Each component can contains properties (ie: `Property`) or sub-components.

* The `IcalParser` returns `IcalCalendar`
* The `VcardParser` returns `VcardContact`

Cargo.toml:
```toml
[dependencies.ical]
version = "0.10"
default-features = false
features = ["ical", "vcard"]
```

Code:
```rust
extern crate ical;

use std::io::BufReader;
use std::fs::File;

fn main() {
    let buf = BufReader::new(File::open("/tmp/component.ics")
        .unwrap());

    let reader = ical::IcalParser::new(buf);

    for line in reader {
        println!("{:?}", line);
    }
}
```

Output:
```
IcalCalendar {
  properties: [],
  events: [
    IcalEvent {
      properties: [ Property { ... }, ... ],
      alarms: [
        IcalAlarm {
          properties: [ Property { ... } ]
        }
      ]
    }
  ],
  alarms: [],
  todos: [],
  journals: [],
  free_busys: [],
  timezones: []
}
```

### PropertyParser

Parse the result of `LineReader` into three parts:

- The name of the line attribute formatted in uppercase.
- A vector of `(key, value)` tuples for the parameters:
    - The param key is formatted in uppercase.
    - The param value is untouched.
- The property value is untouched.

It work for both the vCard and iCal formats.

#### Example:

Cargo.toml:
```toml
[dependencies.ical]
version = "0.10"
default-features = false
features = ["property"]
```

Code:
```rust
extern crate ical;

use std::io::BufReader;
use std::fs::File;

fn main() {
    let buf = BufReader::new(File::open("/tmp/component.ics")
        .unwrap());

    let reader = ical::PropertyParser::from_reader(buf);

    for line in reader {
        println!("{:?}", line);
    }
}
```

Input -> Output:
```
begin:VCALENDAR                           Ok(Property { name: "BEGIN", params: None, value: Some("VCALENDAR") })
ATTENDEE;cn=FooBar:mailto:foo3@bar    ->  Ok(Property { name: "ATTENDEE", params: Some([("CN", "FooBar")]), value: Some("mailto:foo3@bar") })
DESCRIPTION:                              Ok(Property { name: "DESCRIPTION": params: None, value: None })
END:VCALENDAR                             Ok(Property { name: "END", params: None, value: Some("VCALENDAR") })
```

### `LineReader`

This is a very low-level parser. It cleans empty lines and unfolds them.

It work for both the vCard and iCal formats.

#### Example:

Cargo.toml:
```toml
[dependencies.ical]
version = "0.10"
default-features = false
features = ["line"]
```

Code:
```rust
extern crate ical;

use std::io::BufReader;
use std::fs::File;

fn main() {
    let buf = BufReader::new(File::open("/tmp/component.ics")
        .unwrap());

    let reader = ical::LineReader::new(buf);

    for line in reader {
        println!("{}", line);
    }
}
```

Input -> Output:

```
BEGIN:VCALENDAR        Line 0: BEGIN:VCALENDAR
BEGIN:VEVENT           Line 1: BEGIN:VEVENT
SUMMARY:foo and   ->   Line 3: SUMMARY:foo andbar
 bar
END:VEVENT             Line 4: END:VEVENT
END:VCALENDAR          Line 5: END:VCALENDAR
```

### Generator

The other way to use `ical` is to generate ical/ics files. Builder
for Events, Calendar and VCards ensure filling of mandatory fields.

A fair knowledge of the iCal-standards is necessary to create usable
ics-files, even so the `IcalEventBuilder` helps to stick to the
formalities.

Cargo.toml:
```toml
[dependencies.ical]
version = "0.10"
default-features = false
features = ["ical", "vcard", "generator"]
```

Code:
```rust
extern crate ical;

use crate::ical::{generator::*, *};

fn main() {
  let mut cal = IcalCalendarBuilder::version("2.0")
          .gregorian()
          .prodid("-//ical-rs//github.com//")
          .build();

  let event = IcalEventBuilder::tzid("Europe/Berlin")
          .uid("UID for identifying this event.")
          .changed("20210115")
          .one_day("20220101")
          .set(ical_property!("SUMMARY", "New Year"))
          .build();
  cal.events.push(event);

  print!("{}", cal.generate());
}
```
