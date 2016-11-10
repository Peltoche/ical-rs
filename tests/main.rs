
extern crate vcard_ical;
extern crate rustc_serialize;

use std::path::Path;
use std::error::Error;

use rustc_serialize::json::ToJson;

use vcard_ical::parser::Parser;

#[test]
fn invalid_path() {
    let path = Path::new("this is an invalid path");

    match Parser::from_file(path) {
        Ok(_)        => assert!(false, "File should be invalid"),
        Err(err)     => assert_eq!(err.description(), "Invalid file"),
    }
}

#[test]
fn valid_path() {
    let path = Path::new("./example-vcards/vcard1.vcf");

    let reader = match Parser::from_file(path) {
        Ok(parser)  => parser,
        Err(err)     => return assert!(false, err),
    };

    for elem in reader {
        match elem {
            Ok(val)     => {
                println!("{}", val.to_json().pretty());
            }
            Err(err)    => {
                println!("err: {:?}", err);
            }
        };
    };

    assert!(false);
}
