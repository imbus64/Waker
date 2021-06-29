use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Host {
    pub name: String,
    pub macs: Vec<String>,
    // pub ips: Vec<String>,
}

impl Host {
    pub fn new<S: Into<String>>(name: S, mac: S, ipv4: S) -> Host {
        Host {
            name: name.into(),
            macs: vec![mac.into()],
            // ips: vec![ipv4],
        }
    }
}
