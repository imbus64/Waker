// use std::{fs::{File, OpenOptions, metadata}, io::{Read, Write}, path::{Path, PathBuf}};
use std::error::Error;
use std::{
    fs::{File, OpenOptions},
    io::Write,
    path::PathBuf,
};

use crate::host::Host;
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

    pub fn add(&mut self, name: &str, mac_addr: &str) {
        self.list.push(Host {
            name: name.to_string(),
            macs: vec![mac_addr.to_string()],
        });
    }

    pub fn from_json_file(json_path: &PathBuf) -> Result<Machines, Box<dyn Error>> {
        let machines: Machines;
        if json_path.exists() || json_path.is_file() {
            let json: String = std::fs::read_to_string(&json_path)?.parse()?;
            machines = serde_json::from_str(&json)?;
            // println!("Machines loaded from json");
        } else {
            machines = Machines::new();
            let serialized = serde_json::to_string_pretty(&machines)?;
            let mut file = File::create(&json_path)?;
            file.write_all(&serialized.as_bytes())?;
            std::fs::write(&json_path, &serialized)?;
            // println!("Machines created");
        }
        Ok(machines)
    }

    /// Dump this struct in json format. Will NOT create file.
    pub fn dump(&self, json_path: &PathBuf) -> Result<bool, Box<dyn Error>> {
        let serialized = serde_json::to_string_pretty(&self)?;
        let mut file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(&json_path)?;
        file.write_all(&serialized.as_bytes())?;
        // println!("Object written to existing file (truncated)");
        // println!("{}", json_path.to_str().unwrap());
        Ok(true)
    }
}

// The Machine struct holds no information about where its json configuration file resides, if it
// did we could deserialize it into it in the destructor...
// impl Drop for Machines {
//     fn drop(&mut self) {
//         println!("Destructor called...");
//     }
// }

mod tests {
    use super::*;
    #[test]
    fn init_machines() {
        let m = Machines::new();
    }

    #[test]
    fn init_machines_and_add() {
        let mut m = Machines::new();
        m.add("Demo_Machine", "FF:FF:FF:FF:FF:FF");
        assert_eq!(1, m.list.len());
        m.add("Demo_Machine2", "FF:FF:FF:FF:FF:FF");
        assert_eq!(2, m.list.len());
    }

    #[test]
    fn write_and_load_from_file() {
        let mut m = Machines::new();
        m.add("File_Demo_Machine", "FF:FF:FF:FF:FF:FF");
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
