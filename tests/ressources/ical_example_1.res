IcalCalendar {
 properties: [
Property { name: "PRODID", params: None, value: Some("-//Microsoft Corporation//Outlook 16.0 MIMEDIR//EN") },
 Property { name: "VERSION", params: None, value: Some("2.0") },
 Property { name: "METHOD", params: None, value: Some("PUBLISH") },
 Property { name: "X-MS-OLK-FORCEINSPECTOROPEN", params: None, value: Some("TRUE") }],
 events: [
IcalEvent { properties: [
Property { name: "CLASS", params: None, value: Some("PUBLIC") },
 Property { name: "CREATED", params: None, value: Some("20210511T063845Z") },
 Property { name: "DESCRIPTION", params: None, value: Some("Einwahldaten folgen in der Veranstaltungswoche \\nSeminartitel:Software-QS-Cast - Application Performance Monitoring\\nDatum: 27.Mai 2021\\nUhrzeit: 10:30 - ca.12:00 Uhr  \\n \\n") },
 Property { name: "DTEND", params: Some([("TZID", ["W. Europe Standard Time"])]), value: Some("20210527T120000") },
 Property { name: "DTSTAMP", params: None, value: Some("20210511T063845Z") },
 Property { name: "DTSTART", params: Some([("TZID", ["W. Europe Standard Time"])]), value: Some("20210527T103000") },
 Property { name: "LAST-MODIFIED", params: None, value: Some("20210511T063845Z") },
 Property { name: "PRIORITY", params: None, value: Some("5") }, Property { name: "SEQUENCE", params: None, value: Some("0") },
 Property { name: "SUMMARY", params: Some([("LANGUAGE", ["de"])]), value: Some("Software-QS-Cast Application Performance Monitoring") },
 Property { name: "TRANSP", params: None, value: Some("OPAQUE") },
 Property { name: "UID", params: None, value: Some("040000008200E000*************00800000000*****************00000000000000010000000********************************") },
 Property { name: "X-MICROSOFT-CDO-BUSYSTATUS", params: None, value: Some("BUSY") },
 Property { name: "X-MICROSOFT-CDO-IMPORTANCE", params: None, value: Some("1") },
 Property { name: "X-MICROSOFT-DISALLOW-COUNTER", params: None, value: Some("FALSE") },
 Property { name: "X-MS-OLK-CONFTYPE", params: None, value: Some("0") }],
 alarms: [
IcalAlarm { properties: [
Property { name: "TRIGGER", params: None, value: Some("-PT15M") },
 Property { name: "ACTION", params: None, value: Some("DISPLAY") },
 Property { name: "DESCRIPTION", params: None, value: Some("Reminder") }] }] }],
 alarms: [], todos: [], journals: [], free_busys: [],
 timezones: [IcalTimeZone { properties: [Property { name: "TZID", params: None, value: Some("W. Europe Standard Time") }],
 transitions: [IcalTimeZoneTransition { transition: STANDARD, properties: [Property { name: "DTSTART", params: None, value: Some("16011028T030000") },
 Property { name: "RRULE", params: None, value: Some("FREQ=YEARLY;BYDAY=-1SU;BYMONTH=10") },
 Property { name: "TZOFFSETFROM", params: None, value: Some("+0200") },
 Property { name: "TZOFFSETTO", params: None, value: Some("+0100") }] },
 IcalTimeZoneTransition { transition: DAYLIGHT, properties: [Property { name: "DTSTART", params: None, value: Some("16010325T020000") }, Property { name: "RRULE", params: None, value: Some("FREQ=YEARLY;BYDAY=-1SU;BYMONTH=3") }, Property { name: "TZOFFSETFROM", params: None, value: Some("+0100") }, Property { name: "TZOFFSETTO", params: None, value: Some("+0200") }] }] }] }
