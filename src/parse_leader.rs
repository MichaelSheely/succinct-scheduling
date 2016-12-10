// Michael Sheely <msheely@hmc.edu>
// The Read Eval Print Loop for testing the parser

#[macro_use]
//#[macro_use] extern crate lazy_static;
extern crate nom;

mod ir;

use nom::IResult;
use ir::{Entry, Time, Displacement};
use ir::{day_to_int, day_from_int};
use ir::{NUM_DAYS, NUM_HOURS, NUM_MINUTES};

use std::path::Path;
use std::env;
use std::io;
use std::fs::read_dir;
use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::io::stdout;

// TODO make into an object with this as new() method
fn empty_schedule() -> Vec<u8> {
    let num_slots = (NUM_DAYS * NUM_HOURS * NUM_MINUTES) as usize;
    let schedule = vec![0; num_slots];
    schedule
}

fn time_within(hour: u8, d: Displacement) -> bool {
    hour < d.end.to24hr() && hour > d.start.to24hr()
}

fn enter_times(schedule: &mut Vec<u8>, entries: &Vec<Entry>,
                            valid_times: ir::Displacement) {
    let verbosity = false;
    let mut avalibility = vec![false; schedule.len()];
    for entry in entries {
        let days = &entry.days;
        let displacements = &entry.displacements;
        // TODO: Support minutes as well
        for day in days {
            for disp in displacements {
                // TODO Change to use a logger
                if verbosity {
                    println!("Adding {} to {:?}.", disp, day);
                }
                let day_int = day_to_int(day) as usize;
                let mut start_int = day_int * 24 + disp.start.to24hr() as usize;
                let end_int = day_int * 24 + disp.end.to24hr() as usize;
                //TODO assert start_int < end_int
                while start_int < end_int {
                    if time_within((start_int % 24) as u8, valid_times) {
                        //schedule[start_int] += 1;
                        avalibility[start_int] = true;
                    }
                    start_int += 1;
                }
            }
        }
    }
    // We run this as a separate loop so that we do not increment
    // the schedule twice for the same user (for example if they said
    // they were free every Weekday from 1 to 2 and they also specified
    // that they were free on Tuesdays from 1 to 4).
    for (i, avail) in avalibility.iter().enumerate() {
        if *avail {
            schedule[i] += 1;
        }
    }
}

fn display_times(schedule: &Vec<u8>, threshold: u8) {
    let mut times = Vec::new();
    for (i, num) in schedule.iter().enumerate() {
        if *num >= threshold {
            times.push((num, i));
        }
    }
    times.sort_by(|a, b| {
                  let (a1, a2) = *a;
                  let (b1, ref b2) = *b;
                  if a1 == b1 { a2.cmp(b2) } else { b1.cmp(a1) } });
    for (num, i) in times {
        let day = day_from_int((i / 24) as u8);
        let hour = Time::from24hr((i % 24) as u8);
        println!("{:?} at {} has {} people free!", day, hour, num);
    }
}

fn get_possible_times() -> ir::Displacement {
    print!("Please enter the range of times (for example \"9am - 5pm\") \
             over which we will try to find overlapping avalability.\n>> ");
    stdout().flush().unwrap_or(());
    let mut line = String::new();
    loop {
        io::stdin().read_line(&mut line).map(|l| l > 0).unwrap_or(false);
        match ir::parse_time_range(line.as_str().trim().as_bytes()) {
            IResult::Done(rest, ref res) if rest.len() == 0 => {
                return *res;
            },
            _ => {
                print!("Could not parse a time range,\
                          please try again.\n>> ");
                stdout().flush().unwrap_or(());
            },
        }
        line.clear();
    };
}

fn get_threshold() -> u8 {
    print!("How many individuals must be present for you \
            to want to see that timeslot?\n>> ");
    stdout().flush().unwrap_or(());
    let mut line = String::new();
    loop {
        io::stdin().read_line(&mut line).map(|l| l > 0).unwrap_or(false);
        match line.as_str().trim().parse::<u8>() {
            Ok(i) => {
                return i;
            },
            _ => {
                print!("That is not a valid integer. Please try again.\n>> ");
                stdout().flush().unwrap_or(());
            },
        }
        line.clear();
    };
}

fn get_files() -> Vec<String> {
    let paths = read_dir(&Path::new(&env::current_dir().unwrap())).unwrap();
    paths.filter_map(|entry| { entry.ok()
        .and_then(|e| e.path().file_name().and_then(|n|
        n.to_str().map(|s| String::from(s))))}).collect::<Vec<String>>()
}

fn update_schedule_from_txt(schedule: &mut Vec<u8>, filename: String,
                            valid_times: ir::Displacement) {
    let mut contents = String::new();
    let mut f = File::open(&filename).unwrap();
    f.read_to_string(&mut contents).unwrap();
    match ir::parse_txt(contents.as_str().trim().as_bytes()) {
        IResult::Done(rest, ref res) if rest.len() == 0 =>
          enter_times(schedule, res, valid_times),
        _ => println!("Trouble parsing {}!", filename),
    }
}

fn update_schedule_from_ics(schedule: &mut Vec<u8>, filename: String,
                            valid_times: ir::Displacement) {
    let mut contents : String = String::new();
    let mut f = File::open(&filename).unwrap();
    f.read_to_string(&mut contents).unwrap();
    let displacements = ir::parse_ics(contents.as_str().trim());
    for i in 0..schedule.len() {
        if time_within((i % 24) as u8, valid_times) {
            schedule[i] += 1;
        }
    }
    for day_int in 0..NUM_DAYS as usize {
        for disp in &displacements[day_int] {
            let mut start_int = day_int * 24 + disp.start.to24hr() as usize;
            let end_int = day_int * 24 + disp.end.to24hr() as usize;
            //TODO assert start_int < end_int
            while start_int < end_int {
                if time_within((start_int % 24) as u8, valid_times) {
                    schedule[start_int] -= 1;
                }
                start_int += 1;
            }
        }
    }
}

fn main() {
    let possible_times = get_possible_times();
    let threshold = get_threshold();
    let files = get_files();
    let mut schedule = empty_schedule();
    for fname in files {
        if fname.ends_with(".txt") {
            println!("Reading {}", fname);
            update_schedule_from_txt(&mut schedule, fname, possible_times);
        }
        else if fname.ends_with(".ics") {
            println!("Reading {}", fname);
            update_schedule_from_ics(&mut schedule, fname, possible_times);
        }
    }
    display_times(&schedule, threshold);
}

