use std::collections::VecDeque;
use std::os::unix::io::RawFd;

use nix::sys::socket::{bind, linger, listen, socket, setsockopt, sockopt, AddressFamily, InetAddr, IpAddr, Ipv4Addr, SockAddr, SockType, SockFlag};

use diku::types::*;
use diku::utility::{awake, can_see};

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

pub fn send_to_char(messg: String, ch: &mut CharData) {
    match ch.desc.as_mut() {
        Some(desc) => if !messg.is_empty() { write_to_q(messg, &mut desc.output); },
        _ => (),
    }
}

pub fn act(string: &str, world: &RoomTable, hide_invisible: bool, ch: &CharData,
        obj: Option<&ObjData>, vict_obj: Option<&CharData>, typ: VictimType) {
    let tos = match typ {
        ToVict => vict_obj.into_iter().collect(),
        ToChar => vec![ch],
        _ => world[&ch.in_room].people.iter().map(|x| x.as_ref()).collect(),
    };

    for to in tos {
        if to.desc.is_some() && (to != ch || typ == VictimType::ToChar) &&
                (can_see(to, ch) || !hide_invisible) && awake(to) &&
                !(typ == VictimType::ToNotVict && Some(to) == vict_obj) {

        }
    }
}