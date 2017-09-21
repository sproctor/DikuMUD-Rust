pub const PULSE_ZONE: u64           = 240;
pub const PULSE_MOBILE: u64         = 40;
pub const WAIT_SEC: u64             = 4;
pub const WAIT_ROUND: u64           = 4;

pub const MAX_STRING_LENGTH: usize  = 4096;
pub const MAX_INPUT_LENGTH: usize   = 80;
pub const MAX_MESSAGES: usize       = 60;
pub const MAX_ITEMS: usize          = 153;

pub const SECS_PER_REAL_MIN: u64    = 60;
pub const SECS_PER_REAL_HOUR: u64   = 60 * SECS_PER_REAL_MIN;
pub const SECS_PER_REAL_DAY: u64    = 24 * SECS_PER_REAL_HOUR;
pub const SECS_PER_REAL_YEAR: u64   = 365 * SECS_PER_REAL_DAY;

pub const SECS_PER_MUD_HOUR: u64    = 75;
pub const SECS_PER_MUD_DAY: u64     = 24 * SECS_PER_MUD_HOUR;
pub const SECS_PER_MUD_MONTH: u64   = 35 * SECS_PER_MUD_DAY;
pub const SECS_PER_MUD_YEAR: u64    = 17 * SECS_PER_MUD_MONTH;

pub const MAX_OBJ_AFFECT: usize     = 2;

// For 'char_player_data'
pub const MAX_TOUNGE: usize         = 3;
pub const MAX_SKILLS: usize         = 53;
pub const MAX_WEAR: usize           = 18;
pub const MAX_AFFECT: usize         = 25;
pub const MAX_OBJ_SAVE: usize       = 15; // Used in OBJ_FILE_U *DO*NOT*CHANGE*

pub const DFLT_PORT: u16            = 4000;
pub const MAX_NAME_LENGTH: usize    = 15;
pub const MAX_HOSTNAME: usize       = 256;
pub const OPT_USEC: u32             = 250_000;

// data files used by the game system

pub const DFLT_DIR: &str            = "lib";
pub const WORLD_FILE: &str          = "tinyworld.wld";
pub const MOB_FILE: &str            = "tinyworld.mob";
pub const OBJ_FILE: &str            = "tinyworld.obj";
pub const ZONE_FILE: &str           = "tinyworld.zon";
pub const CREDITS_FILE: &str        = "credits";
pub const NEWS_FILE: &str           = "news";
pub const MOTD_FILE: &str           = "motd";
pub const PLAYER_FILE: &str         = "players";
pub const TIME_FILE: &str           = "time";
pub const IDEA_FILE: &str           = "ideas";
pub const TYPO_FILE: &str           = "typos";
pub const BUG_FILE: &str            = "bugs";
pub const MESS_FILE: &str           = "messages";
pub const SOCMESS_FILE: &str        = "actions";
pub const HELP_KWRD_FILE: &str      = "help_table";
pub const HELP_PAGE_FILE: &str      = "help";
pub const INFO_FILE: &str           = "info";
pub const WIZLIST_FILE: &str        = "wizlist";
pub const POSEMESS_FILE: &str       = "poses";