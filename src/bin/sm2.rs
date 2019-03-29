extern crate dmi;

use dmi::sysfs_read_smbios_entry_point;
use dmi::smbios2::SM21EntryPoint;

fn main() {
    let mut buf = [0 as u8; 31];
    sysfs_read_smbios_entry_point(&mut buf).expect("Could not read entry point!");

    let entry_point = SM21EntryPoint::new(&buf);

    println!("Entry Point: {:?}", entry_point);

}