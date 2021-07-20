use std::{convert::TryInto, error::Error, net::{Ipv4Addr, ToSocketAddrs, UdpSocket}};
use crate::sanitizers::{self, sanitize};

// The format of a Wake-on-LAN (WOL) magic packet is defined
// as a byte array with 6 bytes of value 255 (0xFF) followed by
// 16 repetitions of the target machine’s 48-bit (6-byte) MAC address.
// This becomes a total of 102 bytes:

const MAGIC_HEADER: [u8; 6] = [0xFF; 6];

/// Contains raw bytes for magic packet
pub struct MagicPacket {
    pub bytes: [u8; 102],
}

impl MagicPacket {
    /// Create new MagicPacket from a raw 6-byte MAC address
    pub fn new(mac_bytes: &[u8; 6]) -> MagicPacket {
        let mut magic_bytes: [u8; 102];
        unsafe {
            magic_bytes = std::mem::MaybeUninit::uninit().assume_init();
            let mut src: *const u8 = &MAGIC_HEADER[0];
            let mut dst: *mut u8 = &mut magic_bytes[0];
            dst.copy_from_nonoverlapping(src, 6);

            src = &mac_bytes[0];
            for _ in 0..16 {
                dst = dst.offset(6);
                dst.copy_from_nonoverlapping(src, 6);
            }
        }
        return MagicPacket { bytes: magic_bytes };
    }

    /// Parse a MAC-string into a packet.
    /// The MAC-string should be 17 characters long, separated by colons (i.e. XX:XX:XX:XX:XX:XX)
    pub fn from_str(mac_str: &str) -> Result<MagicPacket, Box<dyn Error>> {
        let mac_bytes = MagicPacket::parse_macstr(mac_str, ':')?;
        return Ok(MagicPacket::new(&mac_bytes));
    }

    // This method is a bit allocation heavy.
    pub fn parse_macstr<S: AsRef<str>>(mac_str: S, sep: char) -> Result<Box<[u8; 6]>, Box<dyn Error>> {
        let sanitized_macstr = sanitize(mac_str.as_ref(), sanitizers::AddrType::MAC).unwrap();
        let bytes_split: Vec<u8> = sanitized_macstr.split(sep)
            .flat_map(|x| hex::decode(x).expect("Invalid mac!"))
            .collect();

        let arr: [u8; 6] = bytes_split.try_into().unwrap();
        Ok(Box::new(arr))
    }

    /// Send packet to/from specific address/interface
    pub fn send_to<A: ToSocketAddrs>(&self, to_addr: A, from_addr: A) -> std::io::Result<()> {
        let socket = UdpSocket::bind(from_addr)?;
        socket.set_broadcast(true)?;
        socket.send_to(&self.bytes, to_addr)?;
        Ok(())
    }

    /// Send package from whatever interface the os picks
    pub fn send(&self) -> std::io::Result<()> {
        self.send_to(
            (Ipv4Addr::new(0xFF, 0xFF, 0xFF, 0xFF), 9),
            (Ipv4Addr::new(0x0, 0x0, 0x0, 0x0), 0),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn init_packet() {
        let packet1 = MagicPacket::new(&[0xFF; 6]);
        let packet2 = MagicPacket::new(&[0xAA; 6]);

        let vec: [u8; 102] = [0xFF; 102];

        assert_eq!(packet1.bytes, vec);
        assert_ne!(packet2.bytes, vec);
    }

    #[test]
    fn magic_packet_from_new() {
        let packet3 = MagicPacket::from_str("10:10:10:10:10:10").unwrap();
        //let vec: [u8; 102] = [0xFF; 102];
        let slice = &packet3.bytes[packet3.bytes.len() - 6..];
        for a in slice {
            println!("{}", a);
        }

        let bytes: [u8; 6] = [0x10; 6];
        assert_eq!(slice, bytes);
    }

    #[test]
    fn test_parse() {
        let mp = MagicPacket::parse_macstr("ff:ff:ff:ff:ff:ff", ':').unwrap();
        assert_eq!([0xFF; 6], *mp);
        let mp2 = MagicPacket::parse_macstr("10:10:10:10:10:10", ':').unwrap();
        assert_eq!([0x10; 6], *mp2);
    }
}
