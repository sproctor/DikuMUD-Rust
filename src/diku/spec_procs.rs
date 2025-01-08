use std::ops::{Deref, DerefMut};
use std::rc::Rc;

use diku::act_comm::do_say;
use diku::comm::act;
use diku::fight::hit;
use diku::spells::*;
use diku::structs::*;
use diku::utility::*;

pub fn puff(ch: Rc<CharData>, cmd: i32, _arg: &str, _game: &Game) -> bool {
    if cmd != 0 {
        return false;
    }

    match number(0, 60) {
        0 => {
            do_say(&ch, "My god! It's full of stars!", 0);
            true
        },
        1 => {
            do_say(&ch, "How'd all those fish get up here?", 0);
            true
        },
        2 => {
            do_say(&ch, "I'm a very female dragon.", 0);
            true
        }
        3 => {
            do_say(&ch, "I've got a peaceful, easy feeling.", 0);
            true
        }
        _ => false,
    }
}

pub fn cityguard(ch: Rc<CharData>, cmd: i32, _arg: &str, game: &Game) -> bool {
    if cmd != 0 || !ch.awake() || ch.get_pos() == Position::Fighting {
        return false;
    }

    let mut max_evil = 300;
    let mut evil = None;

    for tch in ch.in_room.people.borrow().iter() {
        if tch.get_alignment() < max_evil &&
                (tch.is_npc() || tch.specials.borrow().fighting.as_ref().map_or(false, |x| x.is_npc())) {
            max_evil = tch.get_alignment();
            evil = Some(Rc::clone(tch));
        }
    }

    match evil {
        Some(evil) => {
            if evil.specials.borrow().fighting.as_ref().map_or(false, |x| x.is_evil()) {
                act("$n screams 'PROTECT THE INNOCENT!  BANZAI!!! CHARGE!!! ARARARAGGGHH!'",
                    false, Rc::as_ref(&ch), None, None, None, None, VictimType::ToRoom);
                hit(Rc::clone(&ch), evil, TYPE_UNDEFINED, game);
                true
            } else {
                false
            }
        },
        None => false,
    }
}