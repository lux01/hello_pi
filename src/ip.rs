use libc;
use std::collections::HashMap;

use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

pub fn get_if_addresses() -> Option<HashMap<String, IpAddr>>{
    let mut addrs_map = HashMap::new();

    unsafe {
        use std::ptr::{null_mut};

        // The C function getifaddrs(8) returns a linked
        // list of ifaddrs. We store two pointers, one to the
        // head of the list, and one to our current position
        // in the list.
        let mut ifaddrs: *mut libc::ifaddrs = null_mut();
        let mut ifa: *mut libc::ifaddrs;

        
        if libc::getifaddrs(&mut ifaddrs) == -1 {
            return None;
        }

        // Store the head for later so we can free the entire list
        ifa = ifaddrs;

        while !ifa.is_null() {
            if !(*ifa).ifa_addr.is_null() {
                use std::ffi::CStr;
                
                let family = (*(*ifa).ifa_addr).sa_family as libc::c_int;
                let name = CStr::from_ptr((*ifa).ifa_name)
                    .to_string_lossy()
                    .into_owned();

                if family == libc::AF_INET {
                    // Read the IP address as a u32 in Network Order (big endian)
                    let addr = (*((*ifa).ifa_addr as *mut libc::sockaddr_in)).sin_addr
                        .s_addr
                        .to_be();
                    let a = ((addr >> 24) & 0xFFu32) as u8;
                    let b = ((addr >> 16) & 0xFFu32) as u8;
                    let c = ((addr >>  8) & 0xFFu32) as u8;
                    let d = ((addr >>  0) & 0xFFu32) as u8;

                    addrs_map.insert(name, IpAddr::V4(Ipv4Addr::new(a, b, c, d)));
                } else if family == libc::AF_INET6 {
                    let addr = (*((*ifa).ifa_addr as *mut libc::sockaddr_in6))
                        .sin6_addr
                        .s6_addr;

                    let a = ((addr[ 0] as u16) << 8) | (addr[ 1] as u16);
                    let b = ((addr[ 2] as u16) << 8) | (addr[ 3] as u16);
                    let c = ((addr[ 4] as u16) << 8) | (addr[ 5] as u16);
                    let d = ((addr[ 6] as u16) << 8) | (addr[ 7] as u16);
                    let e = ((addr[ 8] as u16) << 8) | (addr[ 9] as u16);
                    let f = ((addr[10] as u16) << 8) | (addr[11] as u16);
                    let g = ((addr[12] as u16) << 8) | (addr[13] as u16);
                    let h = ((addr[14] as u16) << 8) | (addr[15] as u16);

                    addrs_map.insert(name, IpAddr::V6(Ipv6Addr::new(a, b, c, d, e, f, g, h)));
                    
                }
            }

            ifa = (*ifa).ifa_next;
        }


        libc::freeifaddrs(ifaddrs);

        Some(addrs_map)
    }
}
