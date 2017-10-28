
pub const TYPE_UNDEFINED:           i32 =  -1;
pub const SPELL_RESERVED_DBC:       i32 =   0;  /* SKILL NUMBER ZERO */
pub const SPELL_ARMOR:              i32 =   1; /* Reserved Skill[] DO NOT CHANGE */
pub const SPELL_TELEPORT:           i32 =   2; /* Reserved Skill[] DO NOT CHANGE */
pub const SPELL_BLESS:              i32 =   3; /* Reserved Skill[] DO NOT CHANGE */
pub const SPELL_BLINDNESS:          i32 =   4; /* Reserved Skill[] DO NOT CHANGE */
pub const SPELL_BURNING_HANDS:      i32 =   5; /* Reserved Skill[] DO NOT CHANGE */
pub const SPELL_CALL_LIGHTNING:     i32 =   6; /* Reserved Skill[] DO NOT CHANGE */
pub const SPELL_CHARM_PERSON:       i32 =   7; /* Reserved Skill[] DO NOT CHANGE */
pub const SPELL_CHILL_TOUCH:        i32 =   8; /* Reserved Skill[] DO NOT CHANGE */
pub const SPELL_CLONE:              i32 =   9; /* Reserved Skill[] DO NOT CHANGE */
pub const SPELL_COLOUR_SPRAY:       i32 =  10; /* Reserved Skill[] DO NOT CHANGE */
pub const SPELL_CONTROL_WEATHER:    i32 =  11; /* Reserved Skill[] DO NOT CHANGE */
pub const SPELL_CREATE_FOOD:        i32 =  12; /* Reserved Skill[] DO NOT CHANGE */
pub const SPELL_CREATE_WATER:       i32 =  13; /* Reserved Skill[] DO NOT CHANGE */
pub const SPELL_CURE_BLIND:         i32 =  14; /* Reserved Skill[] DO NOT CHANGE */
pub const SPELL_CURE_CRITIC:        i32 =  15; /* Reserved Skill[] DO NOT CHANGE */
pub const SPELL_CURE_LIGHT:         i32 =  16; /* Reserved Skill[] DO NOT CHANGE */
pub const SPELL_CURSE:              i32 =  17; /* Reserved Skill[] DO NOT CHANGE */
pub const SPELL_DETECT_EVIL:        i32 =  18; /* Reserved Skill[] DO NOT CHANGE */
pub const SPELL_DETECT_INVISIBLE:   i32 =  19; /* Reserved Skill[] DO NOT CHANGE */
pub const SPELL_DETECT_MAGIC:       i32 =  20; /* Reserved Skill[] DO NOT CHANGE */
pub const SPELL_DETECT_POISON:      i32 =  21; /* Reserved Skill[] DO NOT CHANGE */
pub const SPELL_DISPEL_EVIL:        i32 =  22; /* Reserved Skill[] DO NOT CHANGE */
pub const SPELL_EARTHQUAKE:         i32 =  23; /* Reserved Skill[] DO NOT CHANGE */
pub const SPELL_ENCHANT_WEAPON:     i32 =  24; /* Reserved Skill[] DO NOT CHANGE */
pub const SPELL_ENERGY_DRAIN:       i32 =  25; /* Reserved Skill[] DO NOT CHANGE */
pub const SPELL_FIREBALL:           i32 =  26; /* Reserved Skill[] DO NOT CHANGE */
pub const SPELL_HARM:               i32 =  27; /* Reserved Skill[] DO NOT CHANGE */
pub const SPELL_HEAL:               i32 =  28; /* Reserved Skill[] DO NOT CHANGE */
pub const SPELL_INVISIBLE:          i32 =  29; /* Reserved Skill[] DO NOT CHANGE */
pub const SPELL_LIGHTNING_BOLT:     i32 =  30; /* Reserved Skill[] DO NOT CHANGE */
pub const SPELL_LOCATE_OBJECT:      i32 =  31; /* Reserved Skill[] DO NOT CHANGE */
pub const SPELL_MAGIC_MISSILE:      i32 =  32; /* Reserved Skill[] DO NOT CHANGE */
pub const SPELL_POISON:             i32 =  33; /* Reserved Skill[] DO NOT CHANGE */
pub const SPELL_PROTECT_FROM_EVIL:  i32 =  34; /* Reserved Skill[] DO NOT CHANGE */
pub const SPELL_REMOVE_CURSE:       i32 =  35; /* Reserved Skill[] DO NOT CHANGE */
pub const SPELL_SANCTUARY:          i32 =  36; /* Reserved Skill[] DO NOT CHANGE */
pub const SPELL_SHOCKING_GRASP:     i32 =  37; /* Reserved Skill[] DO NOT CHANGE */
pub const SPELL_SLEEP:              i32 =  38; /* Reserved Skill[] DO NOT CHANGE */
pub const SPELL_STRENGTH:           i32 =  39; /* Reserved Skill[] DO NOT CHANGE */
pub const SPELL_SUMMON:             i32 =  40; /* Reserved Skill[] DO NOT CHANGE */
pub const SPELL_VENTRILOQUATE:      i32 =  41; /* Reserved Skill[] DO NOT CHANGE */
pub const SPELL_WORD_OF_RECALL:     i32 =  42; /* Reserved Skill[] DO NOT CHANGE */
pub const SPELL_REMOVE_POISON:      i32 =  43; /* Reserved Skill[] DO NOT CHANGE */
pub const SPELL_SENSE_LIFE:         i32 =  44; /* Reserved Skill[] DO NOT CHANGE */

/* types of attacks and skills must NOT use same numbers as spells! */

pub const SKILL_SNEAK:              i32 =  45; /* Reserved Skill[] DO NOT CHANGE */
pub const SKILL_HIDE:               i32 =  46; /* Reserved Skill[] DO NOT CHANGE */
pub const SKILL_STEAL:              i32 =  47; /* Reserved Skill[] DO NOT CHANGE */
pub const SKILL_BACKSTAB:           i32 =  48; /* Reserved Skill[] DO NOT CHANGE */
pub const SKILL_PICK_LOCK:          i32 =  49; /* Reserved Skill[] DO NOT CHANGE */

pub const SKILL_KICK:               i32 =  50; /* Reserved Skill[] DO NOT CHANGE */
pub const SKILL_BASH:               i32 =  51; /* Reserved Skill[] DO NOT CHANGE */
pub const SKILL_RESCUE:             i32 =  52; /* MAXIMUM SKILL NUMBER  */

/* END OF SKILL RESERVED "NO TOUCH" NUMBERS */


/* NEW SPELLS are to be inserted here */
pub const SPELL_IDENTIFY:           i32 =  53;
pub const SPELL_ANIMATE_DEAD:       i32 =  54; /* EXAMPLE */
pub const SPELL_FEAR:               i32 =  55; /* EXAMPLE */
pub const SPELL_FIRE_BREATH:        i32 =  56;
pub const SPELL_GAS_BREATH:         i32 =  57;
pub const SPELL_FROST_BREATH:       i32 =  58;
pub const SPELL_ACID_BREATH:        i32 =  59;
pub const SPELL_LIGHTNING_BREATH:   i32 =  60;


pub const TYPE_HIT:                 i32 = 100;
pub const TYPE_BLUDGEON:            i32 = 101;
pub const TYPE_PIERCE:              i32 = 102;
pub const TYPE_SLASH:               i32 = 103;
pub const TYPE_WHIP:                i32 = 104; // EXAMPLE
pub const TYPE_CLAW:                i32 = 105;  // NO MESSAGES WRITTEN YET!
pub const TYPE_BITE:                i32 = 106;  // NO MESSAGES WRITTEN YET!
pub const TYPE_STING:               i32 = 107;  // NO MESSAGES WRITTEN YET!
pub const TYPE_CRUSH:               i32 = 108;  // NO MESSAGES WRITTEN YET!


pub const TYPE_SUFFERING:           i32 = 200;
/* More anything but spells and weapontypes can be insterted here! */