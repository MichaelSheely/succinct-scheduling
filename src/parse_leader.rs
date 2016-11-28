// Michael Sheely <msheely@hmc.edu>
// The Read Eval Print Loop for testing the parser

#[macro_use]
extern crate nom;

mod ir;

use nom::IResult;
use ir::Displacement;
//use ir::Day;
use std::io::Read;
use std::fs::File;

fn main() {
    let mut line = String::new();
    let mut f = File::open("params.txt").unwrap();
    f.read_to_string(&mut line).unwrap();
    println!("Read data from provided file.");
    {
        match ir::leader_parse(line.as_str().trim().as_bytes()) {
            IResult::Done(rest, ref res) if rest.len() == 0 => {
                println!("Parsed Schedule.");
            },
            IResult::Done(_, ref res) =>
                println!("Remaining input after parsing {:#?}", res),
            IResult::Error(x) => println!("Error: {:?}", x),
            IResult::Incomplete(x) => println!("Incomplete: {:?}", x),
        }
        line.clear();
    }
}

