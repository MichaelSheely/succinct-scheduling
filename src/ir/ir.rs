// Michael Sheely <msheely@hmc.edu>
// Definition of the internal representation for the scheduling

// Put this somewhere to allow hashing and thus insertion into maps
// #[derive(PartialEq, Eq, Hash)]

use std::fmt;

#[derive(Debug, PartialEq, PartialOrd)]
pub enum Day {
    Monday, Tuesday, Wednesday//, Thursday,
    //Friday, Saturday, Sunday
}

#[derive(Debug)]
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
#[derive(Debug, PartialEq)]
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

