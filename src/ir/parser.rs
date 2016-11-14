
use ir::Day;
use ir::Meridiem;
use ir::Time;

use nom::digit;
//use std::str;
//use std::str::FromStr;

named!(mon <Day>,
       chain!(alt!(tag!("Monday") | tag!("monday")), || { Day::Monday }));
named!(tues <Day>,
       chain!(alt!(tag!("Tuesday") | tag!("tuesday")), || { Day::Tuesday }));

named!(pub day <Day>, alt!(
       chain!(mon, || { Day::Monday }) |
       chain!(tues, || { Day::Tuesday })
   )
);

named!(meridiem <Meridiem>, alt!(
        chain!(char!('a') ~ alt!(tag!(".m.") | tag!("m")), || {Meridiem::am}) |
        chain!(char!('p') ~ alt!(tag!(".m.") | tag!("m")), || {Meridiem::pm})
    )
);

named!(hour <u8>,
       chain!(d1 : digit, || { String::from_utf8_lossy(d1).parse().unwrap() })
);

named!(pub time <Time>, chain!(h : hour ~ m : meridiem, 
       || Time{hour: h, meridiem: m})
);

// parse a Time
// parse a Displacement
// parse an entry


