extern crate clap;
use clap::{Arg, App, SubCommand};

mod discover;

fn main() {
    let ip_src_arg = Arg::with_name("source_ip")
        .long("sip")
        .help("The local IP on which to open the socket")
        .takes_value(true);

    let ip_dst_arg = Arg::with_name("destination_ip")
        .long("dip")
        .help("The IP of the targeted device.")
        .takes_value(true);

    let token_arg = Arg::with_name("token string")
        .long("token")
        .help("Token used for encryption/decryption (16 characters)")
        .takes_value(true);

    let did_arg = Arg::with_name("deviceID")
        .short("d")
        .long("did")
        .help("Device ID")
        .takes_value(true);

    let stamp_arg = Arg::with_name("stamp")
        .long("stamp")
        .help("Packet stamp")
        .takes_value(true);

    let matches = App::new("miioutil")
        .version("0.1.0")
        .author("Bogdan Olar <olar.bogdan.dev@gmail.com>")
        .about("Mi IO utility")
        .arg(did_arg.clone().required(false))
        .subcommand(SubCommand::with_name("discover")
            .about("Discover miio devices")
            .arg(ip_src_arg
                .takes_value(true)
                .required(true)
            ))
        .subcommand(SubCommand::with_name("info")
            .about("Get device information")
            .arg(token_arg.required(true))
            .arg(did_arg.required(true)))
        .get_matches();

    if let Some(discover_cmd) = matches.subcommand_matches("discover") {
        match discover::discover(discover_cmd.value_of("source_ip").unwrap()) {
            Ok(responses) => {
                println!("{:?}", responses);
            }
            Err(e) => {
                println!("{}", e);
            }
        }
    }

//    let my_did = matches.subcommand_matches("info").unwrap();
//    let myfile = my_did.value_of("deviceID");
//
//    println!("The device id passed is: {:?}", myfile);

//    let num_str = matches.value_of("num");
//    match num_str {
//        None => println!("No idea what your favorite number is."),
//        Some(s) => {
//            match s.parse::<i32>() {
//                Ok(n) => println!("Your favorite number must be {}.", n + 5),
//                Err(_) => println!("That's not a number! {}", s),
//            }
//        }
//    }
}




#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test()
    {

    }
}