extern crate libc;
extern crate regex;

mod ip;

use std::process::Command;
use std::net::Ipv4Addr;
use std::convert::From;
use std::fmt::Display;

#[derive(Debug)]
enum BotError {
    SysCommandFailed,
    NotConnected,
    Regex(regex::Error),
    ParseInt(std::num::ParseIntError)
}

impl Display for BotError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "BotError: {}", std::error::Error::description(self))
    }
}

impl std::error::Error for BotError {
    fn description(&self) -> &str {
        match *self {
            BotError::SysCommandFailed => "Failed to call a system command.",
            BotError::NotConnected     => "Not connected on an IP4 address.",
            BotError::Regex(_)         => "A regex error occurred.",
            BotError::ParseInt(_)      => "Failed to parse a string to an int."
        }
    }

    fn cause(&self) -> Option<&std::error::Error> {
        match *self {
            BotError::Regex(ref e) => Some(e),
            BotError::ParseInt(ref e) => Some(e),
            _ => None
        }
    }
}

impl From<regex::Error> for BotError {
    fn from(err: regex::Error) -> Self {
        BotError::Regex(err)
    }
}

impl From<std::num::ParseIntError> for BotError {
    fn from(err: std::num::ParseIntError) -> Self {
        BotError::ParseInt(err)
    }
}

fn get_ip4_addr(interface: &str) -> Result<Ipv4Addr, BotError> {
    let output = try!(Command::new("ifconfig")
        .arg(interface)
        .output()
        .or(Err(BotError::SysCommandFailed)));

    let output_str = String::from_utf8_lossy(&output.stdout);

    if output_str.contains("inet addr:") {
        use regex::Regex;
        let re = try!(Regex::new(r"inet addr:(\d{1,3})\.(\d{1,3})\.(\d{1,3})\.(\d{1,3})"));
        let caps = re.captures(&output_str).expect("inet addr but no regex match!");

        let a = try!(caps.at(1).expect("missing capture group 1!").parse());
        let b = try!(caps.at(2).expect("missing capture group 2!").parse());
        let c = try!(caps.at(3).expect("missing capture group 3!").parse());
        let d = try!(caps.at(4).expect("missing capture group 4!").parse());

        Ok(Ipv4Addr::new(a, b, c, d))
    } else {
        Err(BotError::NotConnected)
    }
}

fn main() {
    let ip = get_ip4_addr("wlan0");

    println!("Hello from Rust!");

    ip::get_if_addresses();
    
    match ip {
        Ok(ip_addr) => println!("My IP address is: {}", ip_addr),
        Err(err)    => println!("I couldn't find my IP address! {}", err),
    }
}
