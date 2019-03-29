extern crate byteorder;

use std::str;
use std::fmt;

use byteorder::{ByteOrder,NativeEndian};

use super::SMBIOSHeader;

#[derive(Debug)]
pub struct SM3EntryPoint {
    pub checksum: u8,
    pub length: u8,
    pub version_major: u8,
    pub version_minor: u8,
    pub doc_rev: u8,
    pub eps_revision: u8,
    pub reserved: u8,
    pub max_table_size: u32,
    pub table_address: u64
}

// impl fmt::Display for SM3EntryPoint {
//     fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
//         write!(fmt, "Version: {}, Checksum: {}, Length: {}",
//         str::from_utf8(&self.anchor).unwrap(), self.checksum, self.length)
//     }
// }

impl SM3EntryPoint {
    pub fn new(data: &[u8]) -> SM3EntryPoint {
        SM3EntryPoint {
            checksum: data[5],
            length: data[6],
            version_major: data[7],
            version_minor: data[8],
            doc_rev: data[9],
            eps_revision: data[10],
            reserved: data[11],
            max_table_size: NativeEndian::read_u32(&data[12..16]),
            table_address: NativeEndian::read_u64(&data[16..24])
        }
    }
}

#[derive(Debug)]
pub struct SM30BiosInfo {
    header: SMBIOSHeader,
    vendor: u8, // Just make strings fields strings
    version: u8,
    starting_address: u16,
    release_date: u8,
    rom_size: u8,
    characteristics: u64,
    characteristics_ex: u16,
    major_release: u8,
    minor_release: u8,
    ctrl_major_release: u8,
    ctrl_minor_release: u8,
    // extended_rom_size: u16,  Only supported in 3.1
    strings: Vec<String> // get rid of this and just store the 
                         // string pointer in the struct
    // string_segment_size: ?
    // or total size calculated on construction?
    // Calc real length: header.length + characteristic_ex.len + strings.len + 2 (two null lengths)
}

#[derive(Debug)]
pub struct SM31BiosInfo {
    header: SMBIOSHeader,
    vendor: u8, // Just make strings fields strings
    version: u8,
    starting_address: u16,
    release_date: u8,
    rom_size: u8,
    characteristics: u64,
    characteristics_ex: u16,
    major_release: u8,
    minor_release: u8,
    ctrl_major_release: u8,
    ctrl_minor_release: u8,
    extended_rom_size: u16,
    strings: Vec<String> // get rid of this and just store the 
                         // string pointer in the struct
    // string_segment_size: ?
    // or total size calculated on construction?
    // Calc real length: header.length + characteristic_ex.len + strings.len + 2 (two null lengths)
}

impl SM30BiosInfo {
    pub fn new(data: &[u8]) -> SM30BiosInfo {
        let header = SMBIOSHeader::from_bytes(&data);
        let mut strings: Vec<String> = vec![String::new()];
        let mut strings_index: usize = 0;

        // This loop should work for all of them
        for idx in header.length as usize..data.len() {
            if data[idx] == 0x0 && data[idx+1] == 0x0 {
                break
            } else if data[idx] == 0x0 {
                strings_index += 1;
                strings.push(String::new());
                continue
            } else {
                strings[strings_index].push(data[idx] as char);
            }
        }

        SM30BiosInfo {
            header: header,
            vendor: data[4], // todo: make SMBIOSString structure
            version: data[5],
            starting_address: NativeEndian::read_u16(&data[6..8]),
            release_date: data[8],
            rom_size: data[9],
            characteristics: NativeEndian::read_u64(&data[10..18]),
            characteristics_ex: NativeEndian::read_u16(&data[18..20]),
            major_release: data[20],
            minor_release: data[21],
            ctrl_major_release: data[22],
            ctrl_minor_release: data[23],
            strings: strings
        }
    }
}


