extern crate byteorder;

use std::io::prelude::*;
use std::error::Error;
use std::fs::File;

use byteorder::{ByteOrder,NativeEndian};
static ENTRY_POINT_LINUX: &'static str = "/sys/firmware/dmi/tables/smbios_entry_point";
static DMI_TABLE: &'static str = "/sys/firmware/dmi/tables/DMI";

pub mod smbios2;
pub mod smbios3;

use smbios2::{SM21EntryPoint};
use smbios3::{SM3EntryPoint, SM30BiosInfo, SM31BiosInfo};

#[derive(Debug)]
pub struct SMBIOSHeader {
    stype: u8,
    length: u8,
    handle: u16
}


impl SMBIOSHeader {
    pub fn new(stype: u8, length: u8, handle: u16) -> SMBIOSHeader {
        SMBIOSHeader {
            stype,
            length,
            handle
        }
    }
    
    pub fn from_bytes(data: &[u8]) ->SMBIOSHeader {
        SMBIOSHeader {
            stype: data[0],
            length: data[1],
            handle: NativeEndian::read_u16(&data[2..4])
        }
    }
}


pub struct SmbiosVersion {
    pub major: u8,
    pub minor: u8
}

impl SmbiosVersion {
    pub fn new(major: u8, minor: u8) -> SmbiosVersion {
        SmbiosVersion {major, minor}
    }

    pub fn comparable(&self) -> u16 {
        ((self.minor as u16) << 8) | self.major as u16
    }
}

pub struct SmbiosGenericEntry {
    pub version: SmbiosVersion,
    pub table_size: u32,
    pub table_address: u64
}

pub struct SmbiosRaw {
    pub data: [u8]
}

pub struct SmbiosStructure {
    pub smbios_version: SmbiosVersion,
    pub smbios_header: SMBIOSHeader,
}

impl SmbiosStructure {
    pub fn get_strings_offset(&self) -> u8 {
        self.smbios_header.length
    }
}

pub fn sysfs_read_dmi_table(buf: &mut [u8]) -> Result<usize, Box<Error>> {
    let mut fp = File::open(DMI_TABLE)?;
    Ok(fp.read(buf)?)
}

pub fn sysfs_read_smbios_entry_point(buf: &mut[u8]) -> Result<usize, Box<Error>> {
    let mut fp = File::open(ENTRY_POINT_LINUX)?;
    let n = fp.read(buf)?;
    Ok(n)
}

// Returning references.. life times .. etc
pub fn get_entry_point() -> Result<Vec<u8>, Box<Error>> {
    let mut entry_point_data: Vec<u8> = vec![0;31];
    sysfs_read_smbios_entry_point(&mut entry_point_data)?;
    Ok(entry_point_data)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_entry_point_sm2() {
        let entry_data: [u8;31] = 
            [0x5f, 0x53, 0x4d, 0x5f, 0xd3, 0x1f, 0x02,
             0x08, 0xa6, 0x00, 0x00, 0x00, 0x00, 0x00,
             0x00, 0x00, 0x5f, 0x44, 0x4d, 0x49, 0x5f,
             0x9f, 0xe3, 0x04, 0xb0, 0xe8, 0x0e, 0x00,
             0x15, 0x00, 0x27];

        let entry_point = SM21EntryPoint::new(&entry_data);
        assert_eq!(entry_point.major_version, 2);
    }
}