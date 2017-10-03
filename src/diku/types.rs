use std::collections::VecDeque;
use std::mem::transmute;
use std::rc::Rc;
use std::time::{Duration, SystemTime};
use std::vec::Vec;
use enum_map::EnumMap;
use libc::c_int;

use diku::constants;

// The following definitions are for ObjData

// for 'type_flag'
pub enum ItemType {
    Light,
    croll,
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

pub struct ExtraDescrData {
    keyword: String,        // Keyword in look/examine
    description: String,    // What to see
}

pub struct ObjFlagData {
    container_flags: ContainerFlags,
    type_flag: ItemType,                // Type of item
    wear_flags: WearFlags,              // Where you can wear it
    extra_flags: ItemExtraFlags,        // If it hums, glows, etc
    weight: u32,                        // Derr...
    cost: u32,                          // Value when sold (gp.)
    cost_per_day: u32,                  // Cost to keep pr. real day
    timer: u32,                         // Timer for object
    bitvector: u64,                     // To set chars bits
}

pub struct ObjAffectedType {
    location: u8,   // Which ability to change (APPLY_XXX)
    modifier: u16,  // How much it changes by
}

pub struct ObjData {
    item_number:    u16,            // Where in database
    in_room:        Option<u16>,    // In what room. None when conta/carr
    obj_flags:      ObjFlagData,    // Object information
    affected:       [ObjAffectedType; constants::MAX_OBJ_AFFECT],  // Which abilities in PC to change
    name:           String,         // Title of object :get etc
    description:    String,         // When in room
    short_description:  String,     // When worn/carry/in cont.
    action_description: String,     // What to write when used
    ex_description: Vec<ExtraDescrData>,    // extra descriptions
    carried_by:     Option<Rc<CharData>>,   // Carried by. None in room/conta
    in_obj:         Option<Rc<ObjData>>,    // In what object. None when none
    contains:       Vec<Rc<ObjData>>,       // Contains objects
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

pub enum Direction {
    North,
    East,
    South,
    West,
    Up,
    Down,
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
        assert!(SectorType::Inside as u8 <= n && n <= SectorType::WaterNoSwim as u8);
        unsafe { transmute(n) }
    }
}

pub struct RoomDirectionData {
    pub general_description:    String,     // When look DIR.
    pub keyword:                String,     // for open/close
    pub exit_info:              ExitFlags,  // Exit info
    pub key:                    i16,        // Key's number (-1 for no key)
    pub to_room:                i16,        // Where direction leads (NOWHERE)
}

pub struct RoomData {
    number:         i16,        // Rooms number
    zone:           i16,        // Room zone (for resetting)
    sector_type:    SectorType, // sector type (move/hide)
    name:           String,     // Rooms name 'You are ...'
    description:    String,     // Shown when entered
    ex_description: Vec<ExtraDescrData>,    // for examine/look
    dir_option:     [RoomDirectionData; 6], // Directions
    room_flags:     RoomFlags,  // DEATH, DARK, etc
    light:          u8,         // Number of lightsources in room
    funct:          fn(i32),    // special procedure
    contents:       Vec<Rc<ObjData>>,   // List of items in room
    people:         Vec<Rc<CharData>>,  // List of NPC / PC in room
}

// The following defs and structures are related to CharData

// for 'equipment'
#[derive(EnumMap)]
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
    Armor,
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
pub enum Class {
    MagicUser,
    Cleric,
    Thief,
    Warrior,
}

// sex
pub enum Sex {
    Neutral,
    Male,
    Female,
}

// positions
pub enum Position {
    Dead,
    MortallyW,
    Incap,
    Stunned,
    Sleeping,
    Resting,
    Sitting,
    Fighting,
    Standing,
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

pub struct TimeData {
    birth:  SystemTime, // This represents the characters age
    logon:  SystemTime, // Time of last logon (used to calculate played)
    played: Duration,   // This is the total accumulated time played in secs
}

pub struct CharPlayerData {
    name:           String, // PC / NPC s name (kill ... )
    short_descr:    String, // for 'actions'
    long_descr:     String, // for 'look'.. Only here for testing
    description:    String, // Extra descriptions
    title:          String, // PC / NPC s title
    sex:            Sex,    // PC / NPC s sex
    class:          Class,  // PC s class
    level:          u8,     // PC / NPC s level
    hometown:       u32,    // PC s Hometown (zone)
    talks:          [bool; constants::MAX_TOUNGE], // PC s Tounges 0 for NPC
    time:           TimeData,   // PC s AGE in days
    tmptime:        TimeData,   // PC s AGE in days, modified
    weight:         u8,     // PC / NPC s weight
    height:         u8,     // PC / NPC s height
}

// used in CHAR_FILE_U *DO*NOT*CHANGE*
pub struct CharAbilityData {
    stren:      i8,
    str_add:    i8,     // 000 - 100 if strength 18
    intel:      i8,
    wis:        i8,
    dex:        i8,
    con:        i8,
}

// Used in CHAR_FILE_U DO NOT CHANGE
pub struct CharPointData {
    mana:       i16,
    max_mana:   i16, // Not useable may be erased upon player file renewal
    hit:        i16,
    max_hit:    i16,    // Max hit for NPC
    mov:        i16,
    max_mov:    i16,    // Max move for NPC
    armor:      i16,    // internal -100..100, external -10..10 AC
    gold:       i32,    // Money carried
    exp:        i32,    // The experience of the player
    hitroll:    i8,     // Any bonus or penalty to the hit roll
    damroll:    i8,     // Any bonus or penalty to the damage roll
}

pub struct CharSpecialData {
    fighting:           Option<Rc<CharData>>,   // Opponent
    hunting:            Option<Rc<CharData>>,   // Hunting person..
    affected_by:        AffectedFlags,      // bitvector for spells/skills affected by
    position:           Position,           // Standing or ...
    default_pos:        Position,           // Default position for NPC
    act:                SpecialActFlags,    // flags for NPC behavior
    spells_to_learn:    u8,                 // How many can you learn yet this level
    carry_weight:       u32,                // Carried weight
    timer:              i32,                // Timer for update
    was_in_room:        i16,                // storage of location for linkdead people
    apply_saving_throw: EnumMap<SavingThrowModifier, i16>,
    conditions:         EnumMap<Condition, i8>,
    damnodice:          i8,                 // The number of damage dice's
    damsizedice:        i8,                 // The size of te damage dice's
    last_direction:     Direction,          // The last direction the monster went
    attack_type:        i32,                // The Attack Type Bitvector for NPC's
    // Note from sproctor: `attack_type` seems to be unused. It is definitely not a bitvector
    alignment:          i32,                // +-1000 for alignments
}

// Used in CHAR_FILE_U *DO*NOT*CHANGE*
pub struct CharSkillData {
    learned:    i8,
    recognise:  bool,
}

// Used in CHAR_FILE_U *DO*NOT*CHANGE*
pub struct AffectedType {
    spell_type:     i8,     // The type of spell that caused this
    duration:       i16,    // For how long its effects will last
    modifier:       i8,     // This is added to appropriate ability
    location:       AbilityModifier,    // Tells which ability to change(APPLY_XXX)
    bitvector:      AffectedFlags,      // Tells which bits to set (AFF_XXX)
}

// ================== Structure for player/non-player =====================
pub struct CharData {
    nr:             i16,                // monster nr (pos in file)
    in_room:        i16,                // Location
    player:         CharPlayerData,     // Normal data
    abilities:      CharAbilityData,    // Abilities
    tmpabilities:   CharAbilityData,    // The abilities we use
    points:         CharPointData,      // Points
    specials:       CharSpecialData,    // Special plaing constants
    skills:         [CharSkillData; constants::MAX_SKILLS], // Skills

    affected:       Vec<AffectedType>,  // affected by what spells
    equipment:      EnumMap<EquipmentPosition, Rc<ObjData>>, // Equipment array

    carrying:       Vec<Rc<ObjData>>,
    desc:           Option<DescriptorData>, // None for mobiles

    followers:      Vec<Rc<CharData>>,  // List of char followers
    master:         Option<Rc<CharData>>,   // Who is char following?
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

pub struct CharFileU {
    sex:        i8,
    class:      i8,
    level:      i8,
    birth:      i64,        // Time of birth of character
    played:     i32,        // Number of secs played in total
    weight:     u8,
    height:     u8,
    title:      [u8; 80],
    hometown:   i16,
    description:    [u8; 240],
    talks:      [bool; constants::MAX_TOUNGE],
    load_room:  i16,        // Which room to place char in
    abilities:  CharAbilityData,
    points:     CharPointData,
    skills:     [CharSkillData; constants::MAX_SKILLS],
    affected:   [AffectedType; constants::MAX_AFFECT],

    // specials

    spells_to_learn:    i8,
    alignmen:           i32,
    last_logon:         i64,    // Time (in secs) of last logon
    act:                u8,     // ACT Flags

    // char data
    name:           [u8; 20],
    pwd:            [u8; 11],
    apply_saving_throw: [i16; 5],
    conditions:     [i32; 3],
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

pub struct SnoopData {
    snooping:   Rc<CharData>,
    snoop_by:   Rc<CharData>,
}

pub struct DescriptorData {
    descriptor: c_int,
    host:       String,
    pwd:        String,
    pos:        i32,
    connected:  ConnectionMode,
    wait:       i32,
    // showstr_head
    // showstr_point
    // str
    // max_str
    prompt_mode:    i32,
    buf:            [u8; constants::MAX_STRING_LENGTH],
    last_input:     [u8; constants::MAX_INPUT_LENGTH],
    pub output:     VecDeque<String>,   // q of strings to send
    pub input:      VecDeque<String>,   // q of unprocessed input
    character:      Rc<CharData>,       // linked to char
    //original          // original char
    snoop:          SnoopData,          // to snoop people
}

pub struct MsgType {
    attacker_msg:   String, // message to attacker
    victim_msg:     String, // message to victim
    room_msg:       String, // message to room
}

pub struct MessageType {
    die_msg:        MsgType,    // messages when death
    miss_msg:       MsgType,    // messages when miss
    hit_msg:        MsgType,    // messages when hit
    sanctuary_msg:  MsgType,    // messages when hit on sanctuary
    god_msg:        MsgType,    // messages when hit on god
}

pub struct MessageList {
    a_type:             i32,                // Attack type
    number_of_attacks:  i32,                // How many attack messages to chose from
    msg:                Vec<MessageType>,   // List of messages
}

pub struct DexSkillType {
    p_pocket:   i16,
    p_locks:    i16,
    traps:      i16,
    sneak:      i16,
    hide:       i16,
}

pub struct DexAppType {
    reaction:   i16,
    miss_att:   i16,
    defensive:  i16,
}

pub struct StrAppType {
    tohit:      i16,
    todam:      i16,
    carry_w:    i16,
    wield_w:    i16,
}

pub struct WisAppType {
    bonus:  i8, // How many bonus skills a player can practice per level
}

pub struct IntAppType {
    learn:  i8, // how many % a player learns a spell/skill
}

pub struct ConAppType {
    hitp:   i16,
    shock:  i16,
}

pub enum MessageTarget {
    ToRoom,
    ToVict,
    ToNotVict,
    ToChar,
}