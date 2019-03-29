extern crate dmi;

use dmi::get_entry_point;
use dmi::smbios2::SM21EntryPoint;
use dmi::smbios3::SM3EntryPoint;

fn main() {
    let entry_point_data = match get_entry_point() {
        Ok(d) => d,
        Err(_) => panic!("Error")
    };

    println!("LENGTH: {}", entry_point_data.len());
    //let sm21_entry_point = SM21EntryPoint::new(&entry_point_data);
    let version = if &entry_point_data[..5] == b"_SM3_" {
        let sm3 = SM3EntryPoint::new(&entry_point_data);
        sm3.version_major
    } else {
        let sm2 = SM21EntryPoint::new(&entry_point_data);
        sm2.major_version
    };

    
    println!("XXX: {:?}", version);
}