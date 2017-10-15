use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

use diku::constants::MESS_FILE;
use diku::types::*;
use diku::utility::{fread_string, read_line_trim, read_number};

pub fn load_messages() -> HashMap<u32, Vec<MessageType>> {
    let file = File::open(MESS_FILE).expect("read messages");
    let mut reader = BufReader::new(file);
    
    let mut fight_messages = HashMap::new();

    while read_line_trim(&mut reader).bytes().nth(0) == Some(b'M') {
        let a_type = read_number(&mut reader, true).unwrap();
        let message_list = fight_messages.entry(a_type).or_insert(Vec::new());

        let die_msg = read_msgs(&mut reader);
        let miss_msg = read_msgs(&mut reader);
        let hit_msg = read_msgs(&mut reader);
        let god_msg = read_msgs(&mut reader);

        message_list.push(MessageType {
            die_msg,
            miss_msg,
            hit_msg,
            god_msg,
        });
    }

    fight_messages
}

fn read_msgs<R: Read>(reader: &mut BufReader<R>) -> MsgType {
    let attacker_msg = fread_string(reader);
    let victim_msg = fread_string(reader);
    let room_msg = fread_string(reader);

    MsgType {
        attacker_msg,
        victim_msg,
        room_msg,
    }
}

pub fn hit(ch: &CharData, victim: &CharData, a_type: i32) {

}