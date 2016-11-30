
use ir::Day;
use ir::Displacement;
use ir::Entry;
use ir::Meridiem;
use ir::Time;

use nom::digit;

named!(mon <Day>, chain!(
        alt!(tag!("Monday") | tag!("monday")), || { Day::Monday }));
named!(tues <Day>, chain!(
        alt!(tag!("Tuesday") | tag!("tuesday")), || { Day::Tuesday }));
named!(wed <Day>, chain!(
        alt!(tag!("Wednesday") | tag!("wednesday")), || {Day::Wednesday}));

named!(pub day <Day>, alt!(
       chain!(d : alt!(mon | tues | wed ) 
              ~ opt!(char!(',')) ~ whitespace, || { d }))
);

named!(pub date <()>, chain!(digit ~ char!('/') ~
                             digit ~ char!('/') ~
                             digit, || {})
);

// Zero or more instances of whitespace
named!(whitespace<Vec<&[u8]> >, many0!(alt!(
            tag!(" ") | tag!("\n") | tag!("\t") )));

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

named!(pub range <Displacement>, chain!(whitespace ~
        t0: time ~ whitespace ~
        one_of!(",-") ~ whitespace ~
        t1: time ~ whitespace, || Displacement{start: t0, end:t1, badness:0})
);

named!(exception <()>, chain!(whitespace ~ tag!("except") ~ whitespace ~
                                d: date, || d)
);

named!(avail <Displacement>, chain!(tag!("free") ~ r: alt!(
            chain!(char!('(') ~ r: range ~ char!(')'), || r) |
            chain!(whitespace ~ r: range, || r))
        ~ opt!(complete!(exception)), || r)
);

/*
 * This is an issue because of an opt! at the end of a chain! see:
 * https://github.com/Geal/nom/issues/271 
 */
named!(pub entry <Entry>,
       chain!(days : many0!(day) ~ tag!(":") ~ whitespace ~
               displacements : many0!(avail) ~ whitespace,
               || Entry{days: days, displacements: displacements} )
);

named!(pub entries <Vec<Entry> >,
       many1!(entry)
);

