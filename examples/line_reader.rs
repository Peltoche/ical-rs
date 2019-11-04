extern crate ical;

use std::fs::File;
use std::io::BufReader;

fn main() {
    let buf = BufReader::new(File::open("./tests/ressources/ical_input.ics").unwrap());

    let reader = ical::LineReader::new(buf);

    for line in reader {
        println!("{:?}", line);
    }
}
