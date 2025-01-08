use std::io::prelude::*;
use std::io::{BufReader, Seek, SeekFrom};
use std::io::ErrorKind;
use std::rc::Rc;
use std::str::FromStr;
use std::time::{Duration, UNIX_EPOCH};

use rand::distributions::{IndependentSample, Range};
use rand;
use rand::Rng;
use time;

use diku::handler::fname;
use diku::limits::{hit_limit, mana_limit, move_limit};
use diku::structs::*;
use diku::constants::*;

pub fn number(from: i32, to: i32) -> i32 {
    rand::thread_rng().gen_range(from, to)
}

pub fn dice(number: u32, size: u32) -> u32 {
    assert!(size >= 1);
    let mut sum = 0;
    let range = Range::new(0, size);
    let mut rng = rand::thread_rng();
    for _ in 1..number {
        sum += range.ind_sample(&mut rng);
    }
    sum
}

pub fn log(s: &str) {
    let ct = time::now();
    let timestr = ct.asctime();
    eprintln!("{} :: {}", timestr, s);
}

pub fn mud_time_passed(duration: Duration) -> TimeInfoData {
    let mut secs = duration.as_secs();

    let hours = (secs / SECS_PER_MUD_HOUR) % 24;
    secs -= SECS_PER_MUD_HOUR * hours;
    // These subtractions seem useless -Sean

    let day = (secs / SECS_PER_MUD_DAY) % 35;
    secs -= SECS_PER_MUD_DAY * day;

    let month = (secs / SECS_PER_MUD_MONTH) % 17;
    secs -= SECS_PER_MUD_MONTH * month;

    let year = secs / SECS_PER_MUD_YEAR;

    TimeInfoData {
        hours: hours as u8,
        day: day as u8,
        month: month as u8,
        year: year as u16,
    }
}

pub fn read_char<R: Read>(reader: &mut BufReader<R>) -> u8 {
    let mut b = [0; 1];

    reader.read_exact(&mut b).unwrap();
    while b[0] == b' ' || b[0] == b'\t' || b[0] == b'\r' || b[0] == b'\n' {
        reader.read_exact(&mut b).unwrap();
    }
    b[0]
}

pub fn read_number<R: Read + Seek, F: FromStr>(reader: &mut BufReader<R>, clear: bool) -> Result<F, <F as FromStr>::Err> {
    let mut b = [0; 1];
    let mut buf = Vec::new();

    // trim preceding whitespace
    reader.read_exact(&mut b).unwrap();
    while b[0] == b' ' || b[0] == b'\t' || b[0] == b'\r' || b[0] == b'\n' {
        reader.read_exact(&mut b).unwrap();
    }

    // carry over byte from previous read
    while (b[0] >= b'0' && b[0] <= b'9') || b[0] == b'-' {
        buf.push(b[0]);
        match reader.read(&mut b) {
            Ok(0) => break,
            Ok(_) => (),
            Err(e) => if e.kind() == ErrorKind::Interrupted { continue } else { panic!("{}", e) },
        }
    }

    // trim trailing whitespace
    while clear && (b[0] == b' ' || b[0] == b'\t' || b[0] == b'\r' || b[0] == b'\n') {
        match reader.read(&mut b) {
            Ok(0) => break,
            Ok(_) => continue,
            Err(e) => if e.kind() == ErrorKind::Interrupted { continue } else { panic!("{}", e) },
        }
    }
    
    reader.seek(SeekFrom::Current(-1)).unwrap();

    String::from_utf8(buf).unwrap().parse()
}

pub fn read_line_trim<R: Read>(reader: &mut BufReader<R>) -> String {
    let mut line = String::new();
    reader.read_line(&mut line).unwrap();
    String::from(line.trim())
}

pub fn fread_string<R: Read>(reader: &mut BufReader<R>) -> String {
    let mut string = String::new();
    loop {
        reader.read_line(&mut string).expect("fread_string");
        match string.find('~') {
            None => (),
            Some(offset) => {
                string.drain(offset..);
                break;
            },
        }
    }
    string
}

impl RoomData {
    pub fn is_dark(self: &RoomData) -> bool {
        *self.light.borrow() == 0 && self.room_flags.contains(RoomFlags::DARK)
    }

    pub fn is_light(self: &RoomData) -> bool {
        *self.light.borrow() != 0 || !self.room_flags.contains(RoomFlags::DARK)
    }
}

impl CharData {
    fn age(self: &CharData) -> TimeInfoData {
        let mut player_age = mud_time_passed(self.player.borrow().time.birth.duration_since(UNIX_EPOCH).unwrap());
        player_age.year += 17; // All players start at 17
        player_age
    }

    pub fn is_affected(self: &CharData, skill: AffectedFlags) -> bool {
        self.specials.borrow_mut().affected_by.contains(skill)
    }

    // can subject see character "obj"?
    pub fn can_see(self: &CharData, obj: &CharData) -> bool {
        obj.is_affected(AffectedFlags::AFF_INVISIBLE)
    }

    pub fn hshr(self: &CharData) -> &'static str {
        match self.player.borrow().sex {
            Sex::Male => "his",
            Sex::Female => "her",
            Sex::Neutral => "its",
        }
    }

    pub fn hssh(self: &CharData) -> &'static str {
        match self.player.borrow().sex {
            Sex::Male => "he",
            Sex::Female => "she",
            Sex::Neutral => "it",
        }
    }

    pub fn hmhr(self: &CharData) -> &'static str {
        match self.player.borrow().sex {
            Sex::Male => "him",
            Sex::Female => "her",
            Sex::Neutral => "it",
        }
    }

    pub fn is_npc(self: &CharData) -> bool {
        self.specials.borrow().act.contains(SpecialActFlags::ACT_ISNPC)
    }

    pub fn is_mob(self: &CharData) -> bool {
        self.is_npc() && self.nr.is_some()
    }

    pub fn get_pos(self: &CharData) -> Position {
        self.specials.borrow().position
    }

    pub fn get_cond(self: &CharData, i: Condition) -> i8 {
        self.specials.borrow().conditions[i]
    }

    pub fn get_name(self: &CharData) -> String {
        self.player.borrow().name.clone()
    }

    pub fn get_title(self: &CharData) -> &'static str {
        self.player.borrow().title.clone()
    }

    pub fn get_level(self: &CharData) -> u8 {
        self.player.borrow().level
    }

    pub fn get_class(self: &CharData) -> Class {
        self.player.borrow_mut().class
    }

    pub fn get_home(self: &CharData) -> u16 {
        self.player.borrow().hometown
    }

    pub fn get_age(self: &CharData) -> u16 {
        self.age().year
    }

    pub fn get_str(self: &CharData) -> u8 {
        self.tmpabilities.borrow().str
    }

    pub fn get_add(self: &CharData) -> u8 {
        self.tmpabilities.borrow().str_add
    }

    pub fn get_dex(self: &CharData) -> u8 {
        self.tmpabilities.borrow().dex
    }

    pub fn get_int(self: &CharData) -> u8 {
        self.tmpabilities.borrow().intel
    }

    pub fn get_wis(self: &CharData) -> u8 {
        self.tmpabilities.borrow().wis
    }

    pub fn get_con(self: &CharData) -> u8 {
        self.tmpabilities.borrow().con
    }

    pub fn strength_apply_index(self: &CharData) -> usize {
        if self.get_add() == 0 || self.get_str() != 18 {
            self.get_str() as usize
        } else if self.get_add() <= 50 {
            26
        } else if self.get_add() <= 75 {
            27
        } else if self.get_add() <= 90 {
            28
        } else if self.get_add() <= 99 {
            29
        } else {
            30
        }
    }

    pub fn get_ac(self: &CharData) -> i16 {
        self.points.borrow().armor
    }

    pub fn get_hit(self: &CharData) -> i16 {
        self.points.borrow().hit
    }

    pub fn get_max_hit(self: &CharData) -> i16 {
        hit_limit(self)
    }

    pub fn get_move(self: &CharData) -> i16 {
        self.points.borrow().mov
    }

    pub fn get_move_max(self: &CharData) -> i16 {
        move_limit(self)
    }

    pub fn get_mana(self: &CharData) -> i16 {
        self.points.borrow().mana
    }

    pub fn get_max_mana(self: &CharData) -> i16 {
        mana_limit(self)
    }

    pub fn get_gold(self: &CharData) -> i32 {
        self.points.borrow().gold
    }

    pub fn get_exp(self: &CharData) -> i32 {
        self.points.borrow().exp
    }

    pub fn get_height(self: &CharData) -> u8 {
        self.player.borrow().height
    }

    pub fn get_weight(self: &CharData) -> u8 {
        self.player.borrow().weight
    }

    pub fn get_sex(self: &CharData) -> Sex {
        self.player.borrow().sex
    }

    pub fn get_hitroll(self: &CharData) -> i8 {
        self.points.borrow().hitroll
    }

    pub fn get_damroll(self: &CharData) -> i8 {
        self.points.borrow().damroll
    }

    pub fn awake(self: &CharData) -> bool {
        self.get_pos() > Position::Sleeping
    }

    pub fn wait_state(self: &CharData, cycle: i32) {
        if self.desc.is_some() {
            self.desc.as_ref().unwrap().borrow_mut().wait = cycle;
        }
    }


    // Object and carry related macros

    pub fn can_see_obj(self: &CharData, obj: &ObjData) -> bool {
        (!obj.obj_flags.extra_flags.contains(ItemExtraFlags::ITEM_INVISIBLE) ||
         self.is_affected(AffectedFlags::AFF_DETECT_INVISIBLE)) &&
        !self.is_affected(AffectedFlags::AFF_BLIND) && self.in_room.is_light()
    }

    fn can_carry_w(self: &CharData) -> i32 {
        STR_APP[self.strength_apply_index()].carry_w
    }

    fn can_carry_n(self: &CharData) -> u8 {
        5 + self.get_dex() / 2 + self.get_level() / 2
    }

    fn is_carrying_w(self: &CharData) -> i32 {
        self.specials.borrow().carry_weight
    }

    fn is_carrying_n(self: &CharData) -> u8 {
        self.specials.borrow().carry_items
    }

    pub fn can_carry_obj(self: &CharData, obj: &ObjData) -> bool {
        (self.is_carrying_w() + obj.get_obj_weight() <= self.can_carry_w()) &&
        self.is_carrying_n() + 1 <= self.can_carry_n()
    }

    pub fn can_get_obj(self: &CharData, obj: &ObjData) -> bool {
        obj.can_wear(WearFlags::ITEM_TAKE) && self.can_carry_obj(obj) &&
        self.can_see_obj(obj)
    }

    // char name/short desc(for mobs) or someone?

    pub fn pers<'a>(self: &CharData, ch: &'a CharData) -> String {
        if self.can_see(ch) {
            if !ch.is_npc() { ch.get_name() } else { ch.player.borrow().short_descr.clone() }
        } else {
            String::from("someone")
        }
    }

    pub fn outside(self: &CharData) -> bool {
        self.in_room.room_flags.contains(RoomFlags::INDOORS)
    }

    pub fn exit<'a>(self: &'a CharData, door: Direction) -> Option<Rc<RoomDirectionData>> {
        self.in_room.dir_option.get(&door).map(|d| Rc::clone(d))
    }

    pub fn can_go(self: &CharData, door: Direction) -> bool {
        match self.exit(door) {
            Some(dir) => dir.to_room.is_some() && !dir.exit_info.contains(ExitFlags::EX_CLOSED),
            None => false,
        }
    }

    pub fn get_alignment(self: &CharData) -> i32 {
        self.specials.borrow().alignment
    }

    pub fn is_good(self: &CharData) -> bool {
        self.get_alignment() >= 350
    }

    pub fn is_evil(self: &CharData) -> bool {
        self.get_alignment() <= -350
    }

    pub fn is_neutral(self: &CharData) -> bool {
        !self.is_good() && !self.is_evil()
    }
}

impl ObjData {
    pub fn objs<'a>(self: &'a ObjData, vict: &CharData) -> &'a str {
        if vict.can_see_obj(self) { &self.short_description } else { "something" }
    }

    pub fn objn<'a>(self: &'a ObjData, vict: &CharData) -> &'a str {
        if vict.can_see_obj(self) {
            fname(&self.name)
        } else {
            "something"
        }
    }

    pub fn get_item_type(self: &ObjData) -> ItemType {
        self.obj_flags.type_flag
    }

    pub fn can_wear(self: &ObjData, part: WearFlags) -> bool {
        self.obj_flags.wear_flags.contains(part)
    }

    pub fn get_obj_weight(self: &ObjData) -> i32 {
        self.obj_flags.weight
    }

    pub fn is_obj_state(self: &ObjData, stat: ItemExtraFlags) -> bool {
        self.obj_flags.extra_flags.contains(stat)
    }

    pub fn ana(self: &ObjData) -> &'static str {
        if "aeiouyAEIOUY".contains(self.name.chars().nth(0).unwrap()) { "An" } else { "A" }
    }

    pub fn sana(self: &ObjData) -> &'static str {
        if "aeiouyAEIOUY".contains(self.name.chars().nth(0).unwrap()) { "an" } else { "a" }
    }
}

pub fn current_line_number<R: Read + Seek>(reader: &mut BufReader<R>) -> u32 {
    let mut pos = reader.seek(SeekFrom::Current(0)).unwrap();
    reader.seek(SeekFrom::Start(0)).unwrap();
    let mut count = 0;
    loop {
        count += 1;
        match reader.read_line(&mut String::new()) {
            Ok(n) => if n as u64 > pos {
                return count;
            } else {
                pos -= n as u64;
            },
            Err(_) => return count,
        }
    }
}