use std::cell::RefCell;
use std::collections::{HashMap, LinkedList, VecDeque};
use std::fs::File;
use std::ops::Sub;
use std::rc::Rc;
use std::time::{Duration, SystemTime};
use std::vec::Vec;

use chan::Receiver;
use chan_signal::Signal;
use enum_map::EnumMap;
use libc::{c_int, time_t};

use diku::constants;

// The following definitions are for ObjData

// for 'type_flag'
#[derive(Clone, Copy, Eq, PartialEq)]
pub enum ItemType {
    Light,
    Scroll,
    Wand,
    Staff,
    Weapon,
    FireWeapon,
    Missile,
    Treasure,
    Armor,
    Potion,
    Worn,
    Other,
    Trash,
    Container,
    Note,
    DrinkCon,
    Key,
    Food,
    Money,
    Pen,
    Boat,
}

// for 'wear_flags'
bitflags! {
    pub struct WearFlags: u16 {
        const ITEM_TAKE         = 0b0000000000000001;
        const ITEM_WEAR_FINGER  = 0b0000000000000010;
        const ITEM_WEAR_NECK    = 0b0000000000000100;
        const ITEM_WEAR_BODY    = 0b0000000000001000;
        const ITEM_WEAR_HEAD    = 0b0000000000010000;
        const ITEM_WEAR_LEGS    = 0b0000000000100000;
        const ITEM_WEAR_FEET    = 0b0000000001000000;
        const ITEM_WEAR_HANDS   = 0b0000000010000000;
        const ITEM_WEAR_ARMS    = 0b0000000100000000;
        const ITEM_WEAR_SHIELD  = 0b0000001000000000;
        const ITEM_WEAR_ABOUT   = 0b0000010000000000;
        const ITEM_WEAR_WAISTE  = 0b0000100000000000;
        const ITEM_WIELD        = 0b0001000000000000;
        const ITEM_HOLD         = 0b0010000000000000;
        const ITEM_THROW        = 0b0100000000000000;
    }
}

// for 'extra_flags'
bitflags! {
    pub struct ItemExtraFlags : u16 {
        const ITEM_GLOW         = 0b0000000000000001;
        const ITEM_HUM          = 0b0000000000000010;
        const ITEM_DARK         = 0b0000000000000100;
        const ITEM_LOCK         = 0b0000000000001000;
        const ITEM_EVIL         = 0b0000000000010000;
        const ITEM_INVISIBLE    = 0b0000000000100000;
        const ITEM_MAGIC        = 0b0000000001000000;
        const ITEM_NODROP       = 0b0000000010000000;
        const ITEM_BLESS        = 0b0000000100000000;
        const ITEM_ANTI_GOOD    = 0b0000001000000000;
        const ITEM_ANTI_EVIL    = 0b0000010000000000;
        const ITEM_ANTI_NEUTRAL = 0b0000100000000000;
    }
}

// Some different kind of liquids
pub enum Liquid {
    Water,
    Beer,
    Wine,
    Ale,
    DarkAle,
    Whiskey,
    Lemonade,
    FireBrt,
    LocalSpc,
    Slime,
    Milk,
    Tea,
    Coffee,
    Blood,
    SaltWater,
    Coke,
}

// for containers - value[1]
bitflags! {
    pub struct ContainerFlags : u8 {
        const CONT_CLOSEABLE    = 0b00000001;
        const CONT_PICKPROOF    = 0b00000010;
        const CONT_CLOSED       = 0b00000100;
        const CONT_LOCKED       = 0b00001000;
    }
}

#[derive(Eq, PartialEq)]
pub struct ExtraDescrData {
    pub keyword: String,        // Keyword in look/examine
    pub description: String,    // What to see
}

#[derive(Eq, PartialEq)]
pub struct ObjFlagData {
    pub value:              [i32; 4],
    pub type_flag:          ItemType,           // Type of item
    pub wear_flags:         WearFlags,          // Where you can wear it
    pub extra_flags:        ItemExtraFlags,     // If it hums, glows, etc
    pub weight:             i32,                // Derr...
    pub cost:               u32,                // Value when sold (gp.)
    pub cost_per_day:       u32,                // Cost to keep pr. real day
    pub timer:              u32,                // Timer for object
    pub bitvector:          u64,                // To set chars bits
}

#[derive(Eq, PartialEq)]
pub struct ObjAffectedType {
    location: u8,   // Which ability to change (APPLY_XXX)
    modifier: u16,  // How much it changes by
}

#[derive(Eq, PartialEq)]
pub struct ObjData {
    pub item_number:        u32,                    // Where in database
    pub in_room:            Option<Rc<RoomData>>,   // In what room. None when conta/carr
    pub obj_flags:          ObjFlagData,            // Object information
    pub affected:           [ObjAffectedType; constants::MAX_OBJ_AFFECT],  // Which abilities in PC to change
    pub name:               String,                 // Title of object :get etc
    pub description:        String,                 // When in room
    pub short_description:  String,                 // When worn/carry/in cont.
    pub action_description: String,                 // What to write when used
    pub ex_description:     Vec<ExtraDescrData>,    // extra descriptions
    pub carried_by:         Option<Rc<RefCell<CharData>>>,   // Carried by. None in room/conta
    pub in_obj:             Option<Rc<RefCell<ObjData>>>,    // In what object. None when none
    pub contains:           Vec<Rc<RefCell<ObjData>>>,       // Contains objects
}

// For 'room_flags'
bitflags! {
    pub struct RoomFlags: u16 {
        const DARK      = 0b0000000000000001;
        const DEATH     = 0b0000000000000010;
        const NO_MOB    = 0b0000000000000100;
        const INDOORS   = 0b0000000000001000;
        const LAWFUL    = 0b0000000000010000;
        const NEUTRAL   = 0b0000000000100000;
        const CHAOTIC   = 0b0000000001000000;
        const NO_MAGIC  = 0b0000000010000000;
        const TUNNEL    = 0b0000000100000000;
        const PRIVATE   = 0b0000001000000000;
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub enum Direction {
    North = 0,
    East,
    South,
    West,
    Up,
    Down,
}

impl From<u8> for Direction {
    fn from(n: u8) -> Direction {
        match n {
            0 => Direction::North,
            1 => Direction::East,
            2 => Direction::South,
            3 => Direction::West,
            4 => Direction::Up,
            5 => Direction::Down,
            _ => panic!("Invalid direction number {}", n),
        }
    }
}

bitflags! {
    pub struct ExitFlags: u8 {
        const EX_ISDOOR     = 0b00000001;
        const EX_CLOSED     = 0b00000010;
        const EX_LOCKED     = 0b00000100;
        const EX_RSCLOSED   = 0b00001000;
        const EX_RSLOCKED   = 0b00010000;
        const EX_PICKPROOF  = 0b00100000;
    }
}

#[derive(Clone, Copy, EnumMap, Eq, PartialEq)]
pub enum SectorType {
    Inside,
    City,
    Field,
    Forest,
    Hills,
    Mountain,
    WaterSwim,
    WaterNoSwim,
}

impl From<u8> for SectorType {
    fn from(n: u8) -> SectorType {
        match n {
            0 => SectorType::Inside,
            1 => SectorType::City,
            2 => SectorType::Field,
            3 => SectorType::Forest,
            4 => SectorType::Hills,
            5 => SectorType::Mountain,
            6 => SectorType::WaterSwim,
            7 => SectorType::WaterNoSwim,
            _ => panic!("Invalid sector number {}", n),
        }
    }
}

#[derive(Eq, PartialEq)]
pub struct RoomDirectionData {
    pub general_description:    String,     // When look DIR.
    pub keyword:                String,     // for open/close
    pub exit_info:              ExitFlags,  // Exit info
    pub key:                    Option<u32>,        // Key's number (-1 for no key)
    pub to_room:                Option<u32>,        // Where direction leads (NOWHERE)
}

pub struct RoomData {
    pub number:         u32,                // Rooms number
    pub zone:           u16,                // Room zone (for resetting)
    pub sector_type:    SectorType,         // sector type (move/hide)
    pub name:           String,             // Rooms name 'You are ...'
    pub description:    String,             // Shown when entered
    pub ex_description: Vec<ExtraDescrData>,    // for examine/look
    pub dir_option:     HashMap<Direction, Rc<RoomDirectionData>>, // Directions
    pub room_flags:     RoomFlags,          // DEATH, DARK, etc
    pub light:          RefCell<u8>,                 // Number of lightsources in room
    pub funct:          Option<SpecialProcedure>,    // special procedure
    pub contents:       RefCell<Vec<Rc<ObjData>>>,   // List of items in room
    pub people:         RefCell<Vec<Rc<CharData>>>,  // List of NPC / PC in room
}

impl Eq for RoomData {}

impl PartialEq for RoomData {
    fn eq(&self, other: &Self) -> bool {
        self.number == other.number
    }
}

// The following defs and structures are related to CharData

// for 'equipment'
#[derive(Clone, Copy, EnumMap)]
pub enum EquipmentPosition {
    Light,
    FingerR,
    FingerL,
    Neck1,
    Neck2,
    Body,
    Head,
    Legs,
    Feet,
    Hands,
    Arms,
    Shield,
    About,
    Waiste,
    WristR,
    WristL,
    Wield,
    Hold,
}

// Predefined conditions
#[derive(EnumMap)]
pub enum Condition {
    Drunk,
    Full,
    Thirst,
}

// Bitvector for 'affected_by'
bitflags! {
    #[derive(Serialize, Deserialize)]
    pub struct AffectedFlags: u32 {
        const AFF_BLIND             = 1 << 0;
        const AFF_INVISIBLE         = 1 << 1;
        const AFF_DETECT_EVIL       = 1 << 2;
        const AFF_DETECT_INVISIBLE  = 1 << 3;
        const AFF_DETECT_MAGIC      = 1 << 4;
        const AFF_SENSE_LIFE        = 1 << 5;
        const AFF_HOLD              = 1 << 6;
        const AFF_SANCTUARY         = 1 << 7;
        const AFF_GROUP             = 1 << 8;
        const AFF_CURSE             = 1 << 9;
        const AFF_FLAMING           = 1 << 10;
        const AFF_POISON            = 1 << 11;
        const AFF_PROTECT_EVIL      = 1 << 12;
        const AFF_PARALYSIS         = 1 << 13;
        const AFF_MORDEN_SWORD      = 1 << 14;
        const AFF_FLAMING_SWORD     = 1 << 15;
        const AFF_SLEEP             = 1 << 16;
        const AFF_DODGE             = 1 << 17;
        const AFF_SNEAK             = 1 << 18;
        const AFF_HIDE              = 1 << 19;
        const AFF_FEAR              = 1 << 20;
        const AFF_CHARM             = 1 << 21;
        const AFF_FOLLOW            = 1 << 22;
    }
}

#[derive(Clone, Copy, Serialize, Deserialize, Eq, PartialEq, Debug)]
pub enum AbilityModifier {
    None,
    Str,
    Dex,
    Int,
    Wis,
    Con,
    Sex,
    Class,
    Level,
    Age,
    CharWeight,
    CharHeight,
    Mana,
    Hit,
    Move,
    Gold,
    Exp,
    Ac,
    Hitroll,
    Damroll,
    SavingPara,
    SavingRod,
    SavingPetri,
    SavingBreath,
    SavingSpell,
}

#[derive(EnumMap)]
pub enum SavingThrowModifier {
    Para,
    Rod,
    Petri,
    Breath,
    Spell,
}

// 'class' for PC's
#[derive(Clone, Copy, Eq, PartialEq)]
pub enum Class {
    MagicUser = 1,
    Cleric,
    Thief,
    Warrior,
}

// sex
#[derive(Clone, Copy, Eq, PartialEq)]
pub enum Sex {
    Neutral,
    Male,
    Female,
}

// positions
#[derive(Clone, Copy, Eq, Ord, PartialEq, PartialOrd)]
pub enum Position {
    Dead = 0,
    MortallyW,
    Incap,
    Stunned,
    Sleeping,
    Resting,
    Sitting,
    Fighting,
    Standing,
}

impl Sub for Position {
    type Output = i8;

    fn sub(self, other: Position) -> i8 {
        self as i8 - other as i8
    }
}

// for specials.act
bitflags! {
    pub struct SpecialActFlags: u8 {
        const ACT_SPEC          = 0b00000001; // special routine to be called if exist
        const ACT_SENTINEL      = 0b00000010; // this mobile not to be moved
        const ACT_SCAVENGER     = 0b00000100; // pick up stuff lying around
        const ACT_ISNPC         = 0b00001000; // This bit is set for use with IS_NPC()
        const ACT_NICE_THIEF    = 0b00010000; // Set if a thief should NOT be killed
        const ACT_AGGRESSIVE    = 0b00100000; // Set if automatic attack on NPC's
        const ACT_STAY_ZONE     = 0b01000000; // MOB Must stay inside its own zone
        const ACT_WIMPY         = 0b10000000; // MOB will flee when injured, and if aggressive only attack sleeping players
        const PLR_BRIEF         = 0b00000001;
        const PLR_NOSHOUT       = 0b00000010;
        const PLR_COMPACT       = 0b00000100;
        // 0b00001000 intentally skipped. used for IS_NPC()
        const PLR_NOTELL        = 0b00010000;
        const PLR_NOEMOTE       = 0b00100000;
        const PLR_LOG           = 0b01000000; // log activities of this player
        const PLR_FREEZE        = 0b10000000; // No commands available
    }
}

pub struct TimeInfoData {
    pub hours:  u8,
    pub day:    u8,
    pub month:  u8,
    pub year:   u16,
}

#[derive(Eq, PartialEq)]
pub struct TimeData {
    pub birth:  SystemTime, // This represents the characters age
    pub logon:  SystemTime, // Time of last logon (used to calculate played)
    pub played: Duration,   // This is the total accumulated time played in secs
}

#[derive(Eq, PartialEq)]
pub struct CharPlayerData {
    pub name:           String, // PC / NPC s name (kill ... )
    pub short_descr:    String, // for 'actions'
    pub long_descr:     String, // for 'look'.. Only here for testing
    pub description:    String, // Extra descriptions
    pub title:          &'static str, // PC / NPC s title
    pub sex:            Sex,    // PC / NPC s sex
    pub class:          Class,  // PC s class
    pub level:          u8,     // PC / NPC s level
    pub hometown:       u16,    // PC s Hometown (zone)
    pub talks:          [bool; constants::MAX_TOUNGE], // PC s Tounges 0 for NPC
    pub time:           TimeData,   // PC s AGE in days
    pub tmptime:        TimeData,   // PC s AGE in days, modified
    pub weight:         u8,     // PC / NPC s weight
    pub height:         u8,     // PC / NPC s height
}

// used in CHAR_FILE_U *DO*NOT*CHANGE*
#[derive(Serialize, Deserialize, Eq, PartialEq, Debug)]
pub struct CharAbilityData {
    pub str:        u8,
    pub str_add:    u8,     // 000 - 100 if strength 18
    pub intel:      u8,
    pub wis:        u8,
    pub dex:        u8,
    pub con:        u8,
}

// Used in CHAR_FILE_U DO NOT CHANGE
#[derive(Serialize, Deserialize, Eq, PartialEq, Debug)]
pub struct CharPointData {
    pub mana:       i16,
    pub max_mana:   i16, // Not useable may be erased upon player file renewal
    pub hit:        i16,
    pub max_hit:    i16,    // Max hit for NPC
    pub mov:        i16,
    pub max_mov:    i16,    // Max move for NPC
    pub armor:      i16,    // internal -100..100, external -10..10 AC
    pub gold:       i32,    // Money carried
    pub exp:        i32,    // The experience of the player
    pub hitroll:    i8,     // Any bonus or penalty to the hit roll
    pub damroll:    i8,     // Any bonus or penalty to the damage roll
}

#[derive(Eq, PartialEq)]
pub struct CharSpecialData {
    pub fighting:           Option<Rc<CharData>>,   // Opponent
    pub hunting:            Option<Rc<CharData>>,   // Hunting person..
    pub affected_by:        AffectedFlags,      // bitvector for spells/skills affected by
    pub position:           Position,           // Standing or ...
    pub default_pos:        Position,           // Default position for NPC
    pub act:                SpecialActFlags,    // flags for NPC behavior
    pub spells_to_learn:    u8,                 // How many can you learn yet this level
    pub carry_weight:       i32,                // Carried weight
    pub carry_items:        u8,                 // Number of items carried
    pub timer:              i32,                // Timer for update
    pub was_in_room:        u32,                // storage of location for linkdead people
    pub apply_saving_throw: EnumMap<SavingThrowModifier, i16>,
    pub conditions:         EnumMap<Condition, i8>,
    pub damnodice:          i8,                 // The number of damage dice's
    pub damsizedice:        i8,                 // The size of te damage dice's
    pub last_direction:     Direction,          // The last direction the monster went
    pub attack_type:        i32,                // The Attack Type Bitvector for NPC's
    // Note from sproctor: `attack_type` seems to be unused. It is definitely not a bitvector
    pub alignment:          i32,                // +-1000 for alignments
}

// Used in CHAR_FILE_U *DO*NOT*CHANGE*
#[derive(Serialize, Deserialize, Eq, PartialEq, Debug)]
pub struct CharSkillData {
    learned:    i8,
    recognise:  bool,
}

// Used in CHAR_FILE_U *DO*NOT*CHANGE*
#[derive(Clone, Serialize, Deserialize, Eq, PartialEq, Debug)]
pub struct AffectedType {
    pub spell_type:     i32,     // The type of spell that caused this
    pub duration:       i16,    // For how long its effects will last
    pub modifier:       i8,     // This is added to appropriate ability
    pub location:       AbilityModifier,    // Tells which ability to change(APPLY_XXX)
    pub bitvector:      AffectedFlags,      // Tells which bits to set (AFF_XXX)
}

// ================== Structure for player/non-player =====================
#[derive(Eq, PartialEq)]
pub struct CharData {
    pub nr:             Option<u32>,        // monster nr (pos in file)
    pub in_room:        Rc<RoomData>,                // Location
    pub player:         RefCell<CharPlayerData>,     // Normal data
    pub abilities:      CharAbilityData,    // Abilities
    pub tmpabilities:   RefCell<CharAbilityData>,    // The abilities we use
    pub points:         RefCell<CharPointData>,      // Points
    pub specials:       RefCell<CharSpecialData>,    // Special plaing constants
    pub skills:         Vec<CharSkillData>, // Skills

    pub affected:       LinkedList<AffectedType>,  // affected by what spells
    pub equipment:      EnumMap<EquipmentPosition, Option<Rc<ObjData>>>, // Equipment array

    pub carrying:       Vec<Rc<ObjData>>,
    pub desc:           Option<RefCell<DescriptorData>>, // None for mobiles

    pub followers:      RefCell<Vec<Rc<CharData>>>,  // List of char followers
    pub master:         RefCell<Option<Rc<CharData>>>,   // Who is char following?
}

/* ======================================================================== */

// How much light is in the land ?
pub enum Sunlight {
    Dark,
    Sunrise,
    Light,
    Sunset,
}

// And how is the sky ?
pub enum Sky {
    Cloudless,
    Cloudy,
    Raining,
    Lightning,
}

pub struct WeatherData {
    pub pressure:   i32,    // How is the pressure ( Mb )
    pub change:     i32,    // How fast and what way does it change
    pub sky:        Sky,    // How is the sky.
    pub sunlight:   Sunlight,   // And how much sun.
}

// **************************************************************************
// *  file element for player file. BEWARE: Changing it will ruin the file  *
// **************************************************************************

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct CharFileU {
    pub sex:            i8,
    pub class:          i8,
    pub level:          i8,
    pub birth:          time_t,     // Time of birth of character
    pub played:         i32,        // Number of secs played in total
    pub weight:         u8,
    pub height:         u8,
    pub title:          [[u8; 20]; 4],
    pub hometown:       i16,
    pub description:    [[u8; 24]; 10],
    pub talks:          [bool; constants::MAX_TOUNGE],
    pub load_room:      u32,        // Which room to place char in
    pub abilities:      CharAbilityData,
    pub points:         CharPointData,
    pub skills:         ([CharSkillData; 32], [CharSkillData; constants::MAX_SKILLS - 32]),
    pub affected:       [AffectedType; constants::MAX_AFFECT],

    // specials

    pub spells_to_learn:    i8,
    pub alignmen:           i32,
    pub last_logon:         i64,    // Time (in secs) of last logon
    pub act:                u8,     // ACT Flags

    // char data
    pub name:               [u8; 20],
    pub pwd:                [u8; 11],
    pub apply_saving_throw: [i16; 5],
    pub conditions:         [i32; 3],
}

// **************************************************************************
// *  file element for object file. BEWARE: Changing it will ruin the file  *
// **************************************************************************

pub struct ObjFileElem {
    item_number:    i16,
    value:          [i32; 4],
    extra_flags:    i32,
    weight:         i32,
    timer:          i32,
    bitvector:      i32,
    affected:       [ObjAffectedType; constants::MAX_OBJ_AFFECT],
}

pub struct ObjFileU {
    owner:          [u8; 20],   // Name of player
    gold_left:      i32,        // Number of goldcoins left at owner
    total_cost:     i32,        // The cost for all items, per day
    last_update:    i64,        // Time in seconds, when last updated
    objects:        [ObjFileElem; constants::MAX_OBJ_SAVE],
}

// **************************************************************
// *  The following structures are related to descriptor_data   *
// **************************************************************

#[derive(Eq, PartialEq)]
pub enum ConnectionMode {
    Plying,
    Nme,
    NmeCnf,
    PwdNrm,
    PwdGet,
    PwdCnf,
    QSex,
    RMOTD,
    Slct,
    ExDscr,
    QClass,
    LDead,
    PwdNew,
    PwdNCnf,
}

#[derive(Eq, PartialEq)]
pub struct SnoopData {
    snooping:   Rc<RefCell<CharData>>,
    snoop_by:   Rc<RefCell<CharData>>,
}

#[derive(Eq, PartialEq)]
pub struct DescriptorData {
    pub descriptor:     c_int,
    pub host:           String,
    pub pwd:            String,
    pub pos:            i32,
    pub connected:      ConnectionMode,
    pub wait:           i32,
    // showstr_head
    // showstr_point
    // str
    // max_str
    pub prompt_mode:    i32,
    pub buf:            Vec<u8>,
    pub last_input:     Vec<u8>,
    pub output:         VecDeque<String>,   // q of strings to send
    pub input:          VecDeque<String>,   // q of unprocessed input
    pub character:      Rc<RefCell<CharData>>,       // linked to char
    //original          // original char
    pub snoop:          SnoopData,          // to snoop people
}

pub struct MsgType {
    pub attacker_msg:   String, // message to attacker
    pub victim_msg:     String, // message to victim
    pub room_msg:       String, // message to room
}

pub struct MessageType {
    pub die_msg:        MsgType,    // messages when death
    pub miss_msg:       MsgType,    // messages when miss
    pub hit_msg:        MsgType,    // messages when hit
    // sanctuary_msg:  MsgType,    // messages when hit on sanctuary (unused -sproctor)
    pub god_msg:        MsgType,    // messages when hit on god
}

//pub struct MessageList {
//    pub a_type:             i32,                // Attack type
//    pub number_of_attacks:  i32,                // How many attack messages to chose from
//    pub msg:                Vec<MessageType>,   // List of messages
//}

pub struct DexSkillType {
    p_pocket:   i16,
    p_locks:    i16,
    traps:      i16,
    sneak:      i16,
    hide:       i16,
}

pub struct DexAppType {
    pub reaction:   i16,
    pub miss_att:   i16,
    pub defensive:  i16,
}

pub struct StrAppType {
    pub tohit:      i16,
    pub todam:      i16,
    pub carry_w:    i32,
    pub wield_w:    i16,
}

pub struct WisAppType {
    bonus:  i8, // How many bonus skills a player can practice per level
}

pub struct IntAppType {
    learn:  i8, // how many % a player learns a spell/skill
}

pub struct ConAppType {
    pub hitp:   i16,
    pub shock:  i16,
}

pub enum MessageTarget {
    ToRoom,
    ToVict,
    ToNotVict,
    ToChar,
}

pub enum ResetMode {
    DoNot,  // Don't reset, and don't update age.
    NoPC,   // Reset if no PC's are located in zone.
    Do,     // Just reset.
}

pub struct ResetCom {
    pub command:    u8,     // current command
    pub if_flag:    bool,   // if TRUE: exe only if preceding exe'd
    pub arg1:       i32,    //
    pub arg2:       i32,    // Arguments to the command
    pub arg3:       i32,    //

    // Commands:
    // 'M': Read a mobile
    // 'O': Read an object
    // 'G': Give obj to mob
    // 'P': Put obj in obj
    // 'G': Obj to char (What?!? 'G' is above -Sean)
    // 'E': Obj to char equip
    // 'D': Set state of door
}

pub struct ZoneData {
    pub name:       String,         // name of this zone
    pub lifespan:   u32,            // how long between resets (minutes)
    pub age:        u32,            // current age of this zone (minutes)
    pub top:        u32,            // upper limit for rooms in this zone

    pub reset_mode: ResetMode,      // conditions for reset
    pub cmd:        Vec<ResetCom>,  // command table for reset
}

// element in monster and object index-tables
pub struct IndexData {
    //pub virtual_nr: u32,
    pub pos:        u64,
    pub number:     u32,
    pub func:       Option<SpecialProcedure>,
}

pub struct SocialMessg {
    pub act_nr:                 i32,
    pub hide:                   i32,
    pub min_victim_position:    i32,    // Position of victim

    // No argument was supplied
    pub char_no_arg:            Option<String>,
    pub others_no_arg:          Option<String>,

    // An argument was there, and a victim was found
    pub char_found:             Option<String>, // if NULL, read no further, ignore args
    pub others_found:           Option<String>,
    pub vict_found:             Option<String>,

    // An argument was there, but no victim was found
    pub not_found:              Option<String>,

    // The victim turned out to be the character
    pub char_auto:              Option<String>,
    pub others_auto:            Option<String>,
}

pub struct PoseType {
    pub level:      i32,    // minimum level for poser
    pub poser_msg:  [String; 4],
    pub room_msg:   [String; 4],
}

pub struct TitleType {
    pub title_m: &'static str,
    pub title_f: &'static str,
    pub exp: i32,
}

#[derive(Clone, Copy, PartialEq)]
pub enum VictimType {
    ToRoom,
    ToVict,
    ToNotVict,
    ToChar,
}

// Attacktypes with grammar
pub struct AttackHitType {
    pub singular:   &'static str,
    pub plural:     &'static str,
}

pub struct DamWeaponType {
    pub to_room:    &'static str,
    pub to_char:    &'static str,
    pub to_victim:  &'static str,
}

pub type RoomTable = HashMap<u32, RoomData>;
pub type ZoneTable = Vec<ZoneData>;
pub type FilePosTable = HashMap<String, u64>;
pub type IndexTable = HashMap<u32, IndexData>;
pub type SpecialProcedure = fn(Rc<CharData>, i32, &str, &Game) -> bool;

pub struct Game {
    pub descriptor_list:    Vec<DescriptorData>,
    pub lawful:             bool,
    pub wizlock:            bool,
    pub slow_death:         bool,
    pub shutdown:           bool,
    pub reboot:             bool,
    pub no_specials:        bool,
    pub weather_info:       WeatherData,
    pub news:               String,
    pub credits:            String,
    pub motd:               String,
    pub help:               String,
    pub info:               String,
    pub wizlist:            String,
    pub mob_f:              File,
    pub obj_f:              File,
    pub help_f:             Option<File>,
    pub help_index:         FilePosTable,
    pub mob_index:          IndexTable,
    pub obj_index:          IndexTable,
    pub player_table:       FilePosTable,
    pub zone_table:         ZoneTable,
    pub world:              RoomTable,
    pub combat_list:        RefCell<Vec<Rc<CharData>>>,
    pub fight_messages:     HashMap<i32, Vec<MessageType>>,
    pub soc_mess_list:      Vec<SocialMessg>,
    pub pose_messages:      Vec<PoseType>,
    pub shutdown_signal:    Receiver<Signal>,
    pub hup_signal:         Receiver<Signal>,
    pub log_signal:         Receiver<Signal>,
}