extern crate ical;

use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;



#[test]
fn test_ical_line_reader() {
    let input = BufReader::new(File::open("./tests/ressources/ical_input.ics")
                             .unwrap());

    let mut valids = BufReader::new(File::open("./tests/ressources/ical_line_reader.res")
                             .unwrap()).lines();


    let reader = ical::LineReader::new(input);

    for line in reader {
        let output = format!("{:?}", line);

        assert_eq!(output, valids.next().unwrap().unwrap());
    }
}

#[test]
fn test_ical_line_parser() {
    let input = BufReader::new(File::open("./tests/ressources/ical_input.ics")
                             .unwrap());

    let mut valids = BufReader::new(File::open("./tests/ressources/ical_line_parser.res")
                             .unwrap()).lines();


    let reader = ical::LineParser::from_reader(input);

    for line in reader {
        let output = format!("{:?}", line);

        assert_eq!(output, valids.next().unwrap().unwrap());
    }
}
