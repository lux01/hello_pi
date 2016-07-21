extern crate libc;
pub extern crate url;
extern crate hyper;
extern crate websocket;
pub extern crate serde_json;

use std::net::{IpAddr};

mod ip;
pub mod discord;

fn main() {
    println!("Hello from Rust!");

    let interfaces = ip::get_if_addresses().expect("Failed to retrieve interfaces!")
        .into_iter()
        .filter(|ifa| match ifa.addr {
            IpAddr::V4(ref addr) => !addr.is_loopback(),
            IpAddr::V6(_) => false,
        }).collect::<Vec<_>>();
    
    for ifa in interfaces {
        println!("\t{}: {}", ifa.name, ifa.addr);
    }

    let client = hyper::Client::new();
    let conn = discord::Connection::from_token(
        discord::AuthorizationToken::new_bot("TEST".to_owned()),
        &client);

    println!("{:?}", conn);
}
