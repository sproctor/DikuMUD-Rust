use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::os::unix::io::RawFd;
use std::sync::atomic::{AtomicUsize, ATOMIC_USIZE_INIT};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use chan::Receiver;
use chan_signal;
use chan_signal::Signal;

use diku::constants;
use diku::modify::build_help_index;
use diku::types::{DescriptorData, Sky, Sunlight, WeatherData};
use diku::utility::{dice, log, mud_time_passed};

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
        let help_index = match help_f {
            None => HashMap::new(),
            Some(mut file) => build_help_index(&mut file),
        };

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
            mob_f: File::open(constants::MOB_FILE).expect("boot"),
            obj_f: File::open(constants::OBJ_FILE).expect("boot"),
            help_f: File::open(constants::HELP_KWRD_FILE).ok(),
            help_index,
            player_table: HashMap::new(),
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