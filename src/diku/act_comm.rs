

use diku::comm::{act, send_to_char};
use diku::structs::*;

pub fn do_say(ch: &CharData, argument: &str, _cmd: i32) {
    let argument = argument.trim();

    if argument.is_empty() {
        send_to_char("Yes, but WHAT do you want to say?\n\r", ch);
    } else {
        let buf = format!("$n says '{}'", argument);
        act(&buf, false, ch, None, None, None, None, VictimType::ToRoom);
    }
}