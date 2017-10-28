use std::cmp::{max, min};

use diku::comm::send_to_char;
use diku::constants::*;
use diku::structs::*;
use diku::utility::number;

// When age < 15 return the value p0
// When age in 15..29 calculate the line between p1 & p2
// When age in 30..44 calculate the line between p2 & p3
// When age in 45..59 calculate the line between p3 & p4
// When age in 60..79 calculate the line between p4 & p5
// When age >= 80 return the value p6
fn graf(age: u16, p0: u16, p1: u16, p2: u16, p3: u16, p4: u16, p5: u16, p6: u16) -> u16 {
    if age < 15 {
        p0 /* < 15   */
    } else if age <= 29 {
        p1 + (((age - 15) * (p2 - p1)) / 15) /* 15..29 */
    } else if age <= 44 {
        p2 + (((age - 30) * (p3 - p2)) / 15) /* 30..44 */
    } else if age <= 59 {
        p3 + (((age - 45) * (p4 - p3)) / 15) /* 45..59 */
    } else if age <= 79 {
        p4 + (((age - 60) * (p5 - p4)) / 20) /* 60..79 */
    } else {
        p6 /* >= 80 */
    }
}

/* The three MAX functions define a characters Effective maximum */
/* Which is NOT the same as the ch->points.max_xxxx !!!          */
pub fn mana_limit(ch: &CharData) -> i16 {
    let max = if !ch.is_npc() {
        (100) /* + (graf(age(ch).year, 0,0,10,30,50,70,60)); */
    } else {
        100
    };

    max
}

pub fn move_limit(ch: &CharData) -> i16 {
    let max = if !ch.is_npc() {
        /* HERE SHOULD BE CON CALCULATIONS INSTEAD */
        graf(ch.get_age(), 50, 70, 160, 120, 100, 40, 20) as i16
    } else {
        ch.points.borrow().max_mov
    };

    /* Class/Level calculations */

    /* Skill/Spell calculations */

    max
}

pub fn hit_limit(ch: &CharData) -> i16 {
    let max = if !ch.is_npc() {
        ch.points.borrow().max_hit + graf(ch.get_age(), 2, 4, 17, 14, 8, 4, 3) as i16
    } else {
        ch.points.borrow().max_hit
    };


    /* Class/Level calculations */

    /* Skill/Spell calculations */
    max
}

fn advance_level(ch: &CharData) {
    let mut add_hp = CON_APP[ch.get_con() as usize].hitp;

    add_hp += match ch.get_class() {
        Class::MagicUser => number(3, 8),
        Class::Cleric => number(5, 10),
        Class::Thief => number(7, 13),
        Class::Warrior => number(10, 15),
    } as i16;
}

fn set_title(ch: &CharData) {
    ch.player.borrow_mut().title = if ch.get_sex() == Sex::Male {
        TITLES[ch.get_class() as usize - 1][ch.get_level() as usize].title_m } else {
        TITLES[ch.get_class() as usize - 1][ch.get_level() as usize].title_f };
}

pub fn gain_exp(ch: &CharData, gain: i32) {
    let mut gain = gain;
    let mut is_altered = false;

    if ch.is_npc() || (ch.get_level() < 21 && ch.get_level() > 0) {
        if gain > 0 {
            gain = min(100_000, gain);
            ch.points.borrow_mut().exp += gain;
            if !ch.is_npc() {
                let mut i: u8 = 0;
                while TITLES[ch.get_class() as usize - 1][i as usize].exp <= ch.get_exp() {
                    if i > ch.get_level() {
                        send_to_char(String::from("You rase a level\n\r"), &ch);
                        ch.player.borrow_mut().level = i;
                        advance_level(ch);
                        is_altered = true;
                    }
                }
            }
        }

        if gain < 0 {
            gain = max(-500_000, gain);
            ch.points.borrow_mut().exp += gain;
            if ch.get_exp() < 0 {
                ch.points.borrow_mut().exp = 0;
            }
        }

        if is_altered {
            set_title(ch);
        }
    }
}