use std::process::exit;
use std::sync::atomic::Ordering;
use libc::c_int;
use libc;
use std::thread::{sleep, spawn};
use std::time::Duration;

use diku::game;
use diku::utility::log;

pub fn signal_setup() {
    unsafe {
        libc::signal(libc::SIGPIPE, libc::SIG_IGN);
    }

    // TODO: Add deadlock detection, use a signal handler-safe version of log()
    spawn(move || {
        loop {
            sleep(Duration::from_secs(900)); // 15 minutes

            let tics = game::TICS.load(Ordering::Relaxed);
            if tics == 0 {
                log("CHECKPOINT shutdown: tics not updated");
                panic!("Deadlock detected");
            } else {
                game::TICS.store(0, Ordering::Relaxed);
            }
        }
    });
}

extern fn shutdown_request(_: c_int) {
    log("Received USR2 - shutdown request");
    //unsafe { game::shutdown = true; }
}

// kick out players etc
extern fn hupsig(_: c_int) {
    log("Received SIGHUP, SIGINT, or SIGTERM. Shutting down");
    exit(0); // something more elegant should perhaps be substituted
}

extern fn logsig(_: c_int) {
    log("Signal received. Ignoring.");
}

