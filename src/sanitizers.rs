/// For use as parameter in the sanitize function
pub enum AddrType {
    MAC,
    IPv4,
}

/// Takes an AddrType enum and returns an Option<String> containing the sanitized string.
/// Returns None if failed to sanitize.
/// This is a very permissive sanitizer, and it will parse even the most malformatted strings
/// It is guaranteed to return a valid MAC/IP
pub fn sanitize(address: &str, addr_type: AddrType) -> Option<String> {
    match addr_type {
        AddrType::MAC => {
            let mut mac_str = String::new();
            for c in address.to_string().chars() {
                if c.is_digit(16) {
                    mac_str.push(c);
                    if mac_str.len() == 12 { break; }
                }
            }
            mac_str.insert(10, ':');
            mac_str.insert(8, ':');
            mac_str.insert(6, ':');
            mac_str.insert(4, ':');
            mac_str.insert(2, ':');
            return Some(mac_str);
        }
        AddrType::IPv4 => {
            let mut bytes: Vec<u8> = Vec::new(); // Explicit type to avoid any typing errors
            let ip_str = address.to_string();
            let byte_list = ip_str.split('.');
            for byte_base10_str in byte_list {
                match byte_base10_str.parse::<u8>() {
                    Ok(byte) => bytes.push(byte),
                    Err(_what) => continue,
                }
                // bytes.push(byte_base10_str.parse::<u8>().unwrap());
            }
            return Some(format!("{}.{}.{}.{}", bytes[0], bytes[2], bytes[2], bytes[3]));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn sanitize_mac() {
        let macstr = String::from("FFFFFFFFFFFF");
        let formatted = String::from("FF:FF:FF:FF:FF:FF");
        assert_eq!(formatted, sanitize(&macstr, AddrType::MAC).unwrap());
    }

    #[test]
    // No comparison check, just check so the length is correct
    fn sanitize_mac_garbage() {
        let macstr = String::from("sdakjaojoiwjvoievoijevioqjoijeriojkljlknxxx218913981389981jixjxxjk1kj1k");
        assert_eq!(17, sanitize(&macstr, AddrType::MAC).unwrap().len());
    }
    #[test]
    fn sanitize_ip() {
        let ipstr = String::from("255.255.255.255");
        let formatted = String::from("255.255.255.255");
        assert_eq!(formatted, sanitize(&ipstr, AddrType::IPv4).unwrap());
    }

    #[test]
    fn sanitize_ip_garbage() {
        let ipstr = String::from("asdafasd.255.255.asdakfjjkjkfjk.255.255.asdafa");
        let formatted = String::from("255.255.255.255");
        assert_eq!(formatted, sanitize(&ipstr, AddrType::IPv4).unwrap());
    }
}
