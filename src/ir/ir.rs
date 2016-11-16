// Michael Sheely <msheely@hmc.edu>
// Definition of the internal representation for the scheduling 

// Put this somewhere to allow hashing and thus insertion into maps
// #[derive(PartialEq, Eq, Hash)]

#[derive(Debug)]
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

#[allow(non_camel_case_types)]
#[derive(Debug)]
pub enum Meridiem {
    am,
    pm
}

#[derive(Debug)]
pub struct Displacement {
    pub start : Time,
    pub end : Time,
    pub badness : u8
}

#[derive(Debug)]
pub struct Entry {
    pub days : Vec<Day>,
    pub displacements : Vec<Displacement>
}

