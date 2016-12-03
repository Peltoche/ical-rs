// extern crate ical;

// use std::io::BufReader;
// use std::fs::File;


// fn test_raw_line(buf: BufReader<File>) {
    // let reader = ical::IcalParser::new(buf);

    // for res in reader {
        // match res {
            // Ok(line_parsed) => println!("{:?}", line_parsed),
            // Err(err) => println!("{}", err),
        // };
    // }
// }


// #[test]
// fn test_mltiple_root_components() {
    // let buf = BufReader::new(File::open("./tests/ressources/multiple_root_components.ics")
                             // .unwrap());

    // test_raw_line(buf);
    // assert!(false, "END")
// }

// #[test]
// fn test_rfc() {
    // let buf = BufReader::new(File::open("./tests/ressources/rfc.ics").unwrap());

    // test_raw_line(buf);
    // assert!(false, "END")
// }

// #[test]
// fn test_component() {
    // let buf = BufReader::new(File::open("./tests/ressources/component.ics").unwrap());

    // test_raw_line(buf);
    // assert!(false, "END")
// }

// #[test]
// fn test_property_params() {
    // let buf = BufReader::new(File::open("./tests/ressources/property_params.ics").unwrap());

    // test_raw_line(buf);
    // assert!(false, "END")
// }
