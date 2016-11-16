// Michael Sheely <msheely@hmc.edu>
// The Read Eval Print Loop for testing the parser

#[macro_use]
extern crate nom;

mod ir;

use nom::IResult;
use std::io;

fn main() {
    let mut line = String::new();
//    let date_re = Regex::new(r"(\d{4})-(\d{2})-(\d{2})").unwrap();
    while io::stdin().read_line(&mut line).map(|l| l > 0).unwrap_or(false) {
        match ir::parse(line.as_str().trim().as_bytes()) {
            IResult::Done(rest, ref res) if rest.len() == 0 =>
                println!("Parsed: {:#?}", res),
            IResult::Done(_, ref res) =>
                println!("Remaining inputp after parsing {:#?}", res),
            IResult::Error(x) => println!("Error: {:?}", x),
            IResult::Incomplete(x) => println!("Incomplete: {:?}", x),
        }
        line.clear();
    }
}
