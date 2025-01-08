use std::cmp::{max, min};
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::ops::DerefMut;
use std::rc::Rc;

use diku::act_offensive::do_flee;
use diku::comm::act;
use diku::constants::*;
use diku::handler::{affect_from_char, affected_by_spell};
use diku::limits::{gain_exp, hit_limit};
use diku::spell_parser::stop_follower;
use diku::spells::*;
use diku::structs::*;
use diku::utility::{dice, fread_string, log, number, read_line_trim, read_number};

fn appear(ch: &CharData) {
    act("$n slowly fade into existence.", false, ch, None, None, None, None,
        VictimType::ToRoom);
    
    if affected_by_spell(ch, SPELL_INVISIBLE) {
        affect_from_char(ch, SPELL_INVISIBLE);
    }

    ch.specials.borrow_mut().affected_by.remove(AffectedFlags::AFF_INVISIBLE)
}

pub fn load_messages() -> HashMap<i32, Vec<MessageType>> {
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

fn update_pos(victim: &CharData) {
    if victim.get_hit() > 0 && victim.get_pos() > Position::Stunned { return; }
    else if victim.get_hit() > 0 { victim.specials.borrow_mut().position = Position::Standing; }
    else if victim.get_hit() <= -11 { victim.specials.borrow_mut().position = Position::Dead; }
    else if victim.get_hit() <= -6 { victim.specials.borrow_mut().position = Position::MortallyW; }
    else if victim.get_hit() <= -3 { victim.specials.borrow_mut().position = Position::Incap; }
    else { victim.specials.borrow_mut().position = Position::Stunned; }
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

// start one char fighting another (yes, it is horrible, I know... )
fn set_fighting(ch: Rc<CharData>, vict: Rc<CharData>,
        combat_list: &mut Vec<Rc<CharData>>) {
    assert!(!ch.specials.borrow().fighting.is_some());

    combat_list.push(Rc::clone(&ch));

    if ch.is_affected(AffectedFlags::AFF_SLEEP) {
        affect_from_char(ch.as_ref(), SPELL_SLEEP);
    }

    ch.specials.borrow_mut().fighting = Some(Rc::clone(&vict));
    ch.specials.borrow_mut().position = Position::Fighting;
}

// changing to add 'w' to mean singular and 'W' to be plural -sproctor
fn replace_string(string: &str, weapon: &AttackHitType) -> String {
    let mut buf = String::with_capacity(string.len());
    let mut chars = string.chars();
    let mut i = 0;
    while chars.nth(i).is_some() {
        let c = chars.nth(i).unwrap();
        if c == '#' {
            i += 1;
            match chars.nth(i).unwrap() {
                'W' => buf.push_str(weapon.plural),
                'w' => buf.push_str(weapon.singular),
                '#' => buf.push('#'),
                _ => (),
            }
        } else {
            buf.push(c);
        }
        i += 1;
    }
    buf
}

fn dam_send_messages(dam_type: &DamWeaponType, ch: &CharData,
        wield: Option<&ObjData>, victim: &CharData, w_type: usize) {
    let buf = replace_string(dam_type.to_room, &ATTACK_HIT_TEXT[w_type]);
    act(&buf, false, ch, wield, Some(victim), None, None, VictimType::ToNotVict);
    let buf = replace_string(dam_type.to_char, &ATTACK_HIT_TEXT[w_type]);
    act(&buf, false, ch, wield, Some(victim), None, None, VictimType::ToChar);
    let buf = replace_string(dam_type.to_victim, &ATTACK_HIT_TEXT[w_type]);
    act(&buf, false, ch, wield, Some(victim), None, None, VictimType::ToVict);

}

fn dam_message(dam: i16, ch: &CharData, victim: &CharData, w_type: i32) {

    let w_type = w_type - TYPE_HIT; // Change to base of table with text

    let wield = ch.equipment[EquipmentPosition::Wield].as_ref()
        .map(|o| Rc::as_ref(o));

    let indices = [0, 2, 4, 6, 10, 15, 20, 100];
    for i in 0..indices.len() {
        if dam <= indices[i] {
            dam_send_messages(&DAM_WEAPONS[i], ch, wield,
                victim, w_type as usize);
            break;
        }
    }
}

fn send_msg(message: &MsgType, ch: &CharData, victim: &CharData) {
    let obj = ch.equipment[EquipmentPosition::Wield].as_ref()
        .map(|o| Rc::as_ref(o));
    act(&message.attacker_msg, false, ch, obj, Some(victim), None, None,
        VictimType::ToChar);
    act(&message.victim_msg, false, ch, obj, Some(victim), None, None,
        VictimType::ToVict);
    act(&message.room_msg, false, ch, obj, Some(victim), None, None,
        VictimType::ToNotVict);
}

pub fn damage(ch: Rc<CharData>, victim: Rc<CharData>, dam: i16,
        attacktype: i32, game: &Game) {

    let mut combat_list = game.combat_list.borrow_mut();
    let mut dam = dam;

    assert!(victim.get_pos() > Position::Dead);

    if victim.get_level() > 20 && !victim.is_npc() {
        dam = 0;
    }

    if victim != ch {
        if victim.get_pos() > Position::Stunned {
            if victim.specials.borrow().fighting.is_none() {
                set_fighting(Rc::clone(&victim), Rc::clone(&ch), combat_list.deref_mut());
            }
            victim.specials.borrow_mut().position = Position::Fighting;
        }

        if ch.get_pos() > Position::Stunned {
            if ch.specials.borrow().fighting.is_none() {
                set_fighting(Rc::clone(&ch), Rc::clone(&victim), combat_list.deref_mut());
            }
        }
    }

    if victim.master.borrow().is_some() && Rc::as_ref(&victim.master.borrow().as_ref().unwrap()) == Rc::as_ref(&ch) {
        stop_follower(victim.as_ref());
    }

    if ch.is_affected(AffectedFlags::AFF_INVISIBLE) {
        appear(ch.as_ref());
    }

    if victim.is_affected(AffectedFlags::AFF_SANCTUARY) {
        dam = min(dam, 18);
    }

    dam = min(dam, 100);

    dam = max(dam, 0);

    victim.points.borrow_mut().hit -= dam;

    if ch != victim {
        gain_exp(&ch, victim.get_level() as i32 * dam as i32);
    }

    update_pos(&victim);

    if attacktype >= TYPE_HIT && attacktype <= TYPE_SLASH {
        if ch.equipment[EquipmentPosition::Wield].is_none() {
            dam_message(dam, Rc::as_ref(&ch), Rc::as_ref(&victim), TYPE_HIT);
        } else {
            dam_message(dam, Rc::as_ref(&ch), Rc::as_ref(&victim), attacktype);
        }
    } else {
        match game.fight_messages.get(&attacktype) {
            Some(messages) => {
                let nr = dice(1, messages.len() as u32);
                let message = messages.get(nr as usize);
                if !victim.is_npc() && victim.get_level() > 20 {
                    send_msg(&message.unwrap().god_msg, Rc::as_ref(&ch),
                        Rc::as_ref(&victim));
                }
            },
            None => (),
        }
    }
    match victim.get_pos() {
        Position::MortallyW => {
            act("$n is mortally wounded, and will die soon, if not aided.",
                true, &victim, None, None, None, None, VictimType::ToRoom);
            act("You are mortally wounded, and will die soon, if not aided.",
                false, &victim, None, None, None, None, VictimType::ToChar);
        },
        Position::Incap => {
            act("$n is incapacitated and will slowly die, if not aided.", true, &victim, None, None, None, None, VictimType::ToRoom);
			act("You are incapacitated an will slowly die, if not aided.", false, &victim, None, None, None, None, VictimType::ToChar);
        },
        Position::Stunned => {
            act("$n is stunned, but will probably regain conscience again.", true, &victim, None, None, None, None, VictimType::ToRoom);
			act("You're stunned, but will probably regain conscience again.", false, &victim, None, None, None, None, VictimType::ToChar);
        },
        Position::Dead => {
            act("$n is dead! R.I.P.", false, &victim, None, None, None, None, VictimType::ToRoom);
			act("You are dead!  Sorry...", false, &victim, None, None, None, None, VictimType::ToChar);
        },
        _ => {
            let max_hit = hit_limit(&victim);

            if dam > max_hit / 5 {
                act("That Really did HURT!", false, &victim, None, None, None, None, VictimType::ToChar);
            }

            if victim.get_hit() < max_hit / 5 {
                act("You wish that your wounds would stop BLEEDING that much!", false, &victim, None, None, None, None, VictimType::ToChar);
                if victim.is_npc() {
                    if victim.specials.borrow().act.contains(SpecialActFlags::ACT_WIMPY) {
                        do_flee(victim, "", 0, game);
                    }
                }
            }
        }
    }
}

pub fn hit(ch: Rc<CharData>, victim: Rc<CharData>, attacktype: i32, game: &Game) {
    let mut wielded = None;

    if ch.in_room != victim.in_room {
        log("NOT SAME ROOM WHEN FIGHTING!");
        return;
    }

    // held is unused -sproctor
    //if ch.equipment[EquipmentPosition::Hold].is_some() {
    //    held = ch.equipment[EquipmentPosition::Hold].unwrap();
    //}

    let w_type = if ch.equipment[EquipmentPosition::Wield].is_some() &&
            ch.equipment[EquipmentPosition::Wield].as_ref().unwrap().obj_flags.type_flag == ItemType::Weapon {
        wielded = ch.equipment[EquipmentPosition::Wield].as_ref();
        match wielded.unwrap().obj_flags.value[3] {
            0 | 1 | 2 => TYPE_WHIP,
            3 => TYPE_SLASH,
            4 | 5 | 6 => TYPE_CRUSH,
            7 => TYPE_BLUDGEON,
            8 | 9 | 10 | 11 => TYPE_PIERCE,
            _ => TYPE_HIT,
        }
    } else {
        if ch.is_npc() && ch.specials.borrow().attack_type >= TYPE_HIT {
            ch.specials.borrow().attack_type
        } else {
            TYPE_HIT
        }
    };

    // Calculate the raw armor including magic armor
    // The lower AC, the better

    let mut calc_thaco = if !ch.is_npc() {
        THACO[ch.get_class() as usize - 1][ch.get_level() as usize]
    } else {
        20
    };

    calc_thaco -= STR_APP[ch.strength_apply_index()].tohit;
    calc_thaco -= ch.get_hitroll() as i16;

    let diceroll = number(1, 20);

    let mut victim_ac = victim.get_ac() / 10;

    if victim.awake() {
        victim_ac += DEX_APP[victim.get_dex() as usize].defensive;
    }

    victim_ac = max(-10, victim_ac);

    if diceroll < 20 && victim.awake() &&
            (diceroll == 1 || calc_thaco - diceroll as i16 > victim_ac) {
        if attacktype == SKILL_BACKSTAB {
            damage(Rc::clone(&ch), victim, 0, SKILL_BACKSTAB, game);
        } else {
            damage(Rc::clone(&ch), victim, 0, w_type, game);
        }
    } else {
        let mut dam = STR_APP[ch.strength_apply_index()].todam;
        dam += ch.get_damroll() as i16;

        if !wielded.is_some() {
            if ch.is_npc() {
                dam += dice(ch.specials.borrow().damnodice as u32,
                    ch.specials.borrow().damsizedice as u32) as i16;
            } else {
                dam += number(0, 2) as i16; // Max. 2 dam with bare hands
            }
        } else {
            dam += dice(wielded.unwrap().obj_flags.value[1] as u32,
                wielded.unwrap().obj_flags.value[2] as u32) as i16;
        }

        if victim.get_pos() < Position::Fighting {
            dam *= 1 + (Position::Fighting - victim.get_pos()) as i16 / 3;
        }
        /* Position  sitting  x 1.33 */
		/* Position  resting  x 1.66 */
		/* Position  sleeping x 2.00 */
		/* Position  stunned  x 2.33 */
		/* Position  incap    x 2.66 */
		/* Position  mortally x 3.00 */

        dam = max(1, dam);

        if attacktype == SKILL_BACKSTAB {
            dam *= BACKSTAB_MULT[ch.get_level() as usize] as i16;
            damage(Rc::clone(&ch), victim, dam, SKILL_BACKSTAB, game);
        } else {
            damage(Rc::clone(&ch), victim, dam, w_type, game);
        }
    }
}