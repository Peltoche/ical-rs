
extern crate vcard_ical;
extern crate rustc_serialize;

use std::path::Path;
use std::error::Error;
use std::fs::File;
use std::io::{BufReader, BufRead};

use rustc_serialize::json::ToJson;

use vcard_ical::parser::Parser;


fn run_file(in_path_vcf: &Path, res_path_json: &Path) {
    let res_file = File::open(res_path_json).unwrap();
    let mut res_reader = BufReader::new(res_file).lines();

    let parser = Parser::from_file(in_path_vcf).unwrap();

    for elem in parser {
        let expected = res_reader.next().unwrap().unwrap();
        let output = elem.unwrap().to_json();

        assert_eq!(expected, format!("{}", output));
    };
}

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
    let in_path_vcf = Path::new("./tests/parser/vcard1.vcf");
    let out_path_json = Path::new("./tests/parser/vcard1.json");

    run_file(in_path_vcf, out_path_json);
}
