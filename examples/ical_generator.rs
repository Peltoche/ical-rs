extern crate ical;

#[cfg(all(feature = "ical", feature = "generator"))]
use std::{fs::File, io::BufReader};

#[cfg(all(feature = "ical", feature = "generator"))]
fn main() {
    let buf = BufReader::new(File::open("./tests/ressources/ical_input.ics").unwrap());

    let reader = ical::IcalParser::new(buf);

    for line in reader {
        println!("{:?}", &line);
        match &line {
            Err(_) => {}
            Ok(ical) => {
                let ev = ical as &dyn ical::generator::Emitter;
                println!("{}", ev.generate());
            }
        }
    }
}

#[cfg(not(all(feature = "ical", feature = "generator")))]
fn main() {
    println!("feature=\"generator\" not set");
}
