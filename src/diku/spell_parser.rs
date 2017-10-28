use std::rc::Rc;

use diku::comm::act;
use diku::handler::{affect_from_char, affected_by_spell};
use diku::spells::*;
use diku::structs::*;

/* Called when stop following persons, or stopping charm */
/* This will NOT do if a character quits/dies!!          */
pub fn stop_follower(ch: &CharData)
{
	
	let mut master_opt = ch.master.borrow_mut();
	{
		let master = Rc::as_ref(&master_opt.as_ref().unwrap());

		if ch.is_affected(AffectedFlags::AFF_CHARM) {
			act("You realize that $N is a jerk!", false, ch, None, Some(master),
					None, None, VictimType::ToChar);
			act("$n realizes that $N is a jerk!", false, ch, None, Some(master),
					None, None, VictimType::ToNotVict);
			act("$n hates your guts!", false, ch, None, Some(master), None, None,
					VictimType::ToVict);
			if affected_by_spell(ch, SPELL_CHARM_PERSON) {
				affect_from_char(ch, SPELL_CHARM_PERSON);
			}
		} else {
			act("You stop following $N.", false, ch, None, Some(master), None, None,
					VictimType::ToChar);
			act("$n stops following $N.", false, ch, None, Some(master), None, None,
					VictimType::ToNotVict);
			act("$n stops following you.", false, ch, None, Some(master), None, None,
					VictimType::ToNotVict);
		}

		master.followers.borrow_mut().retain(|k| k.as_ref() != ch);
	}

	*master_opt = None;
	ch.specials.borrow_mut().affected_by.remove(AffectedFlags::AFF_CHARM | AffectedFlags::AFF_GROUP);
}