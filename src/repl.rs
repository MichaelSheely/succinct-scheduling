// Michael Sheely <msheely@hmc.edu>
// The Read Eval Print Loop for testing the parser

#[macro_use]
extern crate nom;

mod ir;

use std::io;
use nom::IResult;

fn main() {
    let mut line = String::new();
    while io::stdin().read_line(&mut line).map(|l| l > 0).unwrap_or(false) {
        match ir::parse_txt(line.as_str().trim().as_bytes()) {
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

