// Michael Sheely <msheely@hmc.edu>
// The Read Eval Print Loop for testing the parser

#[macro_use]
extern crate nom;

mod ir;

use nom::IResult;
use ir::Entry;
use ir::Day;
use ir::Time; // delete
use ir::Meridiem; // delete
use std::io::Read;
use std::fs::File;

// TODO make into an object with this as new() method
fn empty_schedule() -> Vec<bool> {
    let num_slots = 7 * 24; //* 60; // 7 days with 24 hours with 60 mins
    let schedule = vec![false; num_slots];
    /*
    //let days = vec![Day::Monday, Day::Tuesday, Day::Wednesday];
    let days = vec![0, 1, 2];
    for day in days {
        schedule.insert(day, Vec::new());
    }
    */
    schedule
}

fn day_int(d: &Day) -> u8 {
    match *d {
        Day::Monday    => 0,
        Day::Tuesday   => 1,
        Day::Wednesday => 2
    }
}

fn enter_times(schedule: &mut Vec<bool>, entries: &Vec<Entry>) {
    println!("{}", ir::Time{hour: 3, meridiem: Meridiem::pm}.to24hr());
    for entry in entries {
        let days = &entry.days;
        let displacements = &entry.displacements;
        for day in days {
            for disp in displacements {
                println!("Adding {} to {:?}.", disp, day);
                let day_int = day_int(day) as usize;
                let mut start_int = day_int * 24 + disp.start.to24hr() as usize;
                let end_int = day_int * 24 + disp.end.to24hr() as usize;
                println!("From {} to {}.", start_int, end_int);
                //TODO assert start_int < end_int
                while start_int < end_int {
                    schedule[start_int] = true;
                    start_int += 1;
                }
                // TODO acutally add this
                // displacement to appropriate vector
            }
        }
    }
    for (i, freedom) in schedule.iter().enumerate() {
        //println!("Slot {} is {}", i, freedom);
    }
}

fn main() {
    let mut schedule = empty_schedule();
    let mut line = String::new();
    let mut f = File::open("schedule2.txt").unwrap();
    f.read_to_string(&mut line).unwrap();
    println!("Read data from provided file.");
    {
        match ir::parse(line.as_str().trim().as_bytes()) {
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
}

