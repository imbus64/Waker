TODO: Review parsing methods

Hosts should be pinged on listing

The format of a Wake-on-LAN (WOL) magic packet is defined 
as a byte array with 6 bytes of value 255 (0xFF) and 
16 repetitions of the target machine’s 48-bit (6-byte) MAC address.

102 bytes

several macs per name

edit, delete, add

wakemode::all

wake(bytes) method
async ping method(ip)?

const MAGIC_BYTES_HEADER: [u8; 6] = [0xFF; 6];
vec<u8> bytes = FFFFFFFFFFFF .reserve(enough) 102?

match run_mode {
    Some() => Do according to some
    None => Figure out what to do(set run_mode)
}

add 8 char string -> to bytes ask for name and add
add 11 char string -> if mac[2,5,8] is same char -> split by it and ask for name -> add

store json in ~/.config/waker.conf
dump-json method
export/import? append?

struct mechine {
    vec of all macs
    ip of machine? so we can ping it?
    name
}

struct mac_list {
    Vec
}


ask what do:
    1. Wake
    2. Wake all
    3. Add
    4. Edit
