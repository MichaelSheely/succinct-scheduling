
#![allow(unused_variables)]
#[macro_use]
extern crate nom;
mod ir;

#[cfg(test)]
mod tests {
    #[test]
    fn time_conversions() {
        use super::ir;
        use super::ir::Meridiem;

        assert_eq!(1,  ir::Time{hour:  1, meridiem: Meridiem::am}.to24hr());
        assert_eq!(15, ir::Time{hour:  3, meridiem: Meridiem::pm}.to24hr());
        assert_eq!(13, ir::Time{hour:  1, meridiem: Meridiem::pm}.to24hr());
        assert_eq!(21, ir::Time{hour:  9, meridiem: Meridiem::pm}.to24hr());
        assert_eq!(12, ir::Time{hour: 12, meridiem: Meridiem::pm}.to24hr());
    }
}
