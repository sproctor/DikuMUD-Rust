use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

use diku::constants::{POSEMESS_FILE, SOCMESS_FILE};
use diku::types::{PoseType, SocialMessg};
use diku::utility::read_number;

pub fn boot_social_messages() -> Vec<SocialMessg> {
    let file = File::open(SOCMESS_FILE).expect("boot_social_messages");
    let mut reader = BufReader::new(file);

    let mut soc_mess_list = Vec::new();
    loop {
        let tmp = read_number(&mut reader, true).unwrap();
        if tmp < 0 {
            break;
        }
        let hide = read_number(&mut reader, true).unwrap();
        let min_victim_position = match read_number(&mut reader, true) {
            Ok(v) => v,
            Err(e) => panic!("Error in social message #{}: {}", tmp, e),
        };
        let char_no_arg = fread_action(&mut reader);
        let others_no_arg = fread_action(&mut reader);
        let char_found = fread_action(&mut reader);

        let mut others_found = None;
        let mut vict_found = None;
        let mut not_found = None;
        let mut char_auto = None;
        let mut others_auto = None;

        if char_found.is_some() {
            others_found = fread_action(&mut reader);
            vict_found = fread_action(&mut reader);
            not_found = fread_action(&mut reader);
            char_auto = fread_action(&mut reader);
            others_auto = fread_action(&mut reader);
        }

        soc_mess_list.push(SocialMessg {
            act_nr: tmp,
            hide,
            min_victim_position,
            char_no_arg,
            others_no_arg,
            char_found,
            others_found,
            vict_found,
            not_found,
            char_auto,
            others_auto,
        });
    }

    soc_mess_list
}

pub fn boot_pose_messages() -> Vec<PoseType> {
    let file = File::open(POSEMESS_FILE).unwrap();
    let mut reader = BufReader::new(file);

    let mut pose_messages = Vec::new();
    loop {
        let level = read_number(&mut reader, true).unwrap();
        if level < 0 {
            break;
        }
        let mut pose = PoseType {
            level,
            poser_msg: Default::default(),
            room_msg: Default::default(),
        };
        for class in 0..4 {
            pose.poser_msg[class] = fread_action(&mut reader).unwrap();
            pose.room_msg[class] = fread_action(&mut reader).unwrap();
        }
        pose_messages.push(pose);
    }

    pose_messages
}

fn fread_action<R: Read>(reader: &mut BufReader<R>) -> Option<String> {
    let mut buf = String::new();

    reader.read_line(&mut buf).unwrap();

    if buf.bytes().nth(0).unwrap() == b'#' {
        None
    } else {
        Some(String::from(buf.trim()))
    }
}