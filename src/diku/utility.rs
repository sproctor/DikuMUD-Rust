use std::io::prelude::*;
use std::io::{BufReader, Seek, SeekFrom};
use std::io::ErrorKind;
use std::str::FromStr;
use std::time::Duration;

use rand::distributions::{IndependentSample, Range};
use rand;
use rand::Rng;
use time;

use diku::types::*;
use diku::constants::*;

pub fn number(from: i32, to: i32) -> i32 {
    rand::thread_rng().gen_range(from, to)
}

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

pub fn mud_time_passed(duration: Duration) -> TimeInfoData {
    let mut secs = duration.as_secs();

    let hours = (secs / SECS_PER_MUD_HOUR) % 24;
    secs -= SECS_PER_MUD_HOUR * hours;
    // These subtractions seem useless -Sean

    let day = (secs / SECS_PER_MUD_DAY) % 35;
    secs -= SECS_PER_MUD_DAY * day;

    let month = (secs / SECS_PER_MUD_MONTH) % 17;
    secs -= SECS_PER_MUD_MONTH * month;

    let year = secs / SECS_PER_MUD_YEAR;

    TimeInfoData {
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

pub fn read_line_trim<R: Read>(reader: &mut BufReader<R>) -> String {
    let mut line = String::new();
    reader.read_line(&mut line).unwrap();
    String::from(line.trim())
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

// can subject see character "obj"?
pub fn can_see(_sub: &CharData, obj: &CharData) -> bool {
    ((is_affected(obj, AffectedFlags::AFF_INVISIBLE)))
}

fn is_affected(ch: &CharData, skill: AffectedFlags) -> bool {
    ch.specials.affected_by.contains(skill)
}

pub fn is_npc(ch: &CharData) -> bool {
    ch.specials.act.contains(SpecialActFlags::ACT_ISNPC)
}

pub fn get_pos(ch: &CharData) -> Position {
    ch.specials.position
}

pub fn awake(ch: &CharData) -> bool {
    get_pos(ch) > Position::Sleeping
}

pub fn get_alignment(ch: &CharData) -> i32 {
    ch.specials.alignment
}

pub fn is_good(ch: &CharData) -> bool {
    get_alignment(ch) >= 350
}

pub fn is_evil(ch: &CharData) -> bool {
    get_alignment(ch) <= -350
}

pub fn is_neutral(ch: &CharData) -> bool {
    !is_good(ch) && !is_evil(ch)
}