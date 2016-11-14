// Michael Sheely <msheely@hmc.edu>
// Definition of the internal representation for the scheduling 

// Put this somewhere to allow hashing and thus insertion into maps
// #[derive(PartialEq, Eq, Hash)]

//pub mod ir {

#[derive(Debug)]
pub enum Day {
    Monday, Tuesday, Wednesday, Thursday,
    Friday, Saturday, Sunday
}

#[derive(Debug)]
pub struct Time {
    hour : i32,
    meridiem : Meridiem
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
    day : Day,
    start : Time,
    end : Time,
}

//} // pub mod IR

