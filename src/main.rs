#![allow(dead_code)]

// use std::{fs::{File, OpenOptions, metadata}, io::{Read, Write}, path::{Path, PathBuf}};
use std::path::PathBuf;
use std::{error::Error, iter::Enumerate, string};

// use serde::{Deserialize, Serialize};
//use serde_json::to;

mod cli_args;           // Provides a custom function that specifies our command line options
mod host;               // The actual Host struct
mod input;              // Gives us a python-like input function, as well as a simple confirm function
mod machines;           // Struct that holds a vec of Hosts, as well as operations on those
mod packet;             // The actual magic packet struct, with wake methods e.t.c.
mod random_machine;     // Exposes a function that generates a random Host, with random mac, ip and name // The actual host struct.

// use crate::packet::*;
use crate::machines::*;
use crate::packet::MagicPacket;
use random_machine::random_host;

// waker -a, --all                  // Wake all configured machines
// waker -n, --name name1, name2    // Specified which configured name to wake
// waker -m, --macs mac1, mac2      // Specifies which macs to send magic packet to in direct mode
// waker -l, --list                 // Lists all configured machines
// waker -e, --edit                 // Enters interactive editing mode
// waker --backup-config [file]     // Prints to stdout unless file is specified

enum RunMode {
    Wake { mode: WakeMode },
    Edit { mode: EditMode },
    List,
    ConfigBak,
}

/// Specifies how and which machines should be woken
enum WakeMode {
    WakeAll,                                  // Wake every configured machine
    WakeSome    { indexes: Vec<i32> },        // Wake machines with these indexes/ids
    Direct      { mac_strings: Vec<String> }, // Wake these mac adresses, non-blocking
}

/// Specifies how to perform edits
enum EditMode {
    Pick,                         // Prompt the user for which machine to edit
    Direct      { name: String }, // Edit machine with specified name
    DirectID    { id: i32 },      // Edit machine with specified id
}

/// Specifies how the program should backup its config file
enum BackupMode {
    ToFile      { path: PathBuf }, // Write to file
    ToStdout,                      // Write to stdout
}

fn main() -> Result<(), Box<dyn Error>> {
    let config_path = match cfg!(debug_assertions) {
        // If this is a debug build, the the path becomes ./waker.json, relative to project root
        true => PathBuf::new().join("waker.json"),

        // If this is a release build, this is essentially ~/.config/waker.json stored in a pathbuf object
        false => dirs::config_dir()
            .expect("Could not find config directory...")
            .join("waker.json"),
    };

    // If file does not exist -> Ask to create it -> dump skeleton json into it
    if !config_path.is_file() {
        let msg = format!(
            "File \"{}\" does not seem to exist...\nCreate it?",
            config_path.to_str().unwrap()
        );
        if input::confirm(&msg) {
            let newfile = std::fs::File::create(&config_path).expect(&format!(
                "Could not create file: {}",
                config_path.to_str().unwrap()
            ));

            // Deserialize an empty Machines struct into the new config file.
            // The parser is not equipped to handle empty or malformatted files...
            // This could potentially be moved into a static method in Machines, like
            // Machines::init_config_file(), which does this for us.
            //
            // For now, this will do.
            let skeleton_machines = Machines::new();
            skeleton_machines.dump(&config_path);
        } else {
            println!("Exiting...");
            return Ok(());
        }
    }

    // TODO: More sophisticated error checking and logging
    let mut machines = Machines::from_json_file(&config_path)?;
    // let rhost = random_host();
    // machines.add("test", "FF:FF:FF:FF:FF:FF");
    // machines.add(&rhost.name, &rhost.macs[0]); // Hack, needs to have a method for this..

    for (index, machine) in machines.list.iter().enumerate() {
        let macs_str = format!("{:?}", machine.macs);
        println!("{:<3}{:25}{}", index, macs_str, machine.name); // TODO: CHANGE THIS FORMAT TO INDEX, NAME, MACS
        for mac in &machine.macs {
            let mp = MagicPacket::from_str(&mac).expect("Could not parse mac address...");
            mp.send();
        }
    }

    machines.dump(&config_path)?;
    return Ok(());
}
