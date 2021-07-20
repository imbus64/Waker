// #![allow(dead_code)]
// #![allow(unused_imports)]

use std::fs::{self, File, OpenOptions};
use std::io::Write;
use std::path::PathBuf;
use std::error::Error;
use std::str::FromStr;

// use std::{fs::{File, OpenOptions, metadata}, io::{Read, Write}, path::{Path, PathBuf}};
// use serde::{Deserialize, Serialize};
//use serde_json::to;

mod cli_args; // Provides a custom function that specifies our command line options
mod host; // The actual Host struct
mod input; // Gives us a python-like input function, as well as a simple confirm function
mod machines; // Struct that holds a vec of Hosts, as well as operations on those
mod packet; // The actual magic packet struct, with wake methods e.t.c.
mod sanitizers; // Functions that sanitizes MAC and IP addresses

// use crate::packet::*;
use crate::machines::*;
use host::Host;
use input::*;

// waker -a, --all                  // Wake all configured machines
// waker -n, --name name1, name2    // Specified which configured name to wake
// waker -m, --macs mac1, mac2      // Specifies which macs to send magic packet to in direct mode
// waker -l, --list                 // Lists all configured machines
// waker -e, --edit                 // Enters interactive editing mode
// waker --backup-config [file]     // Prints to stdout unless file is specified

// Returned by cli argument parser (get_runmode())
// This should later be matched in the main program to execute the corresponding functionality
/// Root enum for dictating program behaviour
pub enum RunMode {
    Wake(WakeMode),
    Edit,
    Add,
    List,
    Backup(BackupMode),
}

/// Specifies how and which machines should be wol'ed
pub enum WakeMode {
    WakeAll,                 // Wake every configured machine
    WakeSome,                // Interactively pick hosts to wake
    DirectMacs(Vec<String>), // Wake these mac adresses, non-blocking
}

// /// Specifies how to perform edits
// pub enum EditMode {
//     Pick,           // Prompt the user for which machine to edit
//     Direct(String), // Edit machine with specified name
//     DirectID(i32),  // Edit machine with specified id
// }

// Describes how to edit a host
enum HostEditMode {
    EditName,
    EditIps,
    EditMacs,
}

/// Specifies how the program should backup its config file
pub enum BackupMode {
    ToFile(String), // Write to file
    ToStdout,       // Write to stdout
}

// fn prompt_file_creation(config_path: &PathBuf) -> Result<(), Box<dyn Error>> {
fn prompt_file_creation(config_path: &PathBuf) -> Option<File> {
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
        match Machines::create_skeleton_config(config_path) {
            Ok(()) => { return Some(newfile); }
            Err(_) => return None,
        }
    } else {
        return None;
    }
}

// I think this one is pretty ok. It gets no points for readability, however.
// This could arguably be done with some kind of dictionary as
// well. A custom struct with a builder pattern would maybe work...
/// Presents a prompt. The user picks between the strings in the vector. The function returns an
/// Option<i32> containing the vector index of the picked element.
/// Returns None if input is empty, re-prompts if input invalid or index is out of range.
fn select_option(text: &str, options: &Vec<String>) -> Option<i32> {
    for (index, option) in options.iter().enumerate() {
        println!("{:<5}{}", format!("{}.", index), option)
    }
    loop {
        let input_str = input(&text);
        if input_str.is_empty() {
            return None;
        }
        match input_str.parse::<i32>() {
            Ok(index) => {
                if index >= 0 && (index as usize) < options.len() {
                    return Some(index);
                }
                return None;
            }
            Err(_what) => {
                println!("Invalid input");
            }
        }
    }
}

// INCOMPLETE
// Return Result<(), dyn Error> ?
/// Takes a host reference and a HostEditMode.
/// Behaves according to the HostEditMode provided.
fn edit_host(host: &mut Host, editmode: HostEditMode) {
    println!("Works");
    match editmode {
        HostEditMode::EditName => {
            let new_name = input("New name: ");
            if new_name.len() > 0 {
                host.name = new_name;
            } else {
                println!("Name unchanged...");
            }
        }
        HostEditMode::EditIps => {
            let select = select_option(
                "What do you want to do?: ",
                &vec![
                    "Add an IP-address".to_string(),
                    "Edit an IP-address".to_string(),
                    "Remove an IP-address".to_string(),
                ],
            );
            match select {
                Some(index) => {
                    match index {
                        0 => { // Add
                            let newip = input("New IP: ");
                            match sanitizers::sanitize(&newip, sanitizers::AddrType::IPv4) {
                                Some(ip) => {
                                    host.ips.push(ip);
                                }
                                None => {
                                    println!("Could not parse IP");
                                }
                            }
                        }
                        1 => { // Edit
                            let select = select_option("Which ip?: ", &host.ips);
                            match select {
                                Some(index) => {
                                    let newip = input("New IP: ");
                                    match sanitizers::sanitize(&newip, sanitizers::AddrType::IPv4) {
                                        Some(ip) => {
                                            host.ips[index as usize] = ip;
                                        }
                                        None => {
                                            println!("Could not parse IP");
                                        }
                                    }
                                }
                                None => {}
                            }
                        }
                        2 => { // Remove
                            let select = select_option("Which ip?: ", &host.ips);
                            match select {
                                Some(index) => {
                                    host.ips.remove(index as usize);
                                }
                                None => {}
                            }
                        }
                        _ => {}
                    }
                }
                None => {
                    println!("None selected.");
                }
            }
        }
        HostEditMode::EditMacs => {
            let select = select_option(
                "What do you want to do?: ",
                &vec![
                    "Add a MAC-address".to_string(),
                    "Edit a MAC-address".to_string(),
                    "Remove a MAC-address".to_string(),
                ],
            );
            match select {
                Some(index) => {
                    match index {
                        0 => { // Add
                            let newmac = input("New MAC: ");
                            match sanitizers::sanitize(&newmac, sanitizers::AddrType::MAC) {
                                Some(mac_addr) => {
                                    host.macs.push(mac_addr);
                                }
                                None => {
                                    println!("Could not parse mac_addr");
                                }
                            }
                        }
                        1 => { // Edit
                            let select = select_option("Which MAC?: ", &host.macs);
                            match select {
                                Some(index) => {
                                    let newmac = input("New MAC: ");
                                    match sanitizers::sanitize(&newmac, sanitizers::AddrType::MAC) {
                                        Some(mac_addr) => {
                                            host.macs[index as usize] = mac_addr;
                                        }
                                        None => {
                                            println!("Could not parse MAC");
                                        }
                                    }
                                }
                                None => {}
                            }
                        }
                        2 => { // Remove
                            let select = select_option("Which MAC?: ", &host.macs);
                            match select {
                                Some(index) => {
                                    host.macs.remove(index as usize);
                                }
                                None => {}
                            }
                        }
                        _ => {}
                    }
                }
                None => {
                    println!("None selected.");
                }
            }
        }
    }
}

// This code seems to be complete
/// Drops the user into a prompt for editing a host
fn edit_machines(machines: &mut Machines) {
    loop {
        println!("{}", machines);
        let index_vec = which_indexes("Which host do you wish to edit? (Integer): ", machines);
        match index_vec.len() {
            0 => break,
            1 => {
                println!("Selected: {}", machines.list[index_vec[0] as usize].name);
                let index = index_vec[0] as usize;
                let host = &mut machines.list[index];
                println!("1. Name\n2. IP addresses\n3. Mac addresses\n4. Delete");
                let choice = parse_integers(&input("What would you like to edit? (Integer): "));
                match choice.len() {
                    0 => break,
                    1 => match choice[0] {
                        1 => edit_host(host, HostEditMode::EditName),
                        2 => edit_host(host, HostEditMode::EditIps),
                        3 => edit_host(host, HostEditMode::EditMacs),
                        4 => {
                            if confirm(&format!("Really delete host \"{}\"", machines.list[index].name)) {
                                machines.list.remove(index);
                            }
                        }
                        _ => break,
                    },
                    _ => break,
                }
            }
            _ => break,
        }
        println!("{:?}", index_vec);
    }
}

// Needs reworking. It works as intended but can be written significantly more elegant and
// efficient.
// TODO: In place searching and parsing
/// Parses any integer found in string into a vector of i32.
/// Integers can be arbitrarily separated
fn parse_integers(int_str: &String) -> Vec<i32> {
    let mut return_vector = Vec::<i32>::new();
    let mut intstr = String::new();

    for c in int_str.chars() {
        let mut parse = false;
        if c.is_digit(10) {
            intstr.push(c);
        } else {
            parse = true;
        }

        if !intstr.is_empty() && parse == true {
            return_vector.push(intstr.parse().unwrap());
            intstr.clear();
        }
    }

    if !intstr.is_empty() {
        return_vector.push(intstr.parse().unwrap());
        intstr.clear();
    }

    return return_vector;
}

fn which_indexes<S: AsRef<str>>(message: S, _machines: &Machines) -> Vec<i32> {
    let indexes = input(message.as_ref());
    let integers = parse_integers(&indexes);
    return integers;
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
        let file = prompt_file_creation(&config_path);
        if file.is_none() {
            println!("Exiting...");
        }
        else {
            println!("Config file created.");
        }
        return Ok(());
    }

    // TODO: More sophisticated error checking and logging
    let mut machines = Machines::from_json_file(&config_path)?;
    
    // Figure out how the program should behave
    let run_mode = cli_args::get_runmode();

    match run_mode {
        RunMode::List => {
            println!("{}", machines);
        }
        RunMode::Wake(wake_mode) => {
            match wake_mode {
                WakeMode::WakeAll => {
                    if confirm("You are about to wake all configured machines.\nContinue?") {
                        machines.wakeall();
                        for host in &machines.list {
                            println!("Woke {}", host.name)
                        }
                    }
                }
                WakeMode::WakeSome => {
                    if ! machines.list.is_empty() {
                        println!("{}", machines);
                        let indexes = which_indexes(
                            "Select which hosts to wake up (Comma separated integers): ",
                            &machines,
                        );
                        for index in indexes {
                            // TODO: Bounds checking
                            let host = &machines.list[index as usize];
                            host.wake();
                            println!("Woke {}", host.name)
                        }
                    }
                    else {
                        println!("No machines configured yet... Try \"waker --help\" for information about usage");
                    }
                }
                _ => println!("undefined"),
            }
        }
        RunMode::Edit => {
            edit_machines(&mut machines);
        }
        RunMode::Add => {
            println!("Add new machine:");
            let mut add_machine: bool = true;
            let mut name: String = String::from("");
            let mut mac_addr: String = String::from("");
            let mut ip_addr: Option<String> = None;
            while add_machine {
                name = input("What would you like to call your host?:\n");
                if name.is_empty() {
                    add_machine = false;
                    break;
                }
                if confirm(&format!("Name: {}, is this correct?", &name)) {
                    break;
                }
            }
            while add_machine {
                mac_addr = input("What MAC address is assigned to your host?:\n");
                if mac_addr.is_empty() {
                    add_machine = false;
                    break;
                }
                if confirm(&format!("MAC: {}, is this correct?", &mac_addr)) {
                    break;
                }
            }
            while add_machine {
                let ip_str = input("What IP address is assigned to your host?: (Blank for none)\n");
                if ip_str.is_empty() {
                    ip_addr = None;
                    break;
                }
                if confirm(&format!("IP: {}, is this correct?", &ip_str)) {
                    ip_addr = Some(ip_str);
                    break;
                }
            }
            if add_machine {
                machines.add(&name, &mac_addr, ip_addr);
            }
        }
        // Might need some polish in regards to guards and error handling.
        // Perhaps there is a cleaner way to do the writing...
        // This seems to work fine for now
        RunMode::Backup(backup_mode) => {
            // Read entire file unbuffered into memory.
            let content = fs::read_to_string(&config_path).unwrap();
            match backup_mode {
                // Another valid, and maybe more concise way of doing this is to just do a plain
                // copy of the config file into the *valid* file_string destination.
                // This does its job "good enough"(TM) for now...
                BackupMode::ToFile(file_string) => {
                    let backup_file = PathBuf::from_str(&file_string).unwrap();
                    // If the target file does not already exist, and is not a directory
                    // TODO: Further guards
                    if !backup_file.exists() && !backup_file.is_dir() {
                        println!("Executing file backup");
                        let mut backup_file_handle = OpenOptions::new().create(true).write(true).open(backup_file).unwrap();
                        backup_file_handle.write_all(content.as_bytes()).unwrap();
                    }
                    else if backup_file.exists() && backup_file.is_file() {
                        if confirm(&format!("The file \"{}\" already exists...\nOverwrite?", &file_string)) {
                            let mut backup_file_handle = OpenOptions::new().write(true).open(backup_file).unwrap();
                            backup_file_handle.write_all(content.as_bytes()).unwrap();
                        }
                    }
                    else {
                        // Well, what else should i do? If a user enters a directory as backup
                        // file, id consider that a dead end. 
                        //
                        // Some other obscure errors that i havent considered might end up in this branch as well.
                        println!("Invalid path... Exiting");
                    }
                },
                BackupMode::ToStdout => {
                    for line in content.lines() {
                        println!("{}", line);
                    }
                },
            }
        }
    }

    machines.dump(&config_path)?;
    return Ok(());
}

#[cfg(test)]
mod main_test {
    use super::*;
    #[test]
    fn parse_ints_test() {
        let numvec = vec![10, 20, 30, 2, 4, 1923];
        let numstr = format!("{:?}", numvec);
        let numstr2 = String::from("10kfsa20fav?30!::]2sd4::;sdalkd c           1923"); // Seriously borked input
        let parsed_vec = parse_integers(&numstr);
        let parsed_vec2 = parse_integers(&numstr2);

        assert_eq!(numvec, parsed_vec);
        assert_eq!(numvec, parsed_vec2);

        println!("{:?}", numvec);
        println!("{:?}", parsed_vec);
        println!("{:?}", parsed_vec2);
    }
}
