// Michael Sheely <msheely@hmc.edu>
// Definition of the internal representation for the scheduling

// Put this somewhere to allow hashing and thus insertion into maps
// #[derive(PartialEq, Eq, Hash)]

use std::fmt;

pub static NUM_DAYS: u8 = 7;
pub static NUM_HOURS: u8 = 24;
pub static NUM_MINUTES: u8 = 1;

#[derive(Debug, PartialEq, PartialOrd)]
pub enum Day {
    Monday, Tuesday, Wednesday, Thursday,
    Friday, Saturday, Sunday
}

pub fn day_to_int(d: &Day) -> u8 {
    match *d {
        Day::Monday    => 0,
        Day::Tuesday   => 1,
        Day::Wednesday => 2,
        Day::Thursday  => 3,
        Day::Friday    => 4,
        Day::Saturday  => 5,
        Day::Sunday    => 6,
    }
}
pub fn day_from_int(n: u8) -> Day {
    match n {
        0 => Day::Monday,
        1 => Day::Tuesday,
        2 => Day::Wednesday,
        3 => Day::Thursday,
        4 => Day::Friday,
        5 => Day::Saturday,
        6 => Day::Sunday,
        _ => panic!("{} is not a valid day!", n),
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Time {
    pub hour : u8,
    pub meridiem : Meridiem
    //minute : i32,
}
impl Time {
    pub fn to24hr(&self) -> u8 {
        let mut hr = self.hour;
        // TODO: Handle 12:00am more elegantly
        if self.meridiem == Meridiem::am && hr == 12 {
            0
        } else if hr == 12 {
            12
        } else {
            if self.meridiem == Meridiem::pm {
                hr += 12;
            }
            hr % 24
        }
    }
    pub fn from24hr(hr: u8) -> Self {
        if hr > 12 {
            Time{hour: hr - 12, meridiem: Meridiem::pm}
        } else {
            Time{hour: hr, meridiem: Meridiem::am}
        }
    }
}
impl fmt::Display for Time {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}:00{}", self.hour, self.meridiem)
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Meridiem {
    am,
    pm
}
impl fmt::Display for Meridiem {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match *self {
            Meridiem::am => "am",
            Meridiem::pm => "pm"
        })
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Displacement {
    pub start : Time,
    pub end : Time,
    pub badness : u8
}
impl fmt::Display for Displacement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} - {}", self.start, self.end)
    }
}

#[derive(Debug)]
pub struct Entry {
    pub days : Vec<Day>,
    pub displacements : Vec<Displacement>
}

