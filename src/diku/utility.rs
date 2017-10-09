use std::io::BufReader;
use std::io::prelude::*;
use std::time::Duration;

use rand::distributions::{IndependentSample, Range};
use rand;
use regex::Regex;
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

pub fn read_number<R: Read>(reader: &mut BufReader<R>) -> u32 {
    // TODO: make the following static
    let re: Regex = Regex::new(r"\s*#(\d+)").expect("read_number regex");

    let mut line = String::new();
    reader.read_line(&mut line).unwrap();
    let cap = re.captures(&mut line).unwrap();
    cap[1].parse::<u32>().unwrap()
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