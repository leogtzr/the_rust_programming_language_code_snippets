extern crate regex;
use regex::Regex;

use std::io::{BufRead, BufReader};
use stringreader::StringReader;

use std::fs::File;

fn read_data<R: BufRead>(r: &mut R) {
    r.lines().map(|line| line.unwrap()).for_each(|line| {
        println!("{}", line);
    });
}

const LOCKED_RGX: &'static str = r#"\s*\- locked\s*<(.*)>\s*\(a\s(.*)\)"#;

fn main() {
    // let mut streader = StringReader::new("Line 1\nLine 2");
    // let mut br = BufReader::new(streader);

    // let f = File::open("file.txt").expect("error: file 'file.txt' not found.");
    // let mut br = BufReader::new(f);
    // read_data(&mut br);

    // for line in String::from("cmn\ncon\narroz").split("\n") {
    //     println!("{}", line);
    // }
    let s = "- locked <0x0000000682e5f948> (a sun.security.provider.Sun)";
    let re = Regex::new(LOCKED_RGX).unwrap();

    if re.is_match(s) {
        println!("Ok ... ");
    }
    
}
