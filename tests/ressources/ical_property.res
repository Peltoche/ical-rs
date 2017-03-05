Property { name: "BEGIN", params: None, value: Some("VCALENDAR") }
Property { name: "CALSCALE", params: None, value: Some("GREGORIAN") }
Property { name: "PRODID", params: None, value: Some("-//Example Inc.//Example Calendar//EN") }
Property { name: "VERSION", params: None, value: Some("2.0") }
Property { name: "BEGIN", params: None, value: Some("VEVENT") }
Property { name: "DTSTAMP", params: None, value: Some("20080205T191224Z") }
Property { name: "DTSTART", params: Some([("VALUE", ["DATE"])]), value: Some("20081006") }
Property { name: "SUMMARY", params: None, value: Some("Planning meeting") }
Property { name: "UID", params: None, value: Some("4088E990AD89CB3DBB484909") }
Property { name: "BEGIN", params: None, value: Some("VALARM") }
Property { name: "SUMMARY", params: None, value: Some("escaped\\, comma and\\; semicolon\\nnewline") }
Property { name: "END", params: None, value: Some("VALARM") }
Property { name: "END", params: None, value: Some("VEVENT") }
Property { name: "END", params: None, value: Some("VCALENDAR") }
Property { name: "BEGIN", params: None, value: Some("VCALENDAR") }
Property { name: "ATTENDEE", params: Some([("DELEGATED-TO", ["mailto:foo7@bar", "mailto:foo8@bar"]), ("CN", ["Foo, Bar"])]), value: Some("mailto:foo1@bar") }
Property { name: "ATTENDEE", params: Some([("DELEGATED-TO", ["mailto:foo7@bar", "mailto:foo8@bar"]), ("CN", ["Foo; Bar"])]), value: Some("mailto:foo2@bar") }
Property { name: "ATTENDEE", params: Some([("CN", ["Foo, Bar"])]), value: Some("mailto:foo3@bar") }
Property { name: "ATTENDEE", params: Some([("CN", ["Foo; Bar"])]), value: Some("mailto:foo4@bar") }
Property { name: "ATTENDEE", params: Some([("DELEGATED-TO", ["mailto:foo7@bar"]), ("CN", ["Foo, Bar"])]), value: Some("mailto:foo5@bar") }
Property { name: "ATTENDEE", params: Some([("DELEGATED-TO", ["mailto:foo7@bar"]), ("CN", ["Foo; Bar"])]), value: Some("mailto:foo6@bar") }
Property { name: "ATTENDEE", params: Some([("ROLE", ["REQ-PARTICIPANT;foo"]), ("DELEGATED-FROM", ["mailto:bar@baz.com"]), ("PARTSTAT", ["ACCEPTED"]), ("RSVP", ["TRUE"])]), value: Some("mailto:foo@bar.com") }
Property { name: "X-FOO", params: Some([("PARAM1", ["VAL1"])]), value: Some("FOO;BAR") }
Property { name: "X-FOO2", params: Some([("PARAM1", ["VAL1"]), ("PARAM2", ["VAL2"])]), value: Some("FOO;BAR") }
Property { name: "X-BAR", params: Some([("PARAM1", ["VAL1:FOO"])]), value: Some("BAZ;BAR") }
Property { name: "X-BAZ", params: Some([("PARAM1", ["VAL1:FOO"]), ("PARAM2", ["VAL2"])]), value: Some("BAZ;BAR") }
Property { name: "X-BAZ2", params: Some([("PARAM1", ["VAL1"]), ("PARAM2", ["VAL2:FOO"])]), value: Some("BAZ;BAR") }
Property { name: "END", params: None, value: Some("VCALENDAR") }
Property { name: "BEGIN", params: None, value: Some("VCALENDAR") }
Property { name: "CALSCALE", params: None, value: Some("GREGORIAN") }
Property { name: "PRODID", params: None, value: Some("-//Example Inc.//Example Calendar//EN") }
Property { name: "VERSION", params: None, value: Some("2.0") }
Property { name: "BEGIN", params: None, value: Some("VEVENT") }
Property { name: "DTSTAMP", params: None, value: Some("20080205T191224Z") }
Property { name: "DTSTART", params: Some([("VALUE", ["DATE"])]), value: Some("20081006") }
Property { name: "SUMMARY", params: None, value: Some("Missing description value, but includes header") }
Property { name: "DESCRIPTION", params: None, value: None }
Property { name: "UID", params: None, value: Some("4088E990AD89CB3DBB484909") }
Property { name: "END", params: None, value: Some("VEVENT") }
Property { name: "END", params: None, value: Some("VCALENDAR") }

