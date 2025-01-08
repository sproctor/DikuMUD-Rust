use std::rc::Rc;

use diku::act_movement::do_simple_move;
use diku::comm::act;
use diku::structs::*;
use diku::utility::number;

pub fn do_flee(ch: Rc<CharData>, argument: &str, cmd: i32, game: &Game) {
    if !ch.specials.borrow().fighting.is_some() {
        for i in 0..6 {
            let attempt = Direction::from(number(0, 5) as u8); // Select a random direction
            if ch.can_go(attempt) &&
            !game.world.get(&ch.exit(attempt).unwrap().to_room.unwrap()).unwrap().room_flags.contains(RoomFlags::DEATH) {
                act("$n panics, and attempts to flee.", true, &ch, None, None, None, None, VictimType::ToRoom);
                let die = do_simple_move(Rc::clone(&ch), attempt, false, game);
                
            }
        }
    }
}