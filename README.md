
# ical-rs 0.2.0
===============

[![Build Status](https://travis-ci.org/Peltoche/rust-vcard-ical.svg?branch=master)](https://travis-ci.org/Peltoche/rust-vcard-ical)


This is a library to parse the ICalendar format defined in [RFC5545](http://tools.ietf.org/html/rfc5545), as well asl
similar formats like VCard.

There are probably some issues to be taken care of, but the library should work for most cases. If you like to help out and
would like to discuss any API changes, please [contact me](dev@halium.fr) or create an issue.

The initial goal was to make a port from the [ical.js](https://github.com/mozilla-comm/ical.js) library in JavaScript and
many code/algorithms was taken from it but in order to but more 'Rusty' a complete rewrite have been made.


## Installing

Put this in your `Cargo.toml`:

```toml
[dependencies]
ical = "0.2.0"
```


## Overview

There is several ways to use Ical depending on the level of parsing you want.


### IcalParser

Parse the file into Ical components. Each component contains other sub-components or properties.

A property contains:
- A raw name in uppercase
- A raw value.
- A struct containing vector of formated  parameters.

Code:
```rust
extern crate ical;

use std::io::BufReader;
use std::fs::File;

fn main() {
    let buf = BufReader::new(File::open("/tmp/component.ics")
        .unwrap());

    let reader = ical::IcalReader::new(buf);

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
      properties: [
        Property {
          name: "ATTENDEE",
          params: [
            IcalParam {
              name: Cn,
              values: ["FooBar"]
            }
          ],
          value: "mailto:foo3@bar"
        }
      ],
      alarms: [
        IcalAlarm { properties: [ Property { ... } ] },
        ...
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

### LineParser

Parse the unfolded line into three parts:

- The name of the line attribute formated in uppercase.
- A vector of `(key/value)` tuple for the parameters. The key is formatted in uppercase and the value is untouched.
- The value which stay untouched.

#### Example:

Code:
```rust
extern crate ical;

use std::io::BufReader;
use std::fs::File;

fn main() {
    let buf = BufReader::new(File::open("/tmp/component.ics")
        .unwrap());

    let reader = ical::LineParser::from_reader(buf);

    for line in reader {
        println!("{:?}", line);
    }

```

Input -> Output:
```
BEGIN:VCALENDAR                           Ok(LineParsed { name: "BEGIN", params: None, value: "VCALENDAR" })
ATTENDEE;cn=FooBar:mailto:foo3@bar    ->  Ok(LineParsed { name: "ATTENDEE", params: Some([("CN", "FooBar")]), value: "mailto:foo3@bar" })
END:VCALENDAR                             Ok(LineParsed { name: "END", params: None, value: "VCALENDAR" })
```

### LineReader

This is a very low level parser. It only unfold the lines.

Individual lines within vCard/ICal are delimited by the [RFC5322](http://tools.ietf.org/html/rfc5322) line break.

#### Example:

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



