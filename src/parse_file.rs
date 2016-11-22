// Michael Sheely <msheely@hmc.edu>
// The Read Eval Print Loop for testing the parser

#[macro_use]
extern crate nom;

mod ir;

use nom::IResult;
use ir::Entry;
//use ir::Day;
use std::io::Read;
use std::fs::File;
use std::collections::btree_map::BTreeMap;

// TODO make into an object with this as new() method
fn empty_schedule() -> BTreeMap<u8, Vec<(u8, u8)> > {
    let mut schedule = BTreeMap::new();
    //let days = vec![Day::Monday, Day::Tuesday];
    let days = vec![0, 1];
    for day in days {
        schedule.insert(day, Vec::new());
    }
    return schedule;
}

fn enter_times(schedule: &mut BTreeMap<u8, Vec<(u8, u8)> >, entries: &Vec<Entry>) {
    for entry in entries {
        let days = &entry.days;
        let displacements = &entry.displacements;
        for day in days {
            for disp in displacements {
                println!("Adding {} to {:?}.", disp, day);
                // TODO acutally add this displacement to appropriate vector
            }
        }
    }
}

fn main() {
    let mut schedule = empty_schedule();
    let mut line = String::new();
    let mut f = File::open("schedule.txt").unwrap();
    f.read_to_string(&mut line).unwrap();
    println!("Read data from provided file.");
    {
        match ir::parse(line.as_str().trim().as_bytes()) {
            IResult::Done(rest, ref res) if rest.len() == 0 => {
                println!("Parsed Schedule.");
                enter_times(&mut schedule, res);
            },
            IResult::Done(_, ref res) =>
                println!("Remaining input after parsing {:#?}", res),
            IResult::Error(x) => println!("Error: {:?}", x),
            IResult::Incomplete(x) => println!("Incomplete: {:?}", x),
        }
        line.clear();
    }
}

