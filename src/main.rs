extern crate libc;
extern crate discord;

use std::net::{IpAddr};

mod ip;

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
    
}
