use std::rc::Rc;

use diku::act_comm::do_say;
use diku::comm::act;
use diku::fight::hit;
use diku::spells::*;
use diku::types::*;
use diku::utility::{awake, get_alignment, get_pos, is_evil, is_npc, number};

pub fn puff(world: &RoomTable, ch: &mut CharData, cmd: i32, _arg: &str) -> bool {
    if cmd != 0 {
        return false;
    }

    match number(0, 60) {
        0 => {
            do_say(world, ch, "My god! It's full of stars!", 0);
            true
        },
        1 => {
            do_say(world, ch, "How'd all those fish get up here?", 0);
            true
        },
        2 => {
            do_say(world, ch, "I'm a very female dragon.", 0);
            true
        }
        3 => {
            do_say(world, ch, "I've got a peaceful, easy feeling.", 0);
            true
        }
        _ => false,
    }
}

pub fn cityguard(world: &RoomTable, ch: &mut CharData, cmd: i32, arg: &str) -> bool {
    if cmd != 0 || !awake(ch) || get_pos(ch) == Position::Fighting {
        return false;
    }

    let mut max_evil = 300;
    let mut evil = None;

    for tch in world[&ch.in_room].people.iter() {
        if get_alignment(tch.as_ref()) < max_evil &&
                (is_npc(tch.as_ref()) || apply_to_rc(&tch.specials.fighting, is_npc)) {
            max_evil = get_alignment(tch.as_ref());
            evil = Some(tch);
        }
    }

    match evil {
        Some(evil) => {
            if !apply_to_rc(&evil.specials.fighting, is_evil) {
                act("$n screams 'PROTECT THE INNOCENT!  BANZAI!!! CHARGE!!! ARARARAGGGHH!'",
                    world, false, ch, None, None, VictimType::ToRoom);
                hit(ch, evil, TYPE_UNDEFINED);
                true
            } else {
                false
            }
        },
        None => false,
    }
}

fn apply_to_rc<T>(rch: &Option<Rc<T>>, f: fn(&T) -> bool) -> bool {
    match rch.as_ref() {
        Some(ch) => f(ch.as_ref()),
        None => false,
    }
}