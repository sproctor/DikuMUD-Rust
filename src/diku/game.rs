use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, Cursor};
use std::io::prelude::*;
use std::os::unix::io::RawFd;
use std::sync::atomic::{AtomicUsize, ATOMIC_USIZE_INIT};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use chan::Receiver;
use chan_signal;
use chan_signal::Signal;

use regex::Regex;

use diku::constants;
use diku::modify::build_help_index;
use diku::types::{DescriptorData, ExitFlags, EX_ISDOOR, EX_PICKPROOF, RoomDirectionData, RoomFlags, SectorType, Sky, Sunlight, WeatherData};
use diku::utility::{dice, log, mud_time_passed};

enum ResetMode {
    DoNot,  // Don't reset, and don't update age.
    NoPC,   // Reset if no PC's are located in zone.
    Do,     // Just reset.
}

struct ResetCom {
    command:    u8,     // current command
    if_flag:    bool,   // if TRUE: exe only if preceding exe'd
    arg1:       i32,    //
    arg2:       i32,    // Arguments to the command
    arg3:       i32,    //

    // Commands:
    // 'M': Read a mobile
    // 'O': Read an object
    // 'G': Give obj to mob
    // 'P': Put obj in obj
    // 'G': Obj to char (What?!? 'G' is above -Sean)
    // 'E': Obj to char equip
    // 'D': Set state of door
}

struct ZoneData {
    name:       String,         // name of this zone
    lifespan:   i32,            // how long between resets (minutes)
    age:        i32,            // current age of this zone (minutes)
    top:        i32,            // upper limit for rooms in this zone

    reset_mode: ResetMode,      // conditions for reset
    cmd:        Vec<ResetCom>,  // command table for reset
}

pub static TICS: AtomicUsize = ATOMIC_USIZE_INIT;

pub struct Game {
    descriptor_list:    Vec<DescriptorData>,
    lawful:             bool,
    wizlock:            bool,
    slow_death:         bool,
    shutdown:           bool,
    reboot:             bool,
    no_specials:        bool,
    weather_info:       WeatherData,
    news:               String,
    credits:            String,
    motd:               String,
    help:               String,
    info:               String,
    wizlist:            String,
    mob_f:              File,
    obj_f:              File,
    help_f:             Option<File>,
    help_index:         HashMap<String, u64>,
    player_table:       HashMap<String, u64>,
    zone_table:         Vec<ZoneData>,
    shutdown_signal:    Receiver<Signal>,
    hup_signal:         Receiver<Signal>,
    log_signal:         Receiver<Signal>,
}

impl Game {
    pub fn new(lawful: bool, no_specials: bool) -> Game {
        log("Boot db -- BEGIN");

        log("Reading newsfile, credits, help-page, info and motd.");
        let news = file_to_string(constants::NEWS_FILE);
        let credits = file_to_string(constants::CREDITS_FILE);
        let motd = file_to_string(constants::MOTD_FILE);
        let help = file_to_string(constants::HELP_PAGE_FILE);
        let info = file_to_string(constants::INFO_FILE);
        let wizlist = file_to_string(constants::WIZLIST_FILE);

        log("Opening mobile, object and help files.");

        let mob_f = File::open(constants::MOB_FILE).expect("boot");
        let obj_f = File::open(constants::OBJ_FILE).expect("boot");
        let mut help_f = File::open(constants::HELP_KWRD_FILE).ok();
        let help_index = match help_f.as_mut() {
            None => HashMap::new(),
            Some(file) => build_help_index(file),
        };

        log("Loading zone table.");
        let zone_table = boot_zones();

        log("Loading rooms.");
        let world = boot_world(&zone_table);

        let mut game = Game {
            descriptor_list: Vec::new(),
            lawful,
            wizlock: false,
            slow_death: false,
            shutdown: false,
            reboot: false,
            no_specials,
            weather_info: WeatherData {
                pressure: 0,
                change: 0,
                sky: Sky::Cloudless,
                sunlight: Sunlight::Dark,
            },
            news,
            credits,
            motd,
            help,
            info,
            wizlist,
            mob_f,
            obj_f,
            help_f,
            help_index,
            player_table: HashMap::new(),
            zone_table,
            shutdown_signal: chan_signal::notify(&[Signal::USR2]),
            hup_signal: chan_signal::notify(&[Signal::HUP, Signal::INT, Signal::TERM]),
            log_signal: chan_signal::notify(&[Signal::ALRM]),
        };

        // had to be moved to after game was created
        log("Resetting the game time:");
        game.reset_time();

        game
    }

    pub fn game_loop(&mut self, s: RawFd) {
        
    }

    fn reset_time(&mut self) {
        let beginning_of_time = UNIX_EPOCH + Duration::from_secs(650336715);
        let time_info = mud_time_passed(SystemTime::now().duration_since(beginning_of_time).expect("Clock is before beginning of time."));
        self.weather_info.sunlight = match time_info.hours {
            5 => {
                Sunlight::Sunrise
            }
            6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18 | 19 | 20 => {
                Sunlight::Light
            }
            21 => {
                Sunlight::Sunset
            }
            0 | 1 | 2 | 3 | 4 | 22 | 23 | _ => {
                Sunlight::Dark
            }
        };

        log(&format!("   Current Gametime: {}H {}D {}M {}Y.",
            time_info.hours, time_info.day,
            time_info.month, time_info.year));
        
        self.weather_info.pressure = 960;
        self.weather_info.pressure += if time_info.month >= 7 && time_info.month <= 12 {
            dice(1, 50) as i32
        } else {
            dice(1, 80) as i32
        };

        self.weather_info.change = 0;

        self.weather_info.sky = if self.weather_info.pressure <= 980 {
            Sky::Lightning
        } else if self.weather_info.pressure <= 1000 {
            Sky::Raining
        } else if self.weather_info.pressure <= 1020 {
            Sky::Cloudy
        } else {
            Sky::Cloudless
        };
    }
}

fn file_to_string(name: &str) -> String {
    let mut file = File::open(name).expect("open file-to-string");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("read_to_string file-to-string");
    contents
}

fn boot_zones() -> Vec<ZoneData> {
    let mut zone_table = Vec::new();

    let file = File::open(constants::ZONE_FILE).expect("boot_zones");
    let mut reader = BufReader::new(file);

    loop {
        let _ = reader.read_line(&mut String::new()); // Read and ignore "#d"

        let check = fread_string(&mut reader);
        if check.chars().nth(0).unwrap() == '$' {
            // end of file
            break;
        }

        let mut line = String::new();
        if reader.read_line(&mut line).is_err() {
            break;
        }
        let mut words = line.split_whitespace();
        let top = words.next().unwrap().parse::<i32>().expect("parse zone top");
        let lifespan = words.next().unwrap().parse::<i32>().expect("parse zone lifespan");
        let reset_mode = match words.next().unwrap().parse::<i32>().expect("parse zone reset_mode") {
            0 => ResetMode::DoNot,
            1 => ResetMode::NoPC,
            2 => ResetMode::Do,
            _ => panic!("Invalid reset_mode"),
        };

        let mut cmd = Vec::new();
        loop {
            let mut line = String::new();
            if reader.read_line(&mut line).is_err() {
                break;
            }
            let mut words = line.split_whitespace();

            let command = words.next().unwrap().as_bytes()[0];
            if command == b'S' {
                break;
            }
            if command == b'*' {
                continue;
            }
            let if_flag = words.next().unwrap().parse::<u8>().expect("parse zone command if_flag") != 0;
            let arg1 = words.next().unwrap().parse::<i32>().expect("parse zone command arg1");
            let arg2 = words.next().unwrap().parse::<i32>().expect("parse zone command arg2");
            let arg3 = if command == b'M' || command == b'O' || command == b'P' || command == b'D' {
                words.next().unwrap().parse::<i32>().expect("parse zone command arg3")
            } else {
                0
            };

            cmd.push(ResetCom {
                command, if_flag, arg1, arg2, arg3,
            });
        }

        zone_table.push(ZoneData {
            name: check,
            lifespan,
            age: 0,
            top,
            reset_mode,
            cmd,
        })
    }
    zone_table
}

fn boot_world(zone_table: &Vec<ZoneData>) {
    let file = File::open(constants::WORLD_FILE).expect("boot_world: could not open world file.");
    let mut reader = BufReader::new(file);

    let mut zone = 0;

    loop {
        let virtual_nr = read_number(&mut reader);
        let temp = fread_string(&mut reader);
        if temp.bytes().nth(0).unwrap() == b'$' {
            break;
        }
        
        let description = fread_string(&mut reader);

        let mut line = String::new();
        reader.read_line(&mut line).unwrap();
        let mut words = line.split_whitespace();
        if !zone_table.is_empty() {
            words.next(); // sproctor: this was originally the zone number? why is this so complicated now?

			// OBS: Assumes ordering of input rooms

            assert!(virtual_nr > if zone > 0 { zone_table[zone - 1].top } else { -1 },
                "Room nr {} is below zone {}.\n", virtual_nr, zone);
            while virtual_nr > zone_table[zone].top {
                zone += 1;
                assert!(zone < zone_table.len(), "Room {} is outside of any zone.\n", virtual_nr);
            }
        }
        let mut room_flags = RoomFlags::from_bits(words.next().unwrap().parse::<u16>().unwrap());
        let mut sector_type = SectorType::from(words.next().unwrap().parse::<u8>().unwrap());

        loop {
            let mut line = String::new();
            reader.read_line(&mut line).unwrap();
            let chk = line.trim();
            let mut dir_option = Vec::new();
            match chk.bytes().nth(0).unwrap() {
                b'D' => {
                    let dir = chk.get(1..).unwrap().parse::<usize>().unwrap();
                    dir_option[dir] = setup_dir(&mut reader);
                },
                _ => (),
            }
        }
    }
}

// read direction data
fn setup_dir<R: Read>(reader: &mut BufReader<R>) -> RoomDirectionData {

    let general_description = fread_string(reader);
    let keyword = fread_string(reader);

    let mut line = String::new();
    reader.read_line(&mut line).unwrap();
    let mut words = line.split_whitespace();
    let exit_info = match words.next().unwrap().parse::<u32>().unwrap() {
        1 => EX_ISDOOR,
        2 => EX_ISDOOR | EX_PICKPROOF,
        _ => ExitFlags::empty(),
    };
    let key = words.next().unwrap().parse::<i16>().unwrap();
    let to_room = words.next().unwrap().parse::<i16>().unwrap();

    RoomDirectionData {
        general_description,
        keyword,
        exit_info,
        key,
        to_room,
     }
}

fn read_number<R: Read>(reader: &mut BufReader<R>) -> i32 {
    // TODO: make the following static
    let re: Regex = Regex::new(r"\s*#(\d+)").expect("read_number regex");

    let mut line = String::new();
    reader.read_line(&mut line).unwrap();
    let cap = re.captures(&mut line).unwrap();
    cap[1].parse::<i32>().unwrap()
}

fn fread_string<R: Read>(reader: &mut BufReader<R>) -> String {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fread_string_test() {
        assert_eq!("test", fread_string(&mut BufReader::new(Cursor::new(String::from("test~\n")))));
        assert_eq!("test", fread_string(&mut BufReader::new(Cursor::new(String::from("test~foo\n")))));
        assert_eq!("test", fread_string(&mut BufReader::new(Cursor::new(String::from("test~")))));
    }
}