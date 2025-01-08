use std::rc::Rc;

use diku::comm::{act, send_to_char};
use diku::constants::*;
use diku::handler::char_from_room;
use diku::interpreter::special;
use diku::structs::*;

pub fn do_simple_move(ch: Rc<CharData>, cmd: Direction, following: bool,
game: &Game) -> bool {
    // Assumes,
    //      1. That there is no master and no followers.
    //      2. That the direction exists.

    // Returns :
    // 1 : If success.
    // 0 : If fail
    // -1 : If dead (this was never checked, so removing it -sproctor)
    if special(Rc::clone(&ch), cmd as i32 + 1, "", game) {
        return false;
    }
    
    let need_movement = (MOVEMENT_LOSS[ch.in_room.sector_type as usize] +
    MOVEMENT_LOSS[game.world[&ch.in_room.dir_option[&cmd].to_room.unwrap()].sector_type as usize]) / 2;

    if ch.in_room.sector_type == SectorType::WaterNoSwim ||
    game.world[&ch.in_room.dir_option[&cmd].to_room.unwrap()].sector_type == SectorType::WaterNoSwim {
        let mut has_boat = false;
        // See if char is carrying a boat
        for obj in &ch.carrying {
            if obj.obj_flags.type_flag == ItemType::Boat {
                has_boat = true;
            }
        }
        if !has_boat {
            send_to_char("You need a boat to go there.\n\r", &ch);
            return false;
        }
    }

    if ch.get_move() < need_movement as i16 && !ch.is_npc() {
        if !following {
            send_to_char("You are too exhausted.\n\r", &ch);
        } else {
            send_to_char("You are too exhausted to follow.\n\r", &ch);
        }
        return false;
    }

    if ch.get_level() < 21 && !ch.is_npc() {
        ch.points.borrow_mut().mov -= need_movement as i16;
    }

    if !ch.is_affected(AffectedFlags::AFF_SNEAK) {
        let tmp = format!("$n leaves {}.", DIRS[cmd as usize]);
        act(&tmp, true, &ch, None, None, None, None, VictimType::ToRoom);
    }

    let was_in = &ch.in_room;

    char_from_room(&ch);

    return true;
}