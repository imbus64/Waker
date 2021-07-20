// use std::{fs::{File, OpenOptions, metadata}, io::{Read, Write}, path::{Path, PathBuf}};
use std::error::Error;
use std::{
    fs::{File, OpenOptions},
    io::Write,
    path::PathBuf,
};

use crate::host::Host;
use crate::packet::MagicPacket;
use serde::{Deserialize, Serialize};

// Possibly rename to HostList
#[derive(Serialize, Deserialize)]
pub struct Machines {
    pub list: Vec<Host>,
}

impl Machines {
    pub fn new() -> Machines {
        Machines {
            list: Vec::<Host>::new(),
        }
    }

    // This one needs refactoring...
    /// Add new host to the list, taking a name and a mac, with an optional IP-adress
    pub fn add(&mut self, name: &str, mac_addr: &str, ip_addr: Option<String>) {
        match ip_addr {
            Some(ip_addr) => {
                self.list.push(Host::new(name.to_string(), mac_addr.to_string(), ip_addr.to_string()))
            }
            None => {
                self.list.push(Host {
                    name: name.to_string(),
                    macs: vec![mac_addr.to_string()],
                    ips: vec![],
                });
            }
        }
    }

    /// Parses the Machine object from a json file
    pub fn from_json_file(json_path: &PathBuf) -> Result<Machines, Box<dyn Error>> {
        let machines: Machines;
        if json_path.exists() || json_path.is_file() {
            let json: String = std::fs::read_to_string(&json_path)?.parse()?;
            machines = serde_json::from_str(&json)?;
        } else {
            machines = Machines::new();
            let serialized = serde_json::to_string_pretty(&machines)?;
            let mut file = File::create(&json_path)?;
            file.write_all(&serialized.as_bytes())?;
            std::fs::write(&json_path, &serialized)?;
        }
        Ok(machines)
    }

    fn create_skeleton_config(file: &PathBuf) -> Result<(), Box<dyn Error>> {
        let skel_machines = Machines::new();
        skel_machines.dump(file)?;
        return Ok(());
    }

    /// Dump this struct in json format. Will NOT create file.
    pub fn dump(&self, json_path: &PathBuf) -> Result<bool, Box<dyn Error>> {
        let serialized = serde_json::to_string_pretty(&self)?;
        let mut file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(&json_path)?;
        file.write_all(&serialized.as_bytes())?;
        Ok(true)
    }

    /// Attempts to wake all configured hosts via the default os-provided network interface
    pub fn wakeall(&self) {
        for host in &self.list {
            for mac_str in &host.macs {
                MagicPacket::from_str(mac_str).unwrap().send().unwrap();
            }
        }
    }
}

// I would like to have some kind of iterator comparison here (for the newline), for now this will do...
impl std::fmt::Display for Machines {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (index, host) in self.list.iter().enumerate() {
            write!(f, "{:<3}{}", index, host).unwrap();
            if index != self.list.len()-1 { // Hacky
                write!(f, "\n").unwrap();
            }
        }
        return Ok(());
    }
}

// The Machine struct holds no information about where its json configuration file resides, if it
// did we could deserialize it into it in the destructor...
// impl Drop for Machines {
//     fn drop(&mut self) {
//         println!("Destructor called...");
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn init_machines() {
        let _machine_initialization_test = Machines::new();
    }

    #[test]
    fn init_machines_and_add() {
        let mut m = Machines::new();
        m.add("Demo_Machine", "FF:FF:FF:FF:FF:FF", None);
        assert_eq!(1, m.list.len());
        m.add("Demo_Machine2", "FF:FF:FF:FF:FF:FF", None);
        assert_eq!(2, m.list.len());
    }

    #[test]
    fn write_and_load_from_file() {
        let mut m = Machines::new();
        m.add("File_Demo_Machine", "FF:FF:FF:FF:FF:FF", None);
        assert_eq!(1, m.list.len());
        let path = PathBuf::from("./DEMO_MACHINES.json");
        std::fs::File::create(&path).unwrap();
        m.dump(&path).unwrap();
        // println!("{}", path. to_str().unwrap());
        let m2 = Machines::from_json_file(&path).unwrap();

        assert_eq!(1, m2.list.len());

        // This cleanup will never happen if the previous assert fails...
        // TODO: Branch eq check to a panic instead
        std::fs::remove_file(&path).unwrap();
    }
}
