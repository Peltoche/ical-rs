extern crate ical;

use std::io::BufReader;
use std::fs::File;

#[test]
fn test_raw_line() {
    let buf = BufReader::new(File::open("./tests/ressources/multiple_root_components.ics")
        .unwrap());


    let reader = ical::IcalReader::new(buf);

    let mut i = 0;

    for res in reader {
        match res {
            Ok(line_parsed) => println!("{}", line_parsed),
            Err(err) => println!("{}", err),
        };

        i += 1;

        if i > 10 {
            break;
        }
    }

    // assert!(false, "end");
}
