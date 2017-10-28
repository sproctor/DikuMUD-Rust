use std::collections::VecDeque;
use std::os::unix::io::RawFd;

use nix::sys::socket::{bind, linger, listen, socket, setsockopt, sockopt, AddressFamily, InetAddr, IpAddr, Ipv4Addr, SockAddr, SockType, SockFlag};

use diku::handler::fname;
use diku::structs::*;
use diku::utility::log;

fn get_from_q(queue: &mut VecDeque<String>) -> Option<String> {
    queue.pop_back()
}

fn write_to_q(txt: String, queue: &mut VecDeque<String>) {
    queue.push_front(txt)
}

fn flush_queues(d: &mut DescriptorData) {
    d.output.clear();
    d.input.clear();
}

/*******************************************************************
*  socket handling                                                 *
********************************************************************/

pub fn init_socket(port: u16) -> RawFd {
    // Skipping all of the gethostbyname stuff, too much work for finding an AF_INET

    let s = socket(AddressFamily::Inet, SockType::Stream, SockFlag::empty(), 0).expect("Init-socket");
    setsockopt(s, sockopt::ReuseAddr, &true).expect("setsockopt REUSEADDR");
    let ld = linger {
        l_onoff: 1,
        l_linger: 1000,
    };
    setsockopt(s, sockopt::Linger, &ld).expect("setsockopt LINGER");

    let sa = SockAddr::new_inet(InetAddr::new(IpAddr::V4(Ipv4Addr::any()), port));
    bind(s, &sa).expect("bind");
    listen(s, 3).expect("listen");
    s
}

/*****************************************************************
*	Public routines for system-to-player-communication           *
******************************************************************/

pub fn send_to_char(messg: String, ch: &CharData) {
    match ch.desc.as_ref() {
        Some(desc) => if !messg.is_empty() { write_to_q(messg, &mut desc.borrow_mut().output); },
        _ => (),
    }
}

pub fn act(string: &str, hide_invisible: bool, ch: &CharData,
        obj: Option<&ObjData>, vict: Option<&CharData>, vict_obj: Option<&ObjData>,
        vict_str: Option<&str>, vtype: VictimType) {
    match vtype {
        VictimType::ToVict =>
            act_helper(vict.unwrap(), string, hide_invisible, ch, obj, vict,
                vict_obj, vict_str, vtype),
        VictimType::ToChar =>
            act_helper(ch, string, hide_invisible, ch, obj, vict, vict_obj,
                vict_str, vtype),
        _ => for to in &ch.in_room.people {
                act_helper(to, string, hide_invisible, ch, obj, vict,
                    vict_obj, vict_str, vtype);
            },
    };
}

fn act_helper(to: &CharData, string: &str, hide_invisible: bool, ch: &CharData,
        obj: Option<&ObjData>, vict: Option<&CharData>, vict_obj: Option<&ObjData>,
        vict_str: Option<&str>, vtype: VictimType) {
    if to.desc.is_some() && (to != ch || vtype == VictimType::ToChar) &&
            (to.can_see(ch) || !hide_invisible) && to.awake() &&
            !(vtype == VictimType::ToNotVict && Some(to) == vict) {
        let mut buf = String::with_capacity(string.len());
        let mut chars = string.chars();
        loop {
            let c = match chars.next() {
                Some(c) => c, None => break,
            };
            if c == '$' {
                match chars.next().unwrap() {
                    'n' => buf.push_str(&ch.pers(to)),
                    'N' => buf.push_str(&vict.unwrap().pers(to)),
                    'm' => buf.push_str(ch.hmhr()),
                    'M' => buf.push_str(vict.unwrap().hmhr()),
                    's' => buf.push_str(ch.hshr()),
                    'S' => buf.push_str(vict.unwrap().hshr()),
                    'e' => buf.push_str(ch.hssh()),
                    'E' => buf.push_str(vict.unwrap().hssh()),
                    'o' => buf.push_str(obj.unwrap().objn(to)),
                    'O' => buf.push_str(vict_obj.unwrap().objn(to)),
                    'p' => buf.push_str(obj.unwrap().objs(to)),
                    'P' => buf.push_str(vict_obj.unwrap().objs(to)),
                    'a' => buf.push_str(obj.unwrap().sana()),
                    'A' => buf.push_str(obj.unwrap().ana()),
                    'T' => buf.push_str(vict_str.unwrap()),
                    'F' => buf.push_str(fname(vict_str.unwrap())),
                    '$' => buf.push_str("$"),
                    _ => { log("Illegal $-code to act():"); log(string)},
                };
            } else {
                buf.push(c);
            }
        }
        write_to_q(buf, &mut to.desc.as_ref().unwrap().borrow_mut().output);
    }
}