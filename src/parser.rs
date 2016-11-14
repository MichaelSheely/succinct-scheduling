#[macro_use]
extern crate nom;

pub mod ir;

macro_rules! check(
  ($input:expr, $submac:ident!( $($args:tt)* )) => (
    {
      let mut failed = false;
      for &idx in $input {
        if !$submac!(idx, $($args)*) {
            failed = true;
            break;
        }
      }
      if failed {
        nom::IResult::Error(nom::Err::Position(nom::ErrorKind::Custom(20),$input))
      } else {
        nom::IResult::Done(&b""[..], $input)
      }
    }
  );
  ($input:expr, $f:expr) => ( check!($input, call!($f)););
);

fn buf_to_u32(s: &[u8]) -> u32 {
    to_u32(to_string(s))
}

fn ret_meridiem(c:&u8) -> IResult<&[u8], Meridiem> {
            if c == 'a' {am} else {pm} }


//named!(pub take_one_digit, flat_map!(take!(1), check!(is_digit)));
named!(pub take_two_digits, flat_map!(take!(2), check!(is_digit)));

named!(pub hour <&[u8], u32>, map!(call!(take_2_digits), buf_to_u32));
named!(pub meridiem <&[u8], Meridiem>,
   chain!(
        alt!(
            char!('a')
            char!('p')
            ) ~
        meridiem: ret_meridiem ~  // result of above parse, used in closure
        alt!(
            tag!(".m.")
            char!('m')
        ),
        || { meridiem }
        )
   );

named!(pub time <&[u8], Time>, chain!(
        h: hour ~
        m: meridiem ~
        || {Time{ hour: h, meridiem: m }}
        ));

