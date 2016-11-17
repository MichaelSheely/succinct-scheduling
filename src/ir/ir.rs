// Michael Sheely <msheely@hmc.edu>
// Definition of the internal representation for the scheduling 

// Put this somewhere to allow hashing and thus insertion into maps
// #[derive(PartialEq, Eq, Hash)]

use std::fmt;

#[derive(Debug, PartialEq, PartialOrd)]
pub enum Day {
    Monday, Tuesday, //Wednesday, Thursday,
    //Friday, Saturday, Sunday
}

#[derive(Debug)]
pub struct Time {
    pub hour : u8,
    pub meridiem : Meridiem
    //minute : i32,
}
impl fmt::Display for Time {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}:00{}", self.hour, self.meridiem)
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug)]
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

#[derive(Debug)]
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

