// Michael Sheely <msheely@hmc.edu>
// Definition of the internal representation for the scheduling 

// Put this somewhere to allow hashing and thus insertion into maps
// #[derive(PartialEq, Eq, Hash)]



pub enum Day {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday
}

struct Time {
    hour : i32,
    minute : i32,
}

struct Displacement {
    day : Day,
    start : Time,
    end : Time,
}


