//#[macro_use] extern crate lazy_static;
extern crate regex;
use self::regex::Regex;

use ir::day_to_int;
use ir::{Day, Displacement, Entry, Meridiem, Time};
use ir::NUM_DAYS;

use nom::digit;

named!(mon <Day>, chain!(alt!(
            tag!("Monday") | tag!("monday") | tag!("mon") |
            tag!("Mon") | tag!("M")), || { Day::Monday }));
named!(tues <Day>, chain!(alt!(
            tag!("Tuesday") | tag!("tuesday") | tag!("Tues") |
            tag!("tues") | tag!("T")), || { Day::Tuesday }));
named!(wed <Day>, chain!(alt!(
            tag!("Wednesday") | tag!("wednesday") | tag!("Wed") |
            tag!("wed") | tag!("W")), || {Day::Wednesday}));

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
        one_of!(",-")? ~ whitespace ~
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

// Below this point we create parsers for calendar documents

fn adjust_timezone(h: u8) -> u8 {
    if h > 7 {
        h - 8
    } else {
        h + 16
    }
}

fn date_to_day(month: u8, date: u8) -> Day {
    if month != 12 {
        unimplemented!();
    }
    match date % 7 {
        0 => Day::Wednesday,
        //1 => Day::Thursday,
        //2 => Day::Friday,
        //3 => Day::Saturday,
        //4 => Day::Sunday,
        5 => Day::Monday,
        6 => Day::Tuesday,
        _ => panic!(),
    }
}

fn empty_displacements() -> Vec<Vec<Displacement>> {
    vec![Vec::<Displacement>::new(); NUM_DAYS as usize]
}

pub fn parse_ics(contents: &str) -> Vec<Vec<Displacement>> {
    let mut displacements = empty_displacements();
    let re = Regex::new(r"(?x)(\d{4})(\d{2})(\d{2})T(\d{2}).*\r\n
                        DTEND:(\d{4})(\d{2})(\d{2})T(\d{2}).*\r\n").unwrap();
    let mut iter = contents.split("DTSTART:");
    iter.next();  // skip first chunk, doesn't start with times
    for s in iter {
        for cap in re.captures_iter(s) {
            // For some reason, time zone is 8 hours ahead???
            let mut start_hour = cap.at(4).unwrap().parse::<u8>().unwrap();
            start_hour = adjust_timezone(start_hour);
            let mut end_hour = cap.at(8).unwrap().parse::<u8>().unwrap();
            end_hour = adjust_timezone(end_hour);
            //println!("{}/{}: {} - {}",
            //cap.at(2).unwrap_or(""), cap.at(3).unwrap_or(""),
            //start_hour, end_hour);
            let date = cap.at(3).unwrap().parse::<u8>().unwrap();
            displacements[day_to_int(&date_to_day(12, date)) as usize].push(
              Displacement{start: Time::from24hr(start_hour),
                           end: Time::from24hr(end_hour),
                           badness : 0} );
        }
    }
    displacements
}

