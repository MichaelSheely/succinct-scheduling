// Michael Sheely <msheely@hmc.edu>
// The Read Eval Print Loop for testing the parser

#[macro_use]
extern crate nom;

mod ir;

use nom::IResult;
use ir::{Entry, Day, Time};
use ir::{day_to_int, day_from_int};
use ir::{NUM_DAYS, NUM_HOURS, NUM_MINUTES};
use std::fs::File;
use std::io;
use std::io::Read;
use std::io::stdin;
use std::env;

// TODO make into an object with this as new() method
fn empty_schedule() -> Vec<bool> {
    let num_slots = (NUM_DAYS * NUM_HOURS * NUM_MINUTES) as usize;
    let schedule = vec![false; num_slots];
    schedule
}

fn enter_times(schedule: &mut Vec<bool>, entries: &Vec<Entry>) {
    for entry in entries {
        let days = &entry.days;
        let displacements = &entry.displacements;
        for day in days {
            for disp in displacements {
                // TODO: Support minutes as well
                println!("Free {} on {:?}.", disp, day);
                let day_int = day_to_int(day) as usize;
                let mut start_int = day_int * 24 + disp.start.to24hr() as usize;
                let end_int = day_int * 24 + disp.end.to24hr() as usize;
                while start_int < end_int {
                    schedule[start_int] = true;
                    start_int += 1;
                }
            }
        }
    }
}

fn get_filename() -> String {
    println!("Enter the name of the file that contains your schedule.");
    let mut line = String::new();
    io::stdin().read_line(&mut line).map(|l| l > 0).unwrap_or(false);
    String::from(line.as_str().trim())  // Remove newline
}

fn main() {
    let mut schedule = empty_schedule();
    let mut line = String::new();
    let mut fname : String;
    match env::args().nth(1) {
        Some(name) => fname = name,
        _ => fname = get_filename(),
    }
    println!("Got filename <{}>", fname);

    let mut f = File::open(&fname).unwrap();
    f.read_to_string(&mut line).unwrap();
    println!("Read data from {}.", fname);
    match ir::parse_txt(line.as_str().trim().as_bytes()) {
        IResult::Done(rest, ref res) if rest.len() == 0 => {
            println!("Parsed Schedule.");
            enter_times(&mut schedule, res);
        },
        IResult::Done(_, ref res) =>
            println!("Remaining input after parsing\n{:#?}", res),
        IResult::Error(x) => println!("Error: {:?}", x),
        IResult::Incomplete(x) => println!("Incomplete: {:?}", x),
    }
}

