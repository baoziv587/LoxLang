// use std::fs;
use std::path;
#[derive(Debug)]
pub struct Lox {}

impl Lox {
    // add code here
    pub fn run_file(filename: &str) {
        if !path::Path::new(filename).exists() {
            eprintln!("Lox Program: path {} not exist!", filename);
            std::process::exit(1)
        }
    }
    fn run(source: &str) {}

    // pub fn new()->Result<Lox,&'static str>{
    //   Ok(Lox{})
    // }
}
