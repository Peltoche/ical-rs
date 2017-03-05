extern crate ical;

use std::io::BufReader;
use std::fs::File;

fn main() {
    let buf = BufReader::new(File::open("./tests/ressources/ical_input.ics").unwrap());

    let reader = ical::PropertyParser::from_reader(buf);

    for line in reader {
        println!("{:?}", line);
    }
}
