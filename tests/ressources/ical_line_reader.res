Line { inner: "BEGIN:VCALENDAR", number: 3 }
Line { inner: "CALSCALE:GREGORIAN", number: 4 }
Line { inner: "PRODID:-//Example Inc.//Example Calendar//EN", number: 5 }
Line { inner: "VERSION:2.0", number: 6 }
Line { inner: "BEGIN:VEVENT", number: 7 }
Line { inner: "DTSTAMP:20080205T191224Z", number: 8 }
Line { inner: "DTSTART;VALUE=DATE:20081006", number: 9 }
Line { inner: "SUMMARY:Planning meeting", number: 10 }
Line { inner: "UID:4088E990AD89CB3DBB484909", number: 11 }
Line { inner: "BEGIN:VALARM", number: 12 }
Line { inner: "SUMMARY:escaped\\, comma and\\; semicolon\\nnewline", number: 13 }
Line { inner: "END:VALARM", number: 14 }
Line { inner: "END:VEVENT", number: 15 }
Line { inner: "END:VCALENDAR", number: 16 }
Line { inner: "BEGIN:VCALENDAR", number: 19 }
Line { inner: "ATTENDEE;DELEGATED-TO=\"mailto:foo7@bar\",\"mailto:foo8@bar\";CN=\"Foo, Bar\":mailto:foo1@bar", number: 20 }
Line { inner: "ATTENDEE;DELEGATED-TO=\"mailto:foo7@bar\",\"mailto:foo8@bar\";CN=\"Foo; Bar\":mailto:foo2@bar", number: 22 }
Line { inner: "ATTENDEE;CN=\"Foo, Bar\":mailto:foo3@bar", number: 24 }
Line { inner: "ATTENDEE;CN=\"Foo; Bar\":mailto:foo4@bar", number: 25 }
Line { inner: "ATTENDEE;DELEGATED-TO=\"mailto:foo7@bar\";CN=\"Foo, Bar\":mailto:foo5@bar", number: 26 }
Line { inner: "ATTENDEE;DELEGATED-TO=\"mailto:foo7@bar\";CN=\"Foo; Bar\":mailto:foo6@bar", number: 27 }
Line { inner: "ATTENDEE;ROLE=\"REQ-PARTICIPANT;foo\";DELEGATED-FROM=\"mailto:bar@baz.com\";PARTSTAT=ACCEPTED;RSVP=TRUE:mailto:foo@bar.com", number: 28 }
Line { inner: "X-FOO;PARAM1=VAL1:FOO;BAR", number: 30 }
Line { inner: "X-FOO2;PARAM1=VAL1;PARAM2=VAL2:FOO;BAR", number: 31 }
Line { inner: "X-BAR;PARAM1=\"VAL1:FOO\":BAZ;BAR", number: 32 }
Line { inner: "X-BAZ;PARAM1=\"VAL1:FOO\";PARAM2=VAL2:BAZ;BAR", number: 33 }
Line { inner: "X-BAZ2;PARAM1=VAL1;PARAM2=\"VAL2:FOO\":BAZ;BAR", number: 34 }
Line { inner: "END:VCALENDAR", number: 35 }
Line { inner: "BEGIN:VCALENDAR", number: 38 }
Line { inner: "CALSCALE:GREGORIAN", number: 39 }
Line { inner: "PRODID:-//Example Inc.//Example Calendar//EN", number: 40 }
Line { inner: "VERSION:2.0", number: 41 }
Line { inner: "BEGIN:VEVENT", number: 42 }
Line { inner: "DTSTAMP:20080205T191224Z", number: 43 }
Line { inner: "DTSTART;VALUE=DATE:20081006", number: 44 }
Line { inner: "SUMMARY:Missing description value, but includes header", number: 45 }
Line { inner: "DESCRIPTION:", number: 46 }
Line { inner: "UID:4088E990AD89CB3DBB484909", number: 47 }
Line { inner: "END:VEVENT", number: 48 }
Line { inner: "END:VCALENDAR", number: 49 }
