// The module for the times entered by the user

mod ir;
mod parser;

pub use self::ir::{Day, Time, Meridiem, Displacement};
pub use self::parser::time as parse;

