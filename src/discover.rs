use std::net::{UdpSocket, IpAddr};
use miiobin::{MI_DISCOVER_PACKET, MiPacket};
use std::time::Duration;
use std::str;
use crate::discover::Error::{Broadcast, SocketConfig};
use std::error::Error as StdError;
use std::fmt;

#[derive(Debug)]
pub enum Error {
    Broadcast,
    SocketConfig,
    NoResponse,
}

#[derive(Debug)]
pub struct  Response {
    packet: MiPacket,
    ip: IpAddr
}


pub fn discover(ip_str: &str) -> Result<Vec<Response>, Error>{
    let mut ret  = Vec::new();
    let mut ip_str = String::from(ip_str);
    ip_str.extend(":54321".chars());

    if let Ok(socket) = UdpSocket::bind(ip_str.clone()) {
        let broadcast_ip_str = "255.255.255.255:54321";

        if let Err(_) = socket.set_broadcast(true) {
            return Err(SocketConfig);
        }

        if let Err(_) = socket.send_to(&MI_DISCOVER_PACKET, broadcast_ip_str) {
            return Err(Broadcast);
        }

        if let Err(_) = socket.set_broadcast(false) {
            return Err(SocketConfig);
        }

        if let Err(_) = socket.set_read_timeout(Option::Some (Duration::from_millis(2000))) {
            return Err(SocketConfig);
        }

        let mut comm_buf = [0u8;1000];

        loop {
            if let Ok((amt, src)) = socket.recv_from(&mut comm_buf)
            {
                if let Ok(resp) = miiobin::MiPacket::parse(&comm_buf[..amt])
                {
                    if let Ok(token_str) = str::from_utf8(&resp.md5) {
                        if token_str.chars().all(char::is_alphanumeric) &&
                            (resp.payload.len() == 0) &&
                            (resp.reserved == 0) {
                            ret.push(Response { packet: resp, ip:src.ip()});
                        }
                    }
                }
            } else {
                break;
            }
        }
    } else {
        return Err(SocketConfig);
    }

    if ret.len() > 0 {
        Ok(ret)
    } else {
        Err(Error::NoResponse)
    }
}

impl StdError for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Broadcast => "Could not broadcast discovery message",
            Error::SocketConfig => "Socket error",
            Error::NoResponse => "No response received"
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Broadcast => f.write_fmt(
                format_args!("Could not broadcast discovery message")),
            Error::SocketConfig => f.write_fmt(
                format_args!("Socket error")),
            Error::NoResponse => f.write_fmt(
                format_args!("No response received")),
        }
    }
}