use std::{
    error::Error,
    net::{Ipv4Addr, ToSocketAddrs, UdpSocket},
};

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

    // This is horrible code
    /// Parse a MAC-string into a packet.
    /// Takes both separated and unseparated
    pub fn from_str(mac_str: &str) -> Result<MagicPacket, Box<dyn Error>> {
        use hex::FromHex;
        let mut mac_string = mac_str.trim().to_string();

        if mac_string.len() == 17 {
            for i in 0..5 {
                mac_string.remove(14 - (i * 3)); // Deal with it
            }
        }

        let mut mac_arr: [u8; 6];
        let hex_vec = Vec::from_hex(mac_string)?;

        unsafe {
            mac_arr = std::mem::MaybeUninit::uninit().assume_init();
            let src: *const u8 = &hex_vec[0];
            let dst: *mut u8 = &mut mac_arr[0];
            dst.copy_from_nonoverlapping(src, 6);
        }
        return Ok(MagicPacket::new(&mac_arr));
    }

    pub fn from_str2(mac_str: &str) -> Result<MagicPacket, Box<dyn Error>> {
        return Ok(MagicPacket::new(&MagicPacket::parse(mac_str).unwrap()));
    }

    // Parses string by position if string is 12+5 characters long (delimited by : for example)
    pub fn parse<S: AsRef<str>>(mac_str: S) -> Result<Box<[u8; 6]>, Box<dyn Error>> {
        use hex::FromHex;
        let mut mstr: &str = mac_str.as_ref();
        mstr = mstr.trim();
        let mut bytes: [u8; 6] = [0, 0, 0, 0, 0, 0];

        for (index, byte) in bytes.iter_mut().enumerate() {
            // 0,3,6,9...
            let substr = &mstr[3 * index..3 * index + 2];
            *byte = <[u8; 1]>::from_hex(substr).unwrap()[0];
        }

        return Ok(Box::new(bytes));
    }

    // Loops the string and parses any valid hex
    pub fn parse_harder<S: Into<String>>(mac_str: S) -> Result<Box<[u8; 6]>, Box<dyn Error>> {
        let mstr: String = mac_str.into();
        let mut hexdigits: String = mstr
            .chars()
            .filter(|c| char::is_ascii_hexdigit(c))
            .collect();
        let mut bytes: [u8; 6] = [0; 6];

        if hexdigits.len() >= 12 {
            hexdigits.truncate(12); // May not be needed, since bytes is only 6 bytes, and from_hex might be smart enough to realize...
                                    //let bytes: [u8; 6] = hex::FromHex::from_hex(hexdigits)?;
            bytes = hex::FromHex::from_hex(hexdigits)?;
        }
        return Ok(Box::new(bytes));
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

    /// Return raw bytes, ready to be broadcasted
    pub fn get_bytes(&self) -> &[u8; 102] {
        &self.bytes
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
        let packet3 = MagicPacket::from_str2("10:10:10:10:10:10").unwrap();
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
        let mp = MagicPacket::parse("ff:ff:ff:ff:ff:ff").unwrap();
        assert_eq!([0xFF; 6], *mp);
        let mp2 = MagicPacket::parse("10:10:10:10:10:10").unwrap();
        assert_eq!([0x10; 6], *mp2);
    }

    #[test]
    fn parse_harder_test() {
        let mac = MagicPacket::parse_harder("10:10:10:10:10:10").unwrap();
        assert_eq!([0x10; 6], *mac);
    }
}
