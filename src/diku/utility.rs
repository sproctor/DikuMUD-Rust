use std::io::prelude::*;
use std::io::{BufReader, Seek, SeekFrom};
use std::io::ErrorKind;
use std::str::FromStr;
use std::time::Duration;

use rand::distributions::{IndependentSample, Range};
use rand;
use time;

use diku::types;
use diku::constants;

pub fn dice(number: u32, size: u32) -> u32 {
    assert!(size >= 1);
    let mut sum = 0;
    let range = Range::new(0, size);
    let mut rng = rand::thread_rng();
    for _ in 1..number {
        sum += range.ind_sample(&mut rng);
    }
    sum
}

pub fn log(s: &str) {
    let ct = time::now();
    let timestr = ct.asctime();
    eprintln!("{} :: {}", timestr, s);
}

pub fn mud_time_passed(duration: Duration) -> types::TimeInfoData {
    let mut secs = duration.as_secs();

    let hours = (secs / constants::SECS_PER_MUD_HOUR) % 24;
    secs -= constants::SECS_PER_MUD_HOUR * hours;
    // These subtractions seem useless -Sean

    let day = (secs / constants::SECS_PER_MUD_DAY) % 35;
    secs -= constants::SECS_PER_MUD_DAY * day;

    let month = (secs / constants::SECS_PER_MUD_MONTH) % 17;
    secs -= constants::SECS_PER_MUD_MONTH * month;

    let year = secs / constants::SECS_PER_MUD_YEAR;

    types::TimeInfoData {
        hours: hours as u8,
        day: day as u8,
        month: month as u8,
        year: year as u16,
    }
}

pub fn read_char<R: Read>(reader: &mut BufReader<R>) -> u8 {
    let mut b = [0; 1];

    reader.read_exact(&mut b).unwrap();
    while b[0] == b' ' || b[0] == b'\t' || b[0] == b'\r' || b[0] == b'\n' {
        reader.read_exact(&mut b).unwrap();
    }
    b[0]
}

pub fn read_number<R: Read + Seek, F: FromStr>(reader: &mut BufReader<R>, clear: bool) -> Result<F, <F as FromStr>::Err> {
    let mut b = [0; 1];
    let mut buf = Vec::new();

    // trim preceding whitespace
    reader.read_exact(&mut b).unwrap();
    while b[0] == b' ' || b[0] == b'\t' || b[0] == b'\r' || b[0] == b'\n' {
        reader.read_exact(&mut b).unwrap();
    }

    // carry over byte from previous read
    while (b[0] >= b'0' && b[0] <= b'9') || b[0] == b'-' {
        buf.push(b[0]);
        match reader.read(&mut b) {
            Ok(0) => break,
            Ok(_) => (),
            Err(e) => if e.kind() == ErrorKind::Interrupted { continue } else { panic!("{}", e) },
        }
    }

    // trim trailing whitespace
    while clear && (b[0] == b' ' || b[0] == b'\t' || b[0] == b'\r' || b[0] == b'\n') {
        match reader.read(&mut b) {
            Ok(0) => break,
            Ok(_) => continue,
            Err(e) => if e.kind() == ErrorKind::Interrupted { continue } else { panic!("{}", e) },
        }
    }
    
    reader.seek(SeekFrom::Current(-1)).unwrap();

    String::from_utf8(buf).unwrap().parse()
}

pub fn fread_string<R: Read>(reader: &mut BufReader<R>) -> String {
    let mut string = String::new();
    loop {
        reader.read_line(&mut string).expect("fread_string");
        match string.find('~') {
            None => (),
            Some(offset) => {
                string.drain(offset..);
                break;
            },
        }
    }
    string
}