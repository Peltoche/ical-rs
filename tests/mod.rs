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
                Err(err) => panic!("Throw error: {}", err),
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
                Err(err) => panic!("Throw error: {}", err),
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
                Err(err) => panic!("Throw error: {}", err),
            };

            let output = format!("{:?}", calendar);

            assert_eq!(output, valids.next().unwrap().unwrap());
        }
    }

    #[test]
    fn ical_example_1() {
        let input = BufReader::new(File::open("./tests/ressources/ical_example_1.ics").unwrap());

        let valids = std::fs::read_to_string("./tests/ressources/ical_example_1.res").unwrap().replace('\n', "");

        let reader = ical::IcalParser::new(input);

        for res in reader {
            let calendar = match res {
                Ok(res) => res,
                Err(err) => panic!("{}", err),
            };

            let output = format!("{:?}", calendar);

            assert_eq!(output, valids);
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
                Err(err) => panic!("Throw error: {}", err),
            };

            let output = format!("{:?}", contact);

            assert_eq!(output, valids.next().unwrap().unwrap());
        }
    }

    #[test]
    fn vcard_lowercase() {
        let input = BufReader::new(File::open("./tests/ressources/vcard_lowercase.vcf").unwrap());

        let mut valids =
            BufReader::new(File::open("./tests/ressources/vcard_lowercase.res").unwrap()).lines();

        let reader = ical::VcardParser::new(input);

        for res in reader {
            let contact = match res {
                Ok(res) => res,
                Err(err) => panic!("Throw error: {:?}", err),
            };

            let output = format!("{:?}", contact);

            assert_eq!(output, valids.next().unwrap().unwrap());
        }
    }
}

#[cfg(all(feature = "ical", feature = "generator"))]
pub mod generator {
    extern crate ical;
    use self::ical::generator::Emitter;
    use std::fs::File;
    use std::io::BufRead;
    use std::io::BufReader;

    #[test]
    fn generate_o365_test() {
        let filename = "./tests/ressources/o365_meeting.ics";

        let original = BufReader::new(File::open(filename).unwrap())
            .lines()
            .map(|line| line.unwrap() + "\n")
            .collect::<String>();

        let input = BufReader::new(File::open(filename).unwrap());
        let mut reader = ical::IcalParser::new(input);
        let generated = reader.next().unwrap().ok().unwrap().generate();

        assert_eq!(&generated, &original);
    }

    #[test]
    fn generate_sabre_test() {
        let filename = "./tests/ressources/sabre_test.ics";

        let original = BufReader::new(File::open(filename).unwrap())
            .lines()
            .map(|line| line.unwrap() + "\n")
            .collect::<String>();

        let input = BufReader::new(File::open(filename).unwrap());
        let mut reader = ical::IcalParser::new(input);
        let generated = reader.next().unwrap().ok().unwrap().generate();

        assert_eq!(&generated, &original);
    }
}
