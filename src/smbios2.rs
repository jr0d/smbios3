extern crate byteorder;

use byteorder::{NativeEndian, ByteOrder};

static SM2_ENTRY_POINT_ANCHOR: &'static [u8; 4] = b"_SM_";
static SM2_INTERMEDIATE_ANCHOR: &'static [u8; 5] = b"_DMI_";

#[derive(Debug)]
pub struct SM21EntryPoint {
    pub checksum: u8,
    pub length: u8,
    pub major_version: u8,
    pub minor_version: u8,
    pub max_structure_size: u16,
    pub entry_point_revision: u8,
    pub intermediate_checksum: u8,
    pub dmi_table_length: u16,
    pub dmi_table_address: u32,
    pub number_of_strcutures: u16,
    pub revision: u8
}

impl SM21EntryPoint {
    pub fn new(data: &[u8]) -> SM21EntryPoint {
        assert_eq!(data.len(), 31);
        // Do checksum validations
        SM21EntryPoint {
            checksum: data[4],
            length: data[5],
            major_version: data[6],
            minor_version: data[7],
            max_structure_size: NativeEndian::read_u16(&data[8..10]),
            entry_point_revision: data[10],
            intermediate_checksum: data[21],
            dmi_table_length: NativeEndian::read_u16(&data[22..24]),
            dmi_table_address: NativeEndian::read_u32(&data[24..28]),
            number_of_strcutures: NativeEndian::read_u16(&data[28..30]),
            revision: data[30]
        }
    }
}