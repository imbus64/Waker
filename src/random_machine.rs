use crate::host::Host;
use eff_wordlist::large::random_word;
use rand::prelude::*;
use rand::thread_rng;

// This file exists purely for debugging/testing purposes

fn random_mac() -> String {
    let mut rng = thread_rng();
    let bytes: [u8; 6] = rng.gen(); // rand can handle array initialization
    let mac_str = format!(
        "{:02X}:{:02X}:{:02X}:{:02X}:{:02X}:{:02X}",
        bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5],
    );
    return mac_str;
}

fn random_ip() -> String {
    let mut rng = thread_rng();
    let bytes: [u8; 4] = rng.gen(); // rand can handle array initialization

    let ip_str = format!("{}.{}.{}.{}", bytes[0], bytes[1], bytes[2], bytes[3],);
    return ip_str;
}

fn random_name() -> String {
    let mut name = String::new();
    name.push_str(random_word());
    name.push_str(random_word());
    return name;
}

pub fn random_host() -> Host {
    return Host::new(random_name(), random_mac(), random_ip());
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_random_name() {
        let mut testvec: Vec<String> = Vec::new();
        for _ in 0..10 {
            // println!("{}", random_name());
            let test_str = random_name();
            println!("{}", test_str);
            for other in &testvec {
                assert!(!other.eq(&test_str)); // Make sure this particular string is not already present in testvec
            }
            testvec.push(test_str);
        }
    }

    #[test]
    fn test_random_ip() {
        let mut testvec: Vec<String> = Vec::new();
        for _ in 0..10 {
            // println!("{}", random_name());
            let test_str = random_ip();
            println!("{}", test_str);
            assert!(test_str.len() >= 7 && test_str.len() <= 15);
            for other in &testvec {
                assert!(!other.eq(&test_str)); // Make sure this particular string is not already present in testvec
            }
            testvec.push(test_str);
        }
    }

    #[test]
    fn test_random_mac() {
        let mut testvec: Vec<String> = Vec::new();
        for _ in 0..10 {
            // println!("{}", random_name());
            let test_str = random_mac();
            println!("{}", test_str);
            // assert!(test_str.len() >= 7 && test_str.len() <= 15);
            assert!(test_str.len() == 17);
            for other in &testvec {
                assert!(!other.eq(&test_str)); // Make sure this particular string is not already present in testvec
            }
            testvec.push(test_str);
        }
    }
}
