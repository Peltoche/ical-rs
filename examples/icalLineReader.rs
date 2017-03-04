extern crate ical;

use std::io::BufReader;
use std::fs::File;

fn main() {
    let buf = BufReader::new(File::open("/tmp/component.ics").unwrap());

    let reader = ical::IcalParser::new(buf);

    for line in reader {
        println!("{:?}", line);
    }
}
