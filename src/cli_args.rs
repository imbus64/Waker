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

pub fn get_cli_matches() -> ArgMatches<'static> {
    /* Move this out to a function that returns a config struct with all the
     * options */
    /* or just return the ArgMatches object for clarity */
    return App::new("Waker")
        //.version("0.01")
        .version(env!("CARGO_PKG_VERSION"))
        .author("Imbus64")
        .about("Utility for sending magic packets to configured machines.")
        .arg(
            Arg::with_name("all")
                .short("a")
                .long("all")
                .help("Wake all configured hosts"),
        )
        .arg(
            Arg::with_name("list")
                .short("l")
                .long("list")
                .conflicts_with("weight")
                .help("Print all entries"),
        )
        .arg(
            Arg::with_name("raw")
                .long("raw")
                .conflicts_with_all(&["list", "plain"])
                .help("Print raw log file to stdout"),
        )
        .arg(
            Arg::with_name("plain")
                .long("plain")
                .conflicts_with_all(&["list", "raw"])
                .help("Print all entries without pretty table formatting"),
        )
        .get_matches();
}
