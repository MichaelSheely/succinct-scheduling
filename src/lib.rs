
#![allow(unused_variables)]
#[macro_use]
extern crate nom;
mod ir;

#[cfg(test)]
mod tests {
    //use super::*;
    #[test]
    fn time_conversions() {
        use super::ir;
        let t = ir::Time{hour: 3, meridiem: ir::Meridiem::pm}.to24hr();
        /*
        println!("{}", ir::Time{hour: 1, meridiem: Meridiem::pm}.to24hr());
        println!("{}", ir::Time{hour: 1, meridiem: Meridiem::am}.to24hr());
        println!("{}", ir::Time{hour: 12, meridiem: Meridiem::am}.to24hr());
        println!("{}", ir::Time{hour: 12, meridiem: Meridiem::pm}.to24hr());
        println!("{}", ir::Time{hour: 9, meridiem: Meridiem::pm}.to24hr());
        */
        assert_eq!(1, 1);
    }
}
