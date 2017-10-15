

use diku::comm::{act, send_to_char};
use diku::types::*;

pub fn do_say(world: &RoomTable, ch: &mut CharData, argument: &str, cmd: i32) {
    let argument = argument.trim();

    if argument.is_empty() {
        send_to_char("Yes, but WHAT do you want to say?\n\r".to_string(), ch);
    } else {
        let buf = format!("$n says '{}'", argument);
        act(buf, world, false, ch, None, None, VictimType::ToRoom);
    }
}