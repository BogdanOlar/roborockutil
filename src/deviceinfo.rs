use std::net::{UdpSocket, IpAddr, Ipv4Addr};

/// The UDP port used by discovery messages
pub const DISCOVER_UDP_PORT: u16 = 54321;

enum GetStatusErr {

}

pub fn status(sip: Ipv4Addr, dip: Ipv4Addr, did: u32, token: &[u8; 16], cmdid: u32) {
    println!("{{\"method\": \"get_status\", \"id\": {}, \"params\": {{}}}}", cmdid);
}