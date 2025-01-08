use std::cmp::max;
use std::rc::Rc;

use enum_map::EnumMap;

use diku::structs::*;

const FILL: &'static [&'static str] = &[
    "in",
    "from",
    "with",
    "the",
    "on",
    "at",
    "to",
    ];

fn search_block(arg: &str, list: &[&str], exact: bool) -> Option<usize> {
    let word = arg.to_lowercase();

    if exact {
        for (i, &item) in list.iter().enumerate() {
            if word == item {
                return Some(i);
            }
        }
    } else {
        let length = max(word.len(), 1);
        for (i, &item) in list.iter().enumerate() {
            if word == item[0..length] {
                return Some(i);
            }
        }
    }
    
    None
}

pub fn fill_word(argument: &str) -> bool {
    search_block(argument, FILL, true).is_some()
}

pub fn special(ch: Rc<CharData>, cmd: i32, arg: &str, game: &Game) -> bool {

    // special in room?
    if ch.in_room.funct.is_some() &&
    (ch.in_room.funct.unwrap())(Rc::clone(&ch), cmd, arg, game) {
        return true;
    }

    // special in equipment list?
    for (_, e) in ch.equipment.iter() {
        if e.is_some() {
            let func = game.obj_index.get(&e.as_ref().unwrap().item_number).unwrap().func;
            if func.is_some() && func.unwrap()(Rc::clone(&ch), cmd, arg, game) {
                return true;
            }
        }
    }

    // special in inventory?
    for i in &ch.carrying {
        let func = game.obj_index.get(&i.item_number).unwrap().func;
        if func.is_some() && func.unwrap()(Rc::clone(&ch), cmd, arg, game) {
            return true;
        }
    }

    // special in mobile present?
    for k in ch.in_room.people.borrow().iter() {
        if k.is_mob() {
            let func = game.mob_index.get(&k.nr.unwrap()).unwrap().func;
            if func.is_some() && func.unwrap()(Rc::clone(&ch), cmd, arg, game) {
                return true;
            }
        }
    }

    // special in object present?
    for i in ch.in_room.contents.borrow().iter() {
        let func = game.obj_index.get(&i.item_number).unwrap().func;
        if func.is_some() && func.unwrap()(Rc::clone(&ch), cmd, arg, game) {
            return true;
        }
    }

    return false;
}