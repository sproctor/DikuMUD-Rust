
use diku::constants::*;
use std::time::Duration;
use diku::structs::*;

pub fn fname<'a>(namelist: &'a str) -> &'a str {
    let mut point = 0;
    
    for c in namelist.chars() {
        if !c.is_alphabetic() {
            break;
        }
        point += 1;
    }

    namelist.get(0..point).unwrap()
}

fn isname(st: &str, namelist: &str) -> bool {
    let mut pos = 0;
    loop {
        for c in st.chars() {
            if pos >= namelist.len() {
                return false;
            }

            let curname = namelist.chars().nth(pos).unwrap();
            if curname == ' ' {
                break;
            }

            if c.to_lowercase().to_string() != curname.to_lowercase().to_string() {
                break;
            }

            pos += 1;
        }
        if !namelist.chars().nth(pos).unwrap().is_alphabetic() {
            return true;
        }
        while namelist.chars().nth(pos).unwrap().is_alphabetic() {
            pos += 1;
            if pos >= namelist.len() {
                return false;
            }
        }
        pos += 1;
    }
}

fn affect_modify(ch: &CharData, loc: AbilityModifier, modifier: i8, bitv: AffectedFlags, add: bool) {
    let mut modifier = modifier;
    if add {
        ch.specials.borrow_mut().affected_by.insert(bitv);
    } else {
        ch.specials.borrow_mut().affected_by.remove(bitv);
        modifier = -modifier;
    }

    match loc {
        AbilityModifier::None => (),
        AbilityModifier::Str => amod(&mut ch.tmpabilities.borrow_mut().str, modifier),
        AbilityModifier::Dex => amod(&mut ch.tmpabilities.borrow_mut().dex, modifier),
        AbilityModifier::Int => amod(&mut ch.tmpabilities.borrow_mut().intel, modifier),
        AbilityModifier::Wis => amod(&mut ch.tmpabilities.borrow_mut().wis, modifier),
        AbilityModifier::Con => amod(&mut ch.tmpabilities.borrow_mut().con, modifier),
        AbilityModifier::Sex => (),
        AbilityModifier::Class => (),
        AbilityModifier::Level => (),
        AbilityModifier::Age =>
            if modifier < 0 {
                ch.player.borrow_mut().time.birth -= Duration::new(SECS_PER_MUD_YEAR * (-modifier) as u64, 0);
            } else {
                ch.player.borrow_mut().time.birth += Duration::new(SECS_PER_MUD_YEAR * modifier as u64, 0);
            },
        AbilityModifier::CharWeight => amod(&mut ch.player.borrow_mut().weight, modifier),
        AbilityModifier::CharHeight => amod(&mut ch.player.borrow_mut().height, modifier),
        AbilityModifier::Mana => (),
        AbilityModifier::Hit => ch.points.borrow_mut().max_hit += modifier as i16,
        AbilityModifier::Move => (), // Will change nothing on playes ch->points.max_move += mod;
        AbilityModifier::Gold => (),
        AbilityModifier::Exp => (),
        AbilityModifier::Ac => ch.points.borrow_mut().armor += modifier as i16,
        AbilityModifier::Hitroll => ch.points.borrow_mut().hitroll += modifier,
        AbilityModifier::Damroll => ch.points.borrow_mut().damroll += modifier,
        AbilityModifier::SavingPara => ch.specials.borrow_mut().apply_saving_throw[SavingThrowModifier::Para] += modifier as i16,
        AbilityModifier::SavingRod => ch.specials.borrow_mut().apply_saving_throw[SavingThrowModifier::Rod] += modifier as i16,
        AbilityModifier::SavingPetri => ch.specials.borrow_mut().apply_saving_throw[SavingThrowModifier::Petri] += modifier as i16,
        AbilityModifier::SavingBreath => ch.specials.borrow_mut().apply_saving_throw[SavingThrowModifier::Breath] += modifier as i16,
        AbilityModifier::SavingSpell => ch.specials.borrow_mut().apply_saving_throw[SavingThrowModifier::Spell] += modifier as i16,
    }
}

pub fn amod(dst: &mut u8, modify: i8) {
    let result = *dst as i16 + modify as i16;
    assert!(result >= 0 && result <= 25);
    *dst = result as u8;
}

pub fn affect_from_char(ch: &CharData, skill: i32) {
    for hjp in &ch.affected {
        if hjp.spell_type == skill {
            affect_remove(ch, &hjp);
        }
    }
}

/* Return if a char is affected by a spell (SPELL_XXX), NULL indicates 
   not affected                                                        */
pub fn affected_by_spell(ch: &CharData, skill: i32) -> bool {
	for hjp in &ch.affected {
        if hjp.spell_type == skill {
            return true;
        }
    }
    return false;
}

fn affect_remove(ch: &CharData, af: &AffectedType) {
    affect_modify(ch, af.location, af.modifier, af.bitvector, false);
}