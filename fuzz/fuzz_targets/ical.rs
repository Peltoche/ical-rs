#![no_main]

extern crate libfuzzer_sys;
extern crate ical;


use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
     let c = std::io::Cursor::new(data);
    for _ in ical::IcalParser::new(c) {} 
});
