//! Implementation for the discovery functionality of the miio protocol, specifically for the Xiaomi RoboRock S5
//! vacuum cleaner.
//!
//! This module can be used to discover vacuum robots on a particular network.
//!
//! If the robot is in "provisioning mode" (i.e. the user reset its wifi connection, and the robot is offering a
//! wifi Access Point, then the `md5` value in the received response is actually the access token, which can be used
//! to encrypt/decrypt the rest of the communication with the robot.
//!
//! It should be noted that the token is only valid while the robot is in "provisioning mode".
//! After it connects to the user provided router/network, the communication is encrypted with a different token
//! which, at the moment, can only be obtained by rooting the device. This "cloud token" is unique to each vacuum robot,
//! and is persistent across firmware updates.
//!
//! If a discovery is performed while the robot is provisioned (connected to the use's router), then the md5 value
//! will always be a 16 byte array containing all 0s.
//!

use std::net::{UdpSocket, IpAddr, Ipv4Addr};
use std::{str, fmt, time::Duration};
use miiobin::{MI_DISCOVER_PACKET, MI_DISCOVER_UDP_PORT, MiPacket};
use crate::discovery::Error::{Socket, NoResponse};
use std::error::Error as StdError;

/// How many milliseconds to wait for a response to the discovery request
const LISTEN_TIMEOUT: Duration = Duration::from_millis(2000);

#[derive(Debug)]
pub enum Error {
    Socket(String),
    NoResponse,
}

#[derive(Debug)]
pub struct  Response {
    pub ip: Ipv4Addr,
    pub packet: MiPacket,
}

/// Return a list of miio devices present on a given network, and their IP's. If no responses are received, then
/// an `Error::NoResponse` will be returned.
///
/// # Arguments
///
/// `sip` - Source IP. This address is used to create an UDP socket on a particular local interface.
///         Specifying this address is useful if your machine has multiple network interfaces.
///
/// `dip_opt` - Optional destination address. If this argument is `Option::None`, then the discovery request will
///         be broadcast (i.e. on IP `255.255.255.255`), otherwise the discovery request will addressed to the
///         `Ipv4Addr` contained in `Option::Some(dip)`
///
pub fn discover(sip: Ipv4Addr, dip_opt: Option<Ipv4Addr>) -> Result<Vec<Response>, Error>{
    let mut ret_responses: Vec<Response> = Vec::new();

    match UdpSocket::bind(sip.to_string() + ":" + MI_DISCOVER_UDP_PORT.to_string().as_str()) {
        Ok(socket) =>  {

            // send discovery request
            match dip_opt {
                Some(dip) => {
                    // send the discovery to a particular given IP
                    if let Err(e) = socket.send_to(&MI_DISCOVER_PACKET, dip.to_string() +
                        ":" + MI_DISCOVER_UDP_PORT.to_string().as_str()) {
                        return Err(Socket(e.to_string()));
                    }
                }
                None => {
                    // broadcast discovery
                    if let Err(e) = socket.set_broadcast(true) { return Err(Socket(e.to_string())); }
                    if let Err(e) = socket.send_to(&MI_DISCOVER_PACKET, Ipv4Addr::BROADCAST.to_string() +
                        ":" + MI_DISCOVER_UDP_PORT.to_string().as_str()) {
                        return Err(Socket(e.to_string()));
                    }
                    if let Err(e) = socket.set_broadcast(false) { return Err(Socket(e.to_string())); }
                }
            }

            // listen for responses
            let mut comm_buf = [0u8;1000];
            if let Err(e) = socket.set_read_timeout(Option::Some(LISTEN_TIMEOUT)) {
                return Err(Socket(e.to_string()));
            }
            loop {
                if let Ok((amt, src)) = socket.recv_from(&mut comm_buf)
                {
                    if let Ok(resp) = miiobin::MiPacket::parse(&comm_buf[..amt])
                    {
                        if let Ok(token_str) = str::from_utf8(&resp.md5) {
                            if token_str.chars().all(char::is_alphanumeric) &&
                                (resp.payload.len() == 0) &&
                                (resp.reserved == 0) {
                                // save received valid discovery response
                                if let IpAddr::V4(dip) = src.ip() {
                                    ret_responses.push(Response { packet: resp, ip: dip });
                                }
                            }
                        }
                    }
                } else {
                    break;
                }
            }
        }
        Err(e) => {
            return Err(Socket(e.to_string()));
        }
    }

    if ret_responses.len() > 0 {
        Ok(ret_responses)
    } else {
        Err(NoResponse)
    }
}

impl StdError for Error {
    fn description(&self) -> &str {
        match &*self {
            Error::Socket(_e) => "Socket error",
            Error::NoResponse => "No response received"
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &*self {
            Error::Socket(e) => f.write_fmt(format_args!("Socket error: {}", e)),
            Error::NoResponse => f.write_fmt(format_args!("No response received")),
        }
    }
}
