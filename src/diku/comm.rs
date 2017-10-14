use std::collections::VecDeque;
use std::rc::Rc;
use std::os::unix::io::RawFd;

use nix::sys::socket::{bind, linger, listen, socket, setsockopt, sockopt, AddressFamily, InetAddr, IpAddr, Ipv4Addr, SockAddr, SockType, SockFlag};

use diku::types::*;

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

fn act(string: &str, world: &RoomTable, hide_invisible: bool, ch: &CharData, obj: &ObjData, vict_obj: &CharData, typ: VictimType) {
    let to = match typ {
        ToVict => vec![Rc::new(vict_obj)],
        ToChar => vec![Rc::new(ch)],
        _ => world.get(&ch.in_room).unwrap().people,
    };
}