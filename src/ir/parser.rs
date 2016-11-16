
use ir::Day;
use ir::Displacement;
use ir::Entry;
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
       chain!(d : alt!(mon | tues) ~ opt!(char!(',')) ~ whitespace, || { d }))
);

// Zero or more instances of whitespace
named!(whitespace<Vec<&[u8]> >, many0!(tag!(" ")));

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

named!(displacement <Displacement>, chain!(
        tag!("free(") ~
        t0 : time ~ char!('-') ~
        t1 : time ~ char!(')'), || Displacement{start: t0, end: t1, badness: 0})
);

named!(pub entry <Entry>,
        chain!(days : many1!(day) ~ tag!(":") ~ whitespace ~
               displacements : many1!(displacement),
               || Entry{days: days, displacements: displacements} )
);

