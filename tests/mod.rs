#[cfg(feature = "property")]
pub mod property {
    extern crate ical;

    use std::fs::File;
    use std::io::BufRead;
    use std::io::BufReader;

    #[test]
    fn ical() {
        let input = BufReader::new(File::open("./tests/ressources/ical_input.ics").unwrap());

        let mut valids =
            BufReader::new(File::open("./tests/ressources/ical_property.res").unwrap()).lines();

        let reader = ical::PropertyParser::from_reader(input);

        for res in reader {
            let calendar = match res {
                Ok(res) => res,
                Err(err) => panic!("Throw errror: {}", err),
            };

            let output = format!("{:?}", calendar);

            assert_eq!(output, valids.next().unwrap().unwrap());
        }
    }

    #[test]
    fn vcard() {
        let input = BufReader::new(File::open("./tests/ressources/vcard_input.vcf").unwrap());

        let mut valids =
            BufReader::new(File::open("./tests/ressources/vcard_property.res").unwrap()).lines();

        let reader = ical::PropertyParser::from_reader(input);

        for res in reader {
            let contact = match res {
                Ok(res) => res,
                Err(err) => panic!("Throw errror: {}", err),
            };

            let output = format!("{:?}", contact);

            assert_eq!(output, valids.next().unwrap().unwrap());
        }
    }

    #[test]
    fn errors() {
        let input = BufReader::new(File::open("./tests/ressources/property_error.vcf").unwrap());

        let mut valids =
            BufReader::new(File::open("./tests/ressources/property_error.res").unwrap()).lines();

        let reader = ical::PropertyParser::from_reader(input);

        for res in reader {
            let error = match res {
                Ok(res) => panic!("Should return an error: {:?}", res),
                Err(err) => err,
            };

            let output = format!("{}", error);

            assert_eq!(output, valids.next().unwrap().unwrap());
        }
    }
}

#[cfg(feature = "line")]
pub mod line {
    extern crate ical;

    use std::fs::File;
    use std::io::BufRead;
    use std::io::BufReader;

    #[test]
    fn ical() {
        let input = BufReader::new(File::open("./tests/ressources/ical_input.ics").unwrap());

        let mut valids =
            BufReader::new(File::open("./tests/ressources/ical_line.res").unwrap()).lines();

        let reader = ical::LineReader::new(input);

        for line in reader {
            let output = format!("{:?}", line);

            assert_eq!(output, valids.next().unwrap().unwrap());
        }
    }

    #[test]
    fn vcard() {
        let input = BufReader::new(File::open("./tests/ressources/vcard_input.vcf").unwrap());

        let mut valids =
            BufReader::new(File::open("./tests/ressources/vcard_line.res").unwrap()).lines();

        let reader = ical::LineReader::new(input);

        for line in reader {
            let output = format!("{:?}", line);

            assert_eq!(output, valids.next().unwrap().unwrap());
        }
    }
}

#[cfg(any(feature = "ical", feature = "vcard"))]
pub mod parser {
    extern crate ical;

    use std::fs::File;
    use std::io::BufRead;
    use std::io::BufReader;

    #[test]
    fn ical() {
        let input = BufReader::new(File::open("./tests/ressources/ical_input.ics").unwrap());

        let mut valids =
            BufReader::new(File::open("./tests/ressources/ical_parser.res").unwrap()).lines();

        let reader = ical::IcalParser::new(input);

        for res in reader {
            let calendar = match res {
                Ok(res) => res,
                Err(err) => panic!("Throw errror: {}", err),
            };

            let output = format!("{:?}", calendar);

            assert_eq!(output, valids.next().unwrap().unwrap());
        }
    }

    #[test]
    fn vcard() {
        let input = BufReader::new(File::open("./tests/ressources/vcard_input.vcf").unwrap());

        let mut valids =
            BufReader::new(File::open("./tests/ressources/vcard_parser.res").unwrap()).lines();

        let reader = ical::VcardParser::new(input);

        for res in reader {
            let contact = match res {
                Ok(res) => res,
                Err(err) => panic!("Throw errror: {}", err),
            };

            let output = format!("{:?}", contact);

            assert_eq!(output, valids.next().unwrap().unwrap());
        }
    }
}
