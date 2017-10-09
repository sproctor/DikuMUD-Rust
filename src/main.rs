#[macro_use] extern crate bitflags;
#[macro_use] extern crate enum_map;
extern crate libc;
extern crate getopts;
extern crate rand;
extern crate time;
extern crate nix;
extern crate chan;
extern crate chan_signal;
extern crate regex;
#[macro_use] extern crate serde_derive;
extern crate bincode;

use std::env;
use std::path::Path;
use getopts::Options;

mod diku;

use diku::comm::init_socket;
use diku::constants;
use diku::game::Game;
use diku::signals::signal_setup;
use diku::utility::log;

fn main() {
    // Parse args
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optflag("l", "", "Enable lawful mode");
    opts.optopt("d", "", "Directory contain", "pathname");
    opts.optflag("s", "", "No specials");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };
    let lawful = matches.opt_present("l");
    let dir = match matches.opt_str("d") {
        Some(d) => { d }
        None => { constants::DFLT_DIR.to_string() }
    };
    let no_specials = matches.opt_present("s");
    let port = if !matches.free.is_empty() {
        match matches.free[0].parse::<u16>() {
            Ok(p) => { p }
            Err(e) => {
                println!("Invalid port: {}", e.to_string());
                print_usage(&program, opts);
                return;
            }
        }
    } else {
        constants::DFLT_PORT
    };

    log(&format!("Running game on port {}", port));

    assert!(env::set_current_dir(Path::new(&dir)).is_ok());
    log(&format!("Using {} as data directory.", dir));

    run_the_game(port, lawful, no_specials);

}

fn run_the_game(port: u16, lawful: bool, no_specials: bool) {
        log("Signal trapping.");
        signal_setup();

        log("Opening mother connection.");
        let s = init_socket(port);

        // Not bothering with weird "lawful" stuff

        let mut game = Game::new(lawful, no_specials); // boot_db()

        log("Entering game loop.");

        game.game_loop(s);
}

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} [options] [ port # ]", program);
    print!("{}", opts.usage(&brief));
}