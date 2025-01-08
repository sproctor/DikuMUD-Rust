use std::cell::RefCell;
use std::cmp::max;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, SeekFrom};
use std::io::prelude::*;
use std::str;
use std::os::unix::io::RawFd;
use std::rc::Rc;
use std::sync::atomic::{AtomicUsize, ATOMIC_USIZE_INIT};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use bincode::{deserialize_from, Infinite};
use chan_signal;
use chan_signal::Signal;

use diku::act_social::{boot_pose_messages, boot_social_messages};
use diku::constants;
use diku::fight::load_messages;
use diku::modify::{build_help_index};
use diku::spec_assign::assign_mobiles;
use diku::structs::*;
use diku::utility::{current_line_number, dice, fread_string, log, mud_time_passed, read_char, read_number};

pub static TICS: AtomicUsize = ATOMIC_USIZE_INIT;

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
        let help_f = File::open(constants::HELP_KWRD_FILE).ok();
        let help_index = match help_f.as_ref() {
            None => HashMap::new(),
            Some(file) => build_help_index(&mut BufReader::new(file)),
        };

        log("Loading zone table.");
        let zone_table = boot_zones();

        log("Loading rooms.");
        let world = boot_world(&zone_table);

        // Using a hash table instead of renumbering rooms -sproctor

        log("Generating index tables for mobile files.");
	    let mut mob_index = generate_indices(&mut BufReader::new(&mob_f));

        log("Generating index tables for object files.");
	    let obj_index = generate_indices(&mut BufReader::new(&obj_f));

        // skip renumbering zone table - sproctor

        log("Generating player index.");
	    let player_table = build_player_index();

        log("Loading fight messages.");
	    let fight_messages = load_messages();

        log("Loading social messages.");
	    let soc_mess_list = boot_social_messages();

        log("Loading pose messages.");
	    let pose_messages = boot_pose_messages();

        log("Assigning function pointers:");
        if !no_specials {
            log("   Mobiles.");
            assign_mobiles(&mut mob_index);
        }

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
            mob_index,
            obj_index,
            player_table,
            zone_table,
            world,
            combat_list: RefCell::new(Vec::new()),
            fight_messages,
            soc_mess_list,
            pose_messages,
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

// generate index table for the player file
fn build_player_index() -> HashMap<String, u64> {
    let file = File::open(constants::PLAYER_FILE).expect("build player index");
    let mut reader = BufReader::new(file);

    let mut player_table = HashMap::new();
    let mut nr = 0;

    loop {
        let dummy: CharFileU = match deserialize_from(&mut reader, Infinite) {
            Ok(v) => v,
            Err(_) => break,
        };
        
        let name = String::from(str::from_utf8(&dummy.name).unwrap().to_lowercase());
        player_table.insert(name, nr);
        nr += 1;
    }

    player_table
}

// generate index table for object or monster file
fn generate_indices<R: Read + Seek>(reader: &mut BufReader<R>) -> HashMap<u32, IndexData> {
    let mut index = HashMap::new();

    // This is unneeded -sproctor
    reader.seek(SeekFrom::Start(0)).unwrap();

    loop {
        let mut buf = String::new();
        reader.read_line(&mut buf).unwrap();
        match buf.chars().nth(0).unwrap() {
            '#' => {
                let virtual_nr = buf.get(1..).unwrap().trim().parse::<u32>().unwrap();
                let pos = reader.seek(SeekFrom::Current(0)).unwrap();
                index.insert(virtual_nr, IndexData {
                    //virtual_nr,
                    pos,
                    number: 0,
                    func: None,
                });
            },
            '$' => break,
            _ => (),
        }
    }

    index
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
        // ignore #nn line
        assert_eq!(read_char(&mut reader), b'#', "Expected '#' in zone file");
        read_number::<File, u32>(&mut reader, true).unwrap();

        let check = fread_string(&mut reader);
        if check.bytes().nth(0).unwrap() == b'$' {
            // end of file
            break;
        }

        let top = read_number(&mut reader, true).expect("parse zone top");
        let lifespan = read_number(&mut reader, true).expect("parse zone lifespan");
        let reset_mode = match read_number(&mut reader, true).expect("parse zone reset_mode") {
            0 => ResetMode::DoNot,
            1 => ResetMode::NoPC,
            2 => ResetMode::Do,
            _ => panic!("Invalid reset_mode"),
        };

        let mut cmd = Vec::new();
        loop {

            let command = read_char(&mut reader);
            if command == b'S' {
                break;
            }
            if command == b'*' {
                reader.read_line(&mut String::new()).unwrap(); // skip command
                continue;
            }
            let if_flag = read_number::<File, u32>(&mut reader, false).expect("parse zone command if_flag") != 0;
            let arg1 = read_number(&mut reader, false).expect("parse zone command arg1");
            let arg2 = read_number(&mut reader, false).expect("parse zone command arg2");
            let arg3 = if command == b'M' || command == b'O' || command == b'P' || command == b'D' {
                read_number(&mut reader, false).expect("parse zone command arg3")
            } else {
                0
            };

            reader.read_line(&mut String::new()).unwrap(); // read comment

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

fn boot_world(zone_table: &Vec<ZoneData>) -> HashMap<u32, RoomData> {
    let file = File::open(constants::WORLD_FILE).expect("boot_world: could not open world file.");
    let mut reader = BufReader::new(file);

    let mut world = HashMap::new();
    let mut zone = 0;

    loop {
        assert_eq!(read_char(&mut reader), b'#', "Expected '#' in world file.");
        let virtual_nr = read_number(&mut reader, true).unwrap();
        let temp = fread_string(&mut reader);
        if temp.bytes().nth(0).unwrap() == b'$' {
            break;
        }
        let description = fread_string(&mut reader);

        if !zone_table.is_empty() {
            read_number::<File, i32>(&mut reader, true).unwrap(); // sproctor: this was originally the zone number? why is this so complicated now?

			// OBS: Assumes ordering of input rooms

            assert!(zone == 0 || virtual_nr > zone_table[zone - 1].top,
                "Room nr {} is below zone {}.\n", virtual_nr, zone);
            while virtual_nr > zone_table[zone].top {
                zone += 1;
                assert!(zone < zone_table.len(), "Room {} is outside of any zone.\n", virtual_nr);
            }
        }
        let room_flags = RoomFlags::from_bits(read_number(&mut reader, true).unwrap()).unwrap();
        let sector_type = SectorType::from(max(read_number::<File, i8>(&mut reader, true).unwrap(), 0) as u8);

        let mut dir_option = HashMap::new();
        let mut ex_description = Vec::new();
        loop {
            let mut line = String::new();
            reader.read_line(&mut line).unwrap();
            let chk = line.trim();
            
            match chk.bytes().nth(0).unwrap() {
                b'D' => {
                    let dir = Direction::from(chk.get(1..).unwrap().parse::<u8>().unwrap());
                    dir_option.insert(dir, Rc::new(setup_dir(&mut reader)));
                },
                b'E' => {
                    let keyword = fread_string(&mut reader);
                    let description = fread_string(&mut reader);
                    ex_description.push(ExtraDescrData { keyword, description })
                },
                b'S' => break,
                _ => panic!("Invalid value in room extra fields: {}", chk),
            }
        }
        world.insert(virtual_nr, RoomData {
            number: virtual_nr,
            zone: zone as u16,
            sector_type,
            name: temp,
            description,
            ex_description,
            dir_option,
            room_flags,
            light: RefCell::new(0),
            funct: None,
            contents: RefCell::new(Vec::new()),
            people: RefCell::new(Vec::new()),
        });
    }
    world
}

// read direction data
fn setup_dir<R: Read + Seek>(reader: &mut BufReader<R>) -> RoomDirectionData {

    let general_description = fread_string(reader);
    let keyword = fread_string(reader);

    let exit_info = match read_number::<R, u32>(reader, true).unwrap() {
        1 => ExitFlags::EX_ISDOOR,
        2 => ExitFlags::EX_ISDOOR | ExitFlags::EX_PICKPROOF,
        _ => ExitFlags::empty(),
    };
    let key_nr = read_number::<R, i32>(reader, true).unwrap();
    let key = if key_nr < 0 { None } else { Some(key_nr as u32) };
    let room_nr = read_number::<R, i32>(reader, true).unwrap();
    let to_room = if room_nr < 0 { None } else { Some(room_nr as u32) };

    RoomDirectionData {
        general_description,
        keyword,
        exit_info,
        key,
        to_room,
     }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn fread_string_test() {
        assert_eq!("test", fread_string(&mut BufReader::new(Cursor::new(String::from("test~\n")))));
        assert_eq!("test", fread_string(&mut BufReader::new(Cursor::new(String::from("test~foo\n")))));
        assert_eq!("test", fread_string(&mut BufReader::new(Cursor::new(String::from("test~")))));
    }
}