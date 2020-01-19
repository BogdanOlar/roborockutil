use roborockutil::{discovery, deviceinfo, provisioning};
use miiobin::{MI_DISCOVER_UDP_PORT};
extern crate clap;
use clap::{Arg, App, SubCommand, ArgMatches};
use std::net::{Ipv4Addr, UdpSocket};
use std::str::{FromStr, from_utf8};
use std::process;
use std::error::Error as StdError;
use std::fmt;

#[derive(Debug)]
enum ArgError {
    NotFound(String),
    Parse(String, String)
}

fn main() {
    let arg_cmd_name_discover = "discover";
    let arg_cmd_name_status = "status";
    let arg_cmd_name_info = "info";

    let arg_name_sip = "sip";
    let sip_arg = Arg::with_name(arg_name_sip)
        .long(arg_name_sip)
        .help("The local IP on which to open the UDP socket")
        .takes_value(true);

    let arg_name_dip= "dip";
    let dip_arg = Arg::with_name(arg_name_dip)
        .long(arg_name_dip)
        .help("The IP of the target device.")
        .takes_value(true);

    let arg_name_token = "token";
    let token_arg = Arg::with_name(arg_name_token)
        .long(arg_name_token)
        .help("Token used for encryption/decryption (16 alphanumeric characters)")
        .takes_value(true);

    let arg_name_did = "did";
    let did_arg = Arg::with_name(arg_name_did)
        .long(arg_name_did)
        .help("Device ID")
        .takes_value(true);

    let arg_name_stamp = "stamp";
    let stamp_arg = Arg::with_name(arg_name_stamp)
        .long(arg_name_stamp)
        .help("Packet stamp")
        .takes_value(true);

    let arg_name_cmdid = "cmdid";
    let cmdid_arg = Arg::with_name(arg_name_cmdid)
        .long(arg_name_cmdid)
        .help("Command ID")
        .takes_value(true);

    let matches = App::new("roborockutil")
        .version("0.1.0")
        .author("Bogdan Olar <olar.bogdan.dev@gmail.com>")
        .about("RoboRock S5 utility")
        .arg(sip_arg.clone()
            .required(false))
        .subcommand(SubCommand::with_name(arg_cmd_name_discover)
            .about("Discover miio devices")
            .arg(sip_arg.clone()
                .required(true))
            .arg(dip_arg.clone()
                .required(false)))
        .subcommand(SubCommand::with_name(arg_cmd_name_status)
            .about("Get device information")
            .arg(sip_arg.clone()
                .required(true))
            .arg(dip_arg.clone()
                .required(true))
            .arg(did_arg.clone()
                .required(true))
            .arg(token_arg.clone()
                .required(true))
            .arg(stamp_arg.clone()
                .required(true))
            .arg(cmdid_arg.clone()))
        .subcommand(SubCommand::with_name(arg_cmd_name_info)
            .about("Get device status")
            .arg(sip_arg.clone()
                .required(true))
            .arg(dip_arg.clone()
                .required(true))
            .arg(did_arg.clone()
                .required(true))
            .arg(token_arg.clone()
                .required(true))
            .arg(cmdid_arg.clone()))
        .get_matches();

    if let Some(discover_cmd) = matches.subcommand_matches(arg_cmd_name_discover) {
        // process required arguments
        let sip = arg_get_ip(arg_name_sip, &discover_cmd).unwrap_or_else(|e|  {
            eprintln!("{}", e);
            process::exit(1);
        });

        // process optional arguments
        let dip_opt;
        match arg_get_ip(arg_name_dip, &discover_cmd) {
            Ok(ip) => dip_opt = Some(ip),
            Err(_e) => dip_opt = None
        }

        // create UDP socket
        let socket = UdpSocket::bind(sip.to_string() + ":" + MI_DISCOVER_UDP_PORT.to_string().as_str())
            .unwrap_or_else(|e|  {
                eprintln!("{}", e);
                process::exit(1);
            });

        // do discovery
        match discovery::discover(socket, dip_opt) {
            Ok(responses) => { print_discover_results(&responses); }
            Err(e) => { eprintln!("{}", e); }
        }
    }

    if let Some(status_cmd) = matches.subcommand_matches(arg_cmd_name_status) {
        // process required arguments
        let sip = arg_get_ip(arg_name_sip, &status_cmd).unwrap_or_else(|e| {
            eprintln!("{}", e);
            process::exit(1);
        });
        let dip = arg_get_ip(arg_name_dip, &status_cmd).unwrap_or_else(|e| {
            eprintln!("{}", e);
            process::exit(1);
        });
        let did = arg_get_u32(arg_name_did, &status_cmd).unwrap_or_else(|e| {
            eprintln!("{}", e);
            process::exit(1);
        });
        let token = arg_get_token(arg_name_token, &status_cmd).unwrap_or_else(|e| {
            eprintln!("{}", e);
            process::exit(1);
        });
        let mut stamp = arg_get_u32(arg_name_stamp, &status_cmd).unwrap_or_else(|e| {
            eprintln!("{}", e);
            process::exit(1);
        });
        let cmdid = arg_get_u32(arg_name_cmdid, &status_cmd).unwrap_or_else(|e| {
            eprintln!("{}", e);
            process::exit(1);
        });

        // create UDP socket
        let socket = UdpSocket::bind(sip.to_string() + ":" + MI_DISCOVER_UDP_PORT.to_string().as_str())
            .unwrap_or_else(|e|  {
            eprintln!("{}", e);
            process::exit(1);
        });

        // get device status
        let resp = deviceinfo::status(&socket, dip, did, &token, &mut stamp, cmdid);
        println!("{:?}", resp);
    }

    if let Some(info_cmd) = matches.subcommand_matches(arg_cmd_name_info) {
        // process required arguments
        let sip = arg_get_ip(arg_name_sip, &info_cmd).unwrap_or_else(|e| {
            eprintln!("{}", e);
            process::exit(1);
        });
        let dip = arg_get_ip(arg_name_dip, &info_cmd).unwrap_or_else(|e| {
            eprintln!("{}", e);
            process::exit(1);
        });
        let did = arg_get_u32(arg_name_did, &info_cmd).unwrap_or_else(|e| {
            eprintln!("{}", e);
            process::exit(1);
        });
        let token = arg_get_token(arg_name_token, &info_cmd).unwrap_or_else(|e| {
            eprintln!("{}", e);
            process::exit(1);
        });
        let cmdid = arg_get_u32(arg_name_cmdid, &info_cmd).unwrap_or_else(|e| {
            eprintln!("{}", e);
            process::exit(1);
        });

        // create UDP socket
        let socket = UdpSocket::bind(sip.to_string() + ":" + MI_DISCOVER_UDP_PORT.to_string().as_str())
            .unwrap_or_else(|e|  {
                eprintln!("{}", e);
                process::exit(1);
            });

        // get device status
        deviceinfo::info(&socket, dip, did, &token, cmdid);
    }
}


fn arg_get_ip(arg_name_str: &str, arg_matches: &ArgMatches) -> Result<Ipv4Addr, ArgError> {
    if let Some(ip_str) = arg_matches.value_of(arg_name_str) {
        if let Ok(ip) = Ipv4Addr::from_str(ip_str) {
            return Ok(ip);
        } else {
            Err(ArgError::Parse(arg_name_str.to_string(), ip_str.to_string()))
        }
    } else {
        Err(ArgError::NotFound(arg_name_str.to_string()))
    }
}

fn arg_get_u32(arg_name_str: &str, arg_matches: &ArgMatches) -> Result<u32, ArgError> {
    if let Some(val_str) = arg_matches.value_of(arg_name_str) {
        if let Ok(val) = val_str.to_string().parse::<u32>() {
            return Ok(val);
        } else {
            Err(ArgError::Parse(arg_name_str.to_string(), val_str.to_string()))
        }
    } else {
        Err(ArgError::NotFound(arg_name_str.to_string()))
    }
}

fn arg_get_token(arg_name_str: &str, arg_matches: &ArgMatches) -> Result<[u8; 16], ArgError> {
    if let Some(val_str) = arg_matches.value_of(arg_name_str) {
        // FIXME There must be a better way to copy bytes from a &str
        let val = val_str.as_bytes();
        if val.len() == 16 {
            let mut ret: [u8; 16] = [0;16];
            for i in 0..16 {
                ret[i] = val[i];
            }
            return Ok(ret);
        }  else {
            Err(ArgError::Parse(arg_name_str.to_string(), val_str.to_string()))
        }
    } else {
        Err(ArgError::NotFound(arg_name_str.to_string()))
    }
}


/// Prints a list of discovery responses.
///
/// For each of the responses, the content is:
///     - device IP (`--dip`)
///         - device ID (`--did`)
///         - the message stamp (`--stamp`)
///         - the provisioning token (`--token`), which is a 16 character alphanumeric string, or the `INVALID` string
///           if the token is invalid
///
/// # Arguments
///
/// `responses` - A vector containing discovery responses
///
fn print_discover_results(responses: &Vec<discovery::Response>) {
    for r in responses {
        println!("\t--dip {}", r.ip.to_string());
        println!("\t\t--did {}", r.packet.device_id);
        println!("\t\t--stamp {}", r.packet.stamp);

        if let Ok(token_str) = from_utf8(&r.packet.md5) {
            if token_str.chars().all(char::is_alphanumeric)
            {
                println!("\t\t--token {}", token_str);
            } else {
                println!("\t\t--token INVALID");
            }
        } else {
            println!("\t\t--token INVALID");
        }
    }
}

impl StdError for ArgError {
    fn description(&self) -> &str {
        match &*self {
            ArgError::NotFound(_arg_str) => "Missing argument",
            ArgError::Parse(_arg_str,_arg_err) => "Missing argument value"
        }
    }
}

impl fmt::Display for ArgError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &*self {
            ArgError::NotFound(arg_name_str) => f.write_fmt(format_args!("Missing value for --{}", arg_name_str)),
            ArgError::Parse(arg_name_str, arg_err_str) => {
                f.write_fmt(format_args!("Could not parse --{} {}", arg_name_str, arg_err_str))
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test()
    {

    }
}