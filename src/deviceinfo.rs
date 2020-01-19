use crate::deviceinfo::Error::*;
use crate::miiopayloads::*;
use std::net::{UdpSocket, Ipv4Addr};
use miiobin::{MI_DISCOVER_UDP_PORT, MiPacket};
use std::error::Error as StdError;
use std::{str, fmt, time::Duration};

/// How many milliseconds to wait for a response to the discovery request
const LISTEN_TIMEOUT: Duration = Duration::from_millis(5000);

#[derive(Debug)]
pub enum Error {
    Socket(String),
    Packet(String),
    Parse(String),
    NoResponse,
}

/// Return the device status
///
/// # Arguments
///
/// `socket` - UDP socket on which to transmit the `get_status` method, and receive the response
/// `dip` - target device IP
/// `did` - target device ID
/// `token` - encryption key
/// `stamp` - the stamp to be used for the `get_status` method. One way to get the current stamp is to use the
///         stamp value returned in a discovery response package.
/// `cmdid` - Command id. This value is used to match the content of a command (`get_status`) with the content of a
///         response (`StatusResponse`). Its value needs to be incremented for each command-response pair.
///
pub fn status(socket: &UdpSocket, dip: Ipv4Addr, did: u32, token: &[u8; 16], stamp: &mut u32, cmdid: u32)
              -> Result<StatusResponse, Error>
{
    let mut comm_buf = [0u8;1024];

    // FIXME implement serialization for this
    let status_cmd = StatusCommand::new(cmdid);
    let cmd_payload_str = serde_json::to_string(&status_cmd).unwrap();

//    let cmd_payload_str = format!("{{\"method\": \"get_status\", \"id\": {}, \"params\": {{}}}}\0", cmdid);

    let mut packet = MiPacket::new(did, *stamp);
    packet.payload.extend_from_slice(cmd_payload_str.as_bytes());
    if let Err(e) = packet.encrypt(&token) { return Err(Packet(e.to_string())); }
    match packet.pack(&mut comm_buf, &token){
        Ok(byte_count) => {
            if let Err(e) = socket.send_to(&comm_buf[..byte_count], dip.to_string() +
                ":" + MI_DISCOVER_UDP_PORT.to_string().as_str()) {
                return Err(Socket(e.to_string()));
            }
        }
        Err(e) => { return Err(Packet(e.to_string())); }
    }

    if let Err(e) = socket.set_read_timeout(Option::Some(LISTEN_TIMEOUT)) { return Err(Socket(e.to_string())); }

    return match socket.recv_from(&mut comm_buf)
    {
        Ok((amt, _src)) => {
            match MiPacket::parse_decrypt(&comm_buf[..amt], &token) {
                Ok(packet) => {
                    if let Ok(payload_string) = String::from_utf8(packet.payload) {
                        match serde_json::from_str(&payload_string[..find_last_closing_bracket(&payload_string)]) {
                            Ok(info_resp) => {
                                Ok(info_resp)
                            }
                            Err(e) => {
                                Err(Parse(e.to_string() + &payload_string))
                            }
                        }
                    } else { Err(Packet("Could not convert payload to UTF-8 string.".to_string())) }
                }
                Err(e) => { Err(Packet(e.to_string())) }
            }
        }
        Err(e) => { Err(Socket(e.to_string())) }
    }
}

/// FIXME add documentation
#[allow(unused_variables)]
pub fn info(socket: &UdpSocket, dip: Ipv4Addr, did: u32, token: &[u8; 16], cmdid: u32) {
    println!("{{\"method\": \"get_status\", \"id\": {}, \"params\": {{}}}}", cmdid);
}

impl StdError for Error {
    fn description(&self) -> &str {
        match &*self {
            Error::Socket(_e) => "Socket error",
            Error::Packet(_e) => "Packet error",
            Error::Parse(_e) => "JSON parse error",
            Error::NoResponse => "No response received"
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &*self {
            Error::Socket(e) => f.write_fmt(format_args!("Socket error: {}", e)),
            Error::Packet(e) => f.write_fmt(format_args!("Packet error: {}", e)),
            Error::Parse(e) => f.write_fmt(format_args!("Parse error for JSON payload: {}", e)),
            Error::NoResponse => f.write_fmt(format_args!("No response received")),
        }
    }
}