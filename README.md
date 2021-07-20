# Waker
**An intelligent wake on lan utility to keep track of your machines**

The output from program help flag should provide a clear idea of its use:
```
Waker 0.1.0
Imbus64
Utility for sending magic packets to configured machines.

USAGE:
    waker [FLAGS] [OPTIONS] [MAC ADDRESSES]...

FLAGS:
    -a, --add             Add a new host
        --all             Wake all configured hosts
    -e, --edit            Enter edit mode
    -h, --help            Prints help information
    -l, --list            List all configured entries
    -p, --print-config    Print contents of configuration file to stdout
    -V, --version         Prints version information

OPTIONS:
        --backup <File>    Backup configuration file

ARGS:
    <MAC ADDRESSES>...    

```
This project is currently in beta. Many features are implemented, but some may not work as expected.

## Future plans:
- Further testing and polish in general.
- [ ] Include pinging functionality, so the user gets feedback on what machines are already awake (Top priority)
- [ ] Enable users to import an existing config, appending selected hosts to current config. (Second priority)
- [ ] Perhaps wrap run_mode in an Option, with None being the default when the program is invoked without CLI parameters
- [ ] Rewrite all input/blocking related code into a struct of some sort
