
use diku::act_comm::do_say;
use diku::types::*;
use diku::utility::number;

fn puff(world: &RoomTable, ch: &mut CharData, cmd: i32, arg: &str) -> i32 {
    if cmd != 0 {
        return 0;
    }

    match number(0, 60) {
        0 => do_say(world, ch, "My god! It's full of stars!", 0),
        _ => (),
    }

    0
}