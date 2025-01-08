use enum_map::EnumMap;

use diku::structs::*;

pub const PULSE_ZONE: u64 = 240;
pub const PULSE_MOBILE: u64 = 40;
pub const WAIT_SEC: u64 = 4;
pub const WAIT_ROUND: u64 = 4;

pub const MAX_STRING_LENGTH: usize = 4096;
pub const MAX_INPUT_LENGTH: usize = 80;
// pub const MAX_MESSAGES: usize       = 60;
pub const MAX_ITEMS: usize = 153;

pub const SECS_PER_REAL_MIN: u64 = 60;
pub const SECS_PER_REAL_HOUR: u64 = 60 * SECS_PER_REAL_MIN;
pub const SECS_PER_REAL_DAY: u64 = 24 * SECS_PER_REAL_HOUR;
pub const SECS_PER_REAL_YEAR: u64 = 365 * SECS_PER_REAL_DAY;

pub const SECS_PER_MUD_HOUR: u64 = 75;
pub const SECS_PER_MUD_DAY: u64 = 24 * SECS_PER_MUD_HOUR;
pub const SECS_PER_MUD_MONTH: u64 = 35 * SECS_PER_MUD_DAY;
pub const SECS_PER_MUD_YEAR: u64 = 17 * SECS_PER_MUD_MONTH;

pub const NOWHERE: i32 = -1;
pub const MAX_OBJ_AFFECT: usize = 2;

// For 'char_player_data'
pub const MAX_TOUNGE: usize = 3;
pub const MAX_SKILLS: usize = 53;
pub const MAX_WEAR: usize = 18;
pub const MAX_AFFECT: usize = 25;
pub const MAX_OBJ_SAVE: usize = 15; // Used in OBJ_FILE_U *DO*NOT*CHANGE*

pub const DFLT_PORT: u16 = 4000;
pub const MAX_NAME_LENGTH: usize = 15;
pub const MAX_HOSTNAME: usize = 256;
pub const OPT_USEC: u32 = 250_000;

// data files used by the game system

pub const DFLT_DIR: &'static str = "lib";
pub const WORLD_FILE: &'static str = "tinyworld.wld";
pub const MOB_FILE: &'static str = "tinyworld.mob";
pub const OBJ_FILE: &'static str = "tinyworld.obj";
pub const ZONE_FILE: &'static str = "tinyworld.zon";
pub const CREDITS_FILE: &'static str = "credits";
pub const NEWS_FILE: &'static str = "news";
pub const MOTD_FILE: &'static str = "motd";
pub const PLAYER_FILE: &'static str = "players";
pub const TIME_FILE: &'static str = "time";
pub const IDEA_FILE: &'static str = "ideas";
pub const TYPO_FILE: &'static str = "typos";
pub const BUG_FILE: &'static str = "bugs";
pub const MESS_FILE: &'static str = "messages";
pub const SOCMESS_FILE: &'static str = "actions";
pub const HELP_KWRD_FILE: &'static str = "help_table";
pub const HELP_PAGE_FILE: &'static str = "help";
pub const INFO_FILE: &'static str = "info";
pub const WIZLIST_FILE: &'static str = "wizlist";
pub const POSEMESS_FILE: &'static str = "poses";

pub const MOVEMENT_LOSS: [u8; 8] = [
    /* SectorType::Inside 		=>*/
    1,
    /* SectorType::City		=>*/
    2,
    /* SectorType::Field		=>*/
    2,
    /* SectorType::Forest		=>*/
    3,
    /* SectorType::Hills		=>*/
    4,
    /* SectorType::Mountain	=>*/
    6,
    /* SectorType::WaterSwim	=>*/
    4,
    /* SectorType::WaterNoSwim	=>*/
    1,
];

pub const DIRS: [&'static str; 6] = [
    "north",
    "east",
    "south",
    "west",
    "up",
    "down",
];

pub const TITLES: [[TitleType; 25]; 4] = [
    /*Class::MagicUser =>*/
    [
        TitleType {
            title_m: "the Man",
            title_f: "the Woman",
            exp: 0,
        },
        TitleType {
            title_m: "the Apprentice of Magic",
            title_f: "the Apprentice of Magic",
            exp: 1,
        },
        TitleType {
            title_m: "the Spell Student",
            title_f: "the Spell Student",
            exp: 2500,
        },
        TitleType {
            title_m: "the Scholar of Magic",
            title_f: "the Scholar of Magic",
            exp: 5000,
        },
        TitleType {
            title_m: "the Delver in Spells",
            title_f: "the Delveress in Spells",
            exp: 10000,
        },
        TitleType {
            title_m: "the Medium of Magic",
            title_f: "the Medium of Magic",
            exp: 20000,
        },
        TitleType {
            title_m: "the Scribe of Magic",
            title_f: "the Scribess of Magic",
            exp: 40000,
        },
        TitleType {
            title_m: "the Seer",
            title_f: "the Seeress",
            exp: 60000,
        },
        TitleType {
            title_m: "the Sage",
            title_f: "the Sage",
            exp: 90000,
        },
        TitleType {
            title_m: "the Illusionist",
            title_f: "the Illusionist",
            exp: 135000,
        },
        TitleType {
            title_m: "the Abjurer",
            title_f: "the Abjuress",
            exp: 250000,
        },
        TitleType {
            title_m: "the Invoker",
            title_f: "the Invoker",
            exp: 375000,
        },
        TitleType {
            title_m: "the Enchanter",
            title_f: "the Enchantress",
            exp: 750000,
        },
        TitleType {
            title_m: "the Conjurer",
            title_f: "the Conjuress",
            exp: 1125000,
        },
        TitleType {
            title_m: "the Magician",
            title_f: "the Witch",
            exp: 1500000,
        },
        TitleType {
            title_m: "the Creator",
            title_f: "the Creator",
            exp: 1875000,
        },
        TitleType {
            title_m: "the Savant",
            title_f: "the Savant",
            exp: 2250000,
        },
        TitleType {
            title_m: "the Magus",
            title_f: "the Craftess",
            exp: 2625000,
        },
        TitleType {
            title_m: "the Wizard",
            title_f: "the Wizard",
            exp: 3000000,
        },
        TitleType {
            title_m: "the Warlock",
            title_f: "the War Witch",
            exp: 3375000,
        },
        TitleType {
            title_m: "the Sorcerer",
            title_f: "the Sorceress",
            exp: 3750000,
        },
        TitleType {
            title_m: "the Immortal Warlock",
            title_f: "the Immortal Enchantress",
            exp: 4000000,
        },
        TitleType {
            title_m: "the Avatar of Magic",
            title_f: "the Empress of Magic",
            exp: 5000000,
        },
        TitleType {
            title_m: "the God of magic",
            title_f: "the Goddess of magic",
            exp: 6000000,
        },
        TitleType {
            title_m: "the Implementator",
            title_f: "the Implementress",
            exp: 7000000,
        },
    ],
    /*Class::Cleric =>*/
    [
        TitleType {
            title_m: "the Man",
            title_f: "the Woman",
            exp: 0,
        },
        TitleType {
            title_m: "the Believer",
            title_f: "the Believer",
            exp: 1,
        },
        TitleType {
            title_m: "the Attendant",
            title_f: "the Attendant",
            exp: 1500,
        },
        TitleType {
            title_m: "the Acolyte",
            title_f: "the Acolyte",
            exp: 3000,
        },
        TitleType {
            title_m: "the Novice",
            title_f: "the Novice",
            exp: 6000,
        },
        TitleType {
            title_m: "the Missionary",
            title_f: "the Missionary",
            exp: 13000,
        },
        TitleType {
            title_m: "the Adept",
            title_f: "the Adept",
            exp: 27500,
        },
        TitleType {
            title_m: "the Deacon",
            title_f: "the Deaconess",
            exp: 55000,
        },
        TitleType {
            title_m: "the Vicar",
            title_f: "the Vicaress",
            exp: 110000,
        },
        TitleType {
            title_m: "the Priest",
            title_f: "the Priestess",
            exp: 225000,
        },
        TitleType {
            title_m: "the Minister",
            title_f: "the Lady Minister",
            exp: 450000,
        },
        TitleType {
            title_m: "the Canon",
            title_f: "the Canon",
            exp: 675000,
        },
        TitleType {
            title_m: "the Levite",
            title_f: "the Levitess",
            exp: 900000,
        },
        TitleType {
            title_m: "the Curate",
            title_f: "the Curess",
            exp: 1125000,
        },
        TitleType {
            title_m: "the Monk",
            title_f: "the Nunne",
            exp: 1350000,
        },
        TitleType {
            title_m: "the Healer",
            title_f: "the Healess",
            exp: 1575000,
        },
        TitleType {
            title_m: "the Chaplain",
            title_f: "the Chaplain",
            exp: 1800000,
        },
        TitleType {
            title_m: "the Expositor",
            title_f: "the Expositress",
            exp: 2025000,
        },
        TitleType {
            title_m: "the Bishop",
            title_f: "the Bishop",
            exp: 2250000,
        },
        TitleType {
            title_m: "the Arch Bishop",
            title_f: "the Arch Lady of the Church",
            exp: 2475000,
        },
        TitleType {
            title_m: "the Patriarch",
            title_f: "the Matriarch",
            exp: 2700000,
        },
        TitleType {
            title_m: "the Immortal Cardinal",
            title_f: "the Immortal Priestess",
            exp: 3000000,
        },
        TitleType {
            title_m: "the Inquisitor",
            title_f: "the Inquisitress",
            exp: 5000000,
        },
        TitleType {
            title_m: "the God of good and evil",
            title_f: "the Goddess of good and evil",
            exp: 6000000,
        },
        TitleType {
            title_m: "the Implementator",
            title_f: "the Implementress",
            exp: 7000000,
        },
    ],
    /*Class::Thief =>*/
    [
        TitleType {
            title_m: "the Man",
            title_f: "the Woman",
            exp: 0,
        },
        TitleType {
            title_m: "the Pilferer",
            title_f: "the Pilferess",
            exp: 1,
        },
        TitleType {
            title_m: "the Footpad",
            title_f: "the Footpad",
            exp: 1250,
        },
        TitleType {
            title_m: "the Filcher",
            title_f: "the Filcheress",
            exp: 2500,
        },
        TitleType {
            title_m: "the Pick-Pocket",
            title_f: "the Pick-Pocket",
            exp: 5000,
        },
        TitleType {
            title_m: "the Sneak",
            title_f: "the Sneak",
            exp: 10000,
        },
        TitleType {
            title_m: "the Pincher",
            title_f: "the Pincheress",
            exp: 20000,
        },
        TitleType {
            title_m: "the Cut-Purse",
            title_f: "the Cut-Purse",
            exp: 30000,
        },
        TitleType {
            title_m: "the Snatcher",
            title_f: "the Snatcheress",
            exp: 70000,
        },
        TitleType {
            title_m: "the Sharper",
            title_f: "the Sharpress",
            exp: 110000,
        },
        TitleType {
            title_m: "the Rogue",
            title_f: "the Rogue",
            exp: 160000,
        },
        TitleType {
            title_m: "the Robber",
            title_f: "the Robber",
            exp: 220000,
        },
        TitleType {
            title_m: "the Magsman",
            title_f: "the Magswoman",
            exp: 440000,
        },
        TitleType {
            title_m: "the Highwayman",
            title_f: "the Highwaywoman",
            exp: 660000,
        },
        TitleType {
            title_m: "the Burglar",
            title_f: "the Burglaress",
            exp: 880000,
        },
        TitleType {
            title_m: "the Thief",
            title_f: "the Thief",
            exp: 1100000,
        },
        TitleType {
            title_m: "the Knifer",
            title_f: "the Knifer",
            exp: 1320000,
        },
        TitleType {
            title_m: "the Quick-Blade",
            title_f: "the Quick-Blade",
            exp: 1540000,
        },
        TitleType {
            title_m: "the Killer",
            title_f: "the Murderess",
            exp: 1760000,
        },
        TitleType {
            title_m: "the Brigand",
            title_f: "the Brigand",
            exp: 1980000,
        },
        TitleType {
            title_m: "the Cut-Throat",
            title_f: "the Cut-Throat",
            exp: 2200000,
        },
        TitleType {
            title_m: "the Immortal Assasin",
            title_f: "the Immortal Assasin",
            exp: 2500000,
        },
        TitleType {
            title_m: "the Demi God of thieves",
            title_f: "the Demi Goddess of thieves",
            exp: 5000000,
        },
        TitleType {
            title_m: "the God of thieves and tradesmen",
            title_f: "the Goddess of thieves and tradesmen",
            exp: 6000000,
        },
        TitleType {
            title_m: "the Implementator",
            title_f: "the Implementress",
            exp: 7000000,
        },
    ],
    /*Class::Warrior =>*/
    [
        TitleType {
            title_m: "the Man",
            title_f: "the Woman",
            exp: 0,
        },
        TitleType {
            title_m: "the Swordpupil",
            title_f: "the Swordpupil",
            exp: 1,
        },
        TitleType {
            title_m: "the Recruit",
            title_f: "the Recruit",
            exp: 2000,
        },
        TitleType {
            title_m: "the Sentry",
            title_f: "the Sentress",
            exp: 4000,
        },
        TitleType {
            title_m: "the Fighter",
            title_f: "the Fighter",
            exp: 8000,
        },
        TitleType {
            title_m: "the Soldier",
            title_f: "the Soldier",
            exp: 16000,
        },
        TitleType {
            title_m: "the Warrior",
            title_f: "the Warrior",
            exp: 32000,
        },
        TitleType {
            title_m: "the Veteran",
            title_f: "the Veteran",
            exp: 64000,
        },
        TitleType {
            title_m: "the Swordsman",
            title_f: "the Swordswoman",
            exp: 125000,
        },
        TitleType {
            title_m: "the Fencer",
            title_f: "the Fenceress",
            exp: 250000,
        },
        TitleType {
            title_m: "the Combatant",
            title_f: "the Combatess",
            exp: 500000,
        },
        TitleType {
            title_m: "the Hero",
            title_f: "the Heroine",
            exp: 750000,
        },
        TitleType {
            title_m: "the Myrmidon",
            title_f: "the Myrmidon",
            exp: 1000000,
        },
        TitleType {
            title_m: "the Swashbuckler",
            title_f: "the Swashbuckleress",
            exp: 1250000,
        },
        TitleType {
            title_m: "the Mercenary",
            title_f: "the Mercenaress",
            exp: 1500000,
        },
        TitleType {
            title_m: "the Swordmaster",
            title_f: "the Swordmistress",
            exp: 1750000,
        },
        TitleType {
            title_m: "the Lieutenant",
            title_f: "the Lieutenant",
            exp: 2000000,
        },
        TitleType {
            title_m: "the Champion",
            title_f: "the Lady Champion",
            exp: 2250000,
        },
        TitleType {
            title_m: "the Dragoon",
            title_f: "the Lady Dragoon",
            exp: 2500000,
        },
        TitleType {
            title_m: "the Cavalier",
            title_f: "the Cavalier",
            exp: 2750000,
        },
        TitleType {
            title_m: "the Knight",
            title_f: "the Lady Knight",
            exp: 3000000,
        },
        TitleType {
            title_m: "the Immortal Warlord",
            title_f: "the Immortal Lady of War",
            exp: 3250000,
        },
        TitleType {
            title_m: "the Extirpator",
            title_f: "the Queen of Destruction",
            exp: 5000000,
        },
        TitleType {
            title_m: "the God of war",
            title_f: "the Goddess of war",
            exp: 6000000,
        },
        TitleType {
            title_m: "the Implementator",
            title_f: "the Implementress",
            exp: 7000000,
        },
    ],
];


/* [class], [level] (all) */
pub const THACO: [[i16; 25]; 4] = [
    [
        100,
        20,
        20,
        20,
        19,
        19,
        19,
        18,
        18,
        18,
        17,
        17,
        17,
        16,
        16,
        16,
        15,
        15,
        15,
        14,
        14,
        14,
        13,
        13,
        13,
    ],
    [
        100,
        20,
        20,
        20,
        18,
        18,
        18,
        16,
        16,
        16,
        14,
        14,
        14,
        12,
        12,
        12,
        10,
        10,
        10,
        8,
        8,
        8,
        6,
        6,
        6,
    ],
    [
        100,
        20,
        20,
        19,
        19,
        18,
        18,
        17,
        17,
        16,
        16,
        15,
        15,
        14,
        13,
        13,
        12,
        12,
        11,
        11,
        10,
        10,
        9,
        9,
        8,
    ],
    [
        100,
        20,
        19,
        18,
        17,
        16,
        15,
        14,
        13,
        12,
        11,
        10,
        9,
        8,
        7,
        6,
        5,
        4,
        3,
        2,
        1,
        1,
        1,
        1,
        1,
    ],
];

pub const STR_APP: [StrAppType; 31] = [
    StrAppType {
        tohit: -5,
        todam: -4,
        carry_w: 0,
        wield_w: 0,
    }, /* 0  */
    StrAppType {
        tohit: -5,
        todam: -4,
        carry_w: 3,
        wield_w: 1,
    }, /* 1  */
    StrAppType {
        tohit: -3,
        todam: -2,
        carry_w: 3,
        wield_w: 2,
    },
    StrAppType {
        tohit: -3,
        todam: -1,
        carry_w: 10,
        wield_w: 3,
    }, /* 3  */
    StrAppType {
        tohit: -2,
        todam: -1,
        carry_w: 25,
        wield_w: 4,
    },
    StrAppType {
        tohit: -2,
        todam: -1,
        carry_w: 55,
        wield_w: 5,
    }, /* 5  */
    StrAppType {
        tohit: -1,
        todam: 0,
        carry_w: 80,
        wield_w: 6,
    },
    StrAppType {
        tohit: -1,
        todam: 0,
        carry_w: 90,
        wield_w: 7,
    },
    StrAppType {
        tohit: 0,
        todam: 0,
        carry_w: 100,
        wield_w: 8,
    },
    StrAppType {
        tohit: 0,
        todam: 0,
        carry_w: 100,
        wield_w: 9,
    },
    StrAppType {
        tohit: 0,
        todam: 0,
        carry_w: 115,
        wield_w: 10,
    }, /* 10  */
    StrAppType {
        tohit: 0,
        todam: 0,
        carry_w: 115,
        wield_w: 11,
    },
    StrAppType {
        tohit: 0,
        todam: 0,
        carry_w: 140,
        wield_w: 12,
    },
    StrAppType {
        tohit: 0,
        todam: 0,
        carry_w: 140,
        wield_w: 13,
    },
    StrAppType {
        tohit: 0,
        todam: 0,
        carry_w: 170,
        wield_w: 14,
    },
    StrAppType {
        tohit: 0,
        todam: 0,
        carry_w: 170,
        wield_w: 15,
    }, /* 15  */
    StrAppType {
        tohit: 0,
        todam: 1,
        carry_w: 195,
        wield_w: 16,
    },
    StrAppType {
        tohit: 1,
        todam: 1,
        carry_w: 220,
        wield_w: 18,
    },
    StrAppType {
        tohit: 1,
        todam: 2,
        carry_w: 255,
        wield_w: 20,
    }, /* 18  */
    StrAppType {
        tohit: 3,
        todam: 7,
        carry_w: 640,
        wield_w: 40,
    },
    StrAppType {
        tohit: 3,
        todam: 8,
        carry_w: 700,
        wield_w: 40,
    }, /* 20  */
    StrAppType {
        tohit: 4,
        todam: 9,
        carry_w: 810,
        wield_w: 40,
    },
    StrAppType {
        tohit: 4,
        todam: 10,
        carry_w: 970,
        wield_w: 40,
    },
    StrAppType {
        tohit: 5,
        todam: 11,
        carry_w: 1130,
        wield_w: 40,
    },
    StrAppType {
        tohit: 6,
        todam: 12,
        carry_w: 1440,
        wield_w: 40,
    },
    StrAppType {
        tohit: 7,
        todam: 14,
        carry_w: 1750,
        wield_w: 40,
    }, /* 25            */
    StrAppType {
        tohit: 1,
        todam: 3,
        carry_w: 280,
        wield_w: 22,
    }, /* 18/01-50      */
    StrAppType {
        tohit: 2,
        todam: 3,
        carry_w: 305,
        wield_w: 24,
    }, /* 18/51-75      */
    StrAppType {
        tohit: 2,
        todam: 4,
        carry_w: 330,
        wield_w: 26,
    }, /* 18/76-90      */
    StrAppType {
        tohit: 2,
        todam: 5,
        carry_w: 380,
        wield_w: 28,
    }, /* 18/91-99      */
    StrAppType {
        tohit: 3,
        todam: 6,
        carry_w: 480,
        wield_w: 30,
    } /* 18/100   (30) */,
];

/* [level] backstab multiplyer (thieves only) */
pub const BACKSTAB_MULT: [u8; 25] = [
    1, /* 0 */
    2, /* 1 */
    2,
    2,
    2,
    3, /* 5 */
    3,
    3,
    3,
    4,
    4, /* 10 */
    4,
    4,
    4,
    5,
    5, /* 15 */
    5,
    5,
    5,
    5,
    5, /* 20 */
    5,
    5,
    5,
    5 /* 25 */,
];

pub const DEX_APP: [DexAppType; 26] = [
    DexAppType {
        reaction: -7,
        miss_att: -7,
        defensive: 6,
    }, /* 0 */
    DexAppType {
        reaction: -6,
        miss_att: -6,
        defensive: 5,
    }, /* 1 */
    DexAppType {
        reaction: -4,
        miss_att: -4,
        defensive: 5,
    },
    DexAppType {
        reaction: -3,
        miss_att: -3,
        defensive: 4,
    },
    DexAppType {
        reaction: -2,
        miss_att: -2,
        defensive: 3,
    },
    DexAppType {
        reaction: -1,
        miss_att: -1,
        defensive: 2,
    }, /* 5 */
    DexAppType {
        reaction: 0,
        miss_att: 0,
        defensive: 1,
    },
    DexAppType {
        reaction: 0,
        miss_att: 0,
        defensive: 0,
    },
    DexAppType {
        reaction: 0,
        miss_att: 0,
        defensive: 0,
    },
    DexAppType {
        reaction: 0,
        miss_att: 0,
        defensive: 0,
    },
    DexAppType {
        reaction: 0,
        miss_att: 0,
        defensive: 0,
    }, /* 10 */
    DexAppType {
        reaction: 0,
        miss_att: 0,
        defensive: 0,
    },
    DexAppType {
        reaction: 0,
        miss_att: 0,
        defensive: 0,
    },
    DexAppType {
        reaction: 0,
        miss_att: 0,
        defensive: 0,
    },
    DexAppType {
        reaction: 0,
        miss_att: 0,
        defensive: 0,
    },
    DexAppType {
        reaction: 0,
        miss_att: 0,
        defensive: -1,
    }, /* 15 */
    DexAppType {
        reaction: 1,
        miss_att: 1,
        defensive: -2,
    },
    DexAppType {
        reaction: 2,
        miss_att: 2,
        defensive: -3,
    },
    DexAppType {
        reaction: 2,
        miss_att: 2,
        defensive: -4,
    },
    DexAppType {
        reaction: 3,
        miss_att: 3,
        defensive: -4,
    },
    DexAppType {
        reaction: 3,
        miss_att: 3,
        defensive: -4,
    }, /* 20 */
    DexAppType {
        reaction: 4,
        miss_att: 4,
        defensive: -5,
    },
    DexAppType {
        reaction: 4,
        miss_att: 4,
        defensive: -5,
    },
    DexAppType {
        reaction: 4,
        miss_att: 4,
        defensive: -5,
    },
    DexAppType {
        reaction: 5,
        miss_att: 5,
        defensive: -6,
    },
    DexAppType {
        reaction: 5,
        miss_att: 5,
        defensive: -6,
    } /* 25 */,
];

pub const CON_APP: [ConAppType; 26] = [
    ConAppType {
        hitp: -4,
        shock: 20,
    }, /* 0 */
    ConAppType {
        hitp: -3,
        shock: 25,
    }, /* 1 */
    ConAppType {
        hitp: -2,
        shock: 30,
    },
    ConAppType {
        hitp: -2,
        shock: 35,
    },
    ConAppType {
        hitp: -1,
        shock: 40,
    },
    ConAppType {
        hitp: -1,
        shock: 45,
    }, /* 5 */
    ConAppType {
        hitp: -1,
        shock: 50,
    },
    ConAppType { hitp: 0, shock: 55 },
    ConAppType { hitp: 0, shock: 60 },
    ConAppType { hitp: 0, shock: 65 },
    ConAppType { hitp: 0, shock: 70 }, /* 10 */
    ConAppType { hitp: 0, shock: 75 },
    ConAppType { hitp: 0, shock: 80 },
    ConAppType { hitp: 0, shock: 85 },
    ConAppType { hitp: 0, shock: 88 },
    ConAppType { hitp: 1, shock: 90 }, /* 15 */
    ConAppType { hitp: 2, shock: 95 },
    ConAppType { hitp: 2, shock: 97 },
    ConAppType { hitp: 3, shock: 99 },
    ConAppType { hitp: 3, shock: 99 },
    ConAppType { hitp: 4, shock: 99 }, /* 20 */
    ConAppType { hitp: 5, shock: 99 },
    ConAppType { hitp: 5, shock: 99 },
    ConAppType { hitp: 5, shock: 99 },
    ConAppType { hitp: 6, shock: 99 },
    ConAppType {
        hitp: 7,
        shock: 100,
    } /* 25 */,
];

// Weapon attack texts
pub const ATTACK_HIT_TEXT: [AttackHitType; 9] = [
    AttackHitType {
        singular: "hit",
        plural: "hits",
    }, /* TYPE_HIT      */
    AttackHitType {
        singular: "pound",
        plural: "pounds",
    }, /* TYPE_BLUDGEON */
    AttackHitType {
        singular: "pierce",
        plural: "pierces",
    }, /* TYPE_PIERCE   */
    AttackHitType {
        singular: "slash",
        plural: "slashes",
    }, /* TYPE_SLASH    */
    AttackHitType {
        singular: "whip",
        plural: "whips",
    }, /* TYPE_WHIP     */
    AttackHitType {
        singular: "claw",
        plural: "claws",
    }, /* TYPE_CLAW     */
    AttackHitType {
        singular: "bite",
        plural: "bites",
    }, /* TYPE_BITE     */
    AttackHitType {
        singular: "sting",
        plural: "stings",
    }, /* TYPE_STING    */
    AttackHitType {
        singular: "crush",
        plural: "crushes",
    } /* TYPE_CRUSH    */,
];

pub const DAM_WEAPONS: [DamWeaponType; 8] = [
    DamWeaponType {
        to_room: "$n misses $N with $s #w.", /*    0    */
        to_char: "You miss $N with your #w.",
        to_victim: "$n miss you with $s #w.",
    },

    DamWeaponType {
        to_room: "$n tickles $N with $s #w.", /*  1.. 2  */
        to_char: "You tickle $N as you #w $M.",
        to_victim: "$n tickle you as $e #W you.",
    },

    DamWeaponType {
        to_room: "$n barely #w $N.", /*  3.. 4  */
        to_char: "You barely #w $N.",
        to_victim: "$n barely #W you.",
    },

    DamWeaponType {
        to_room: "$n #W $N.", /*  5.. 6  */
        to_char: "You #w $N.",
        to_victim: "$n #W you.",
    },

    DamWeaponType {
        to_room: "$n #W $N hard.", /*  7..10  */
        to_char: "You #w $N hard.",
        to_victim: "$n #w you hard.",
    },

    DamWeaponType {
        to_room: "$n #W $N very hard.", /* 11..14  */
        to_char: "You #w $N very hard.",
        to_victim: "$n #W you very hard.",
    },

    DamWeaponType {
        to_room: "$n #w $N extremely hard.", /* 15..20  */
        to_char: "You #w $N extremely hard.",
        to_victim: "$n #w you extremely hard.",
    },

    DamWeaponType {
        to_room: "$n massacre $N to small fragments with $s #w.", /* > 20    */
        to_char: "You massacre $N to small fragments with your #w.",
        to_victim: "$n massacre you to small fragments with $s #w.",
    },
];
