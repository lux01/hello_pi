use libc;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

#[derive(Debug)]
pub struct Interface {
    pub name: String,
    pub addr: IpAddr
}

#[derive(Debug)]
pub enum InterfaceError {
    FfiError
}

pub fn get_if_addresses() -> Result<Vec<Interface>, InterfaceError>{
    let mut addrs_vec = Vec::new();

    unsafe {
        use std::ptr::{null_mut};

        // The C function getifaddrs(8) returns a linked
        // list of ifaddrs. We store two pointers, one to the
        // head of the list, and one to our current position
        // in the list.
        let mut ifaddrs: *mut libc::ifaddrs = null_mut();
        let mut ifa: *mut libc::ifaddrs;


        // Build the linked list of interfaces, return None on error.
        // TODO: Handle this error with better accuracy.
        if libc::getifaddrs(&mut ifaddrs) == -1 {
            return Err(InterfaceError::FfiError);
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
                
                // The ifa_addr member can be one of many different sockaddr_X
                // structs. We are only interested in AF_INET (IPv4) and AF_INET6 (IPv6)
                // ones. They return the addresses in various formats, so we have to do
                // some bit juggling to get them into a form suitable for Ipv4Addr and
                // Ipv6Addr respectively.
                if family == libc::AF_INET {
                    let addr = (*((*ifa).ifa_addr as *mut libc::sockaddr_in))
                        .sin_addr
                        .s_addr
                        .to_be(); // Ensure the bytes are big endian
                    
                    let a = ((addr >> 24) & 0xFFu32) as u8;
                    let b = ((addr >> 16) & 0xFFu32) as u8;
                    let c = ((addr >>  8) & 0xFFu32) as u8;
                    let d = ((addr >>  0) & 0xFFu32) as u8;

                    addrs_vec.push(Interface {
                        name: name,
                        addr: IpAddr::V4(Ipv4Addr::new(a, b, c, d)),
                    });
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

                    addrs_vec.push(Interface {
                        name: name,
                        addr: IpAddr::V6(Ipv6Addr::new(a, b, c, d, e, f, g, h))
                    });
                }
            }
            // Advance to the next interface in the list.
            ifa = (*ifa).ifa_next;
        }

        // Free the linked list that we requested.
        libc::freeifaddrs(ifaddrs);

        Ok(addrs_vec)
    }
}
