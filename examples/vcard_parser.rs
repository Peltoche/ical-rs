extern crate ical;

use std::fs::File;
use std::io::BufReader;

fn main() {
    let buf = BufReader::new(File::open("./tests/ressources/vcard_input.vcf").unwrap());

    let reader = ical::VcardParser::new(buf);

    for line in reader {
        println!("{:?}", line);
    }
}
