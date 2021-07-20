use serde::{Deserialize, Serialize};

use crate::packet::MagicPacket;

#[derive(Serialize, Deserialize)]
pub struct Host {
    pub name: String,
    pub macs: Vec<String>,
    pub ips: Vec<String>,
}

impl Host {
    pub fn new<S: Into<String>>(name: S, mac: S, ipv4: S) -> Host {
        Host {
            name: name.into(),
            macs: vec![mac.into()],
            ips: vec![ipv4.into()],
        }
    }

    pub fn wake(&self) {
        for mac_str in &self.macs {
            MagicPacket::from_str(&mac_str).unwrap().send().unwrap();
        }
    }
}

impl std::fmt::Display for Host {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let macs_str = format!("{:?}", &self.macs);
        let ips_str = format!("{:?}", &self.ips);
        write!(f, "{:<16} {} - {}", self.name, macs_str, ips_str)
    }
}
