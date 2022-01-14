// use std::{path::PathBuf, str::FromStr};

use crate::{BackupMode, RunMode, WakeMode};
use clap::{App, Arg, ArgMatches};

// use crate::main::RunMode;

// let a = RunMode;
// waker FF:FF:FF:FF:FF:FF          // Direct mode, wakes the given mac address
// waker -a, --all                  // Wake all configured machines
// waker -n, --name name1, name2    // Specified which configured name to wake
// waker -m, --macs mac1, mac2      // Specifies which macs to send magic packet to in direct mode
// waker -l, --list                 // Lists all configured machines
// waker -e, --edit                 // Enters interactive editing mode
// waker --backup-config [file]     // Prints to stdout unless file is specified

// This is essentially and abstraction of clap
/// Parses command line arguments and returns a RunMode enum containing desired run mode.
pub fn get_runmode() -> RunMode {
    let matches = get_cli_matches();
    if matches.is_present("add") {
        return RunMode::Add;
    }
    if matches.is_present("all") {
        return RunMode::Wake(WakeMode::WakeAll);
    }
    if matches.is_present("edit") {
        return RunMode::Edit;
    }
    if matches.is_present("list") {
        return RunMode::List;
    }
    if matches.is_present("backup") {
        let path_str = matches.value_of("backup").unwrap();
        return RunMode::Backup(BackupMode::ToFile(path_str.to_string()));
    }
    if matches.is_present("print_config") {
        return RunMode::Backup(BackupMode::ToStdout);
    }
    return RunMode::Wake(WakeMode::WakeSome);
}

pub fn get_cli_matches() -> ArgMatches {
    /* Move this out to a function that returns a config struct with all the
     * options */
    /* or just return the ArgMatches object for clarity */
    return App::new("Waker")
        //.version("0.01")
        .version(env!("CARGO_PKG_VERSION"))
        .author("Imbus64")
        .about("Utility for sending magic packets to configured machines.")
        .arg(
            Arg::new("add")
                .short('a')
                .long("add")
                .help("Add a new host"),
        )
        .arg(
            Arg::new("all")
                .long("all")
                .help("Wake all configured hosts"),
        )
        .arg(
            Arg::new("edit")
                .short('e')
                .long("edit")
                .help("Enter edit mode"),
        )
        .arg(
            Arg::new("list")
                .short('l')
                .long("list")
                .help("List all configured entries"),
        )
        .arg(
            Arg::new("backup")
                .long("backup")
                .conflicts_with_all(&["list", "all"])
                .help("Backup configuration file")
                .value_name("File"),
        )
        .arg(
            Arg::new("print_config")
                .long("print-config")
                .short('p')
                .conflicts_with_all(&["list", "all"])
                .help("Print contents of configuration file to stdout"),
        )
        .arg(
            Arg::new("MAC ADDRESSES")
                .conflicts_with_all(&["all", "list", "edit", "backup"])
                .multiple_occurrences(true),
        )
        // .short("MAC to be directly woken")
        // .long("asdf")
        .get_matches();
}
