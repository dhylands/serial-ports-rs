// Clippy is disabled by default since it doesn't work in stable rust.
//
// You can either use cargo's --features clippy, i.e.:
//
//     cargo run --features clippy --example list_ports
// 
// or edit the features in the Cargo.tomlk file if you'd like to have clippy
// run by default.

#![cfg_attr(feature="clippy", feature(plugin))]

// Make linter fail for every warning
#![cfg_attr(feature="clippy", plugin(clippy))]
#![cfg_attr(feature="clippy", deny(clippy))]

#[macro_use]
extern crate cfg_if;

extern crate libc;

use std::path::PathBuf;
use std::slice::Iter;

cfg_if! {
    if #[cfg(target_os = "linux")] {
        pub mod linux;
        pub use linux::*;
        extern crate glob;
    } else if #[cfg(target_os = "macos")] {
        pub mod macos;
        pub use macos::*;
        extern crate IOKit_sys;
        extern crate mach;
        extern crate CoreFoundation_sys as cf;
    } else {
        // ...
    }
}

#[derive(Debug, Clone)]
pub struct UsbPortInfo {
    pub vid: u16,
    pub pid: u16,
    pub serial_number: Option<String>,
    pub location: Option<String>,
    pub manufacturer: Option<String>,
    pub product: Option<String>,
    pub interface: Option<String>,
}

impl UsbPortInfo {
    pub fn hwid(&self) -> String {
        format!("USB VID:PID={:04X}:{:04X}{}{}",
                self.vid, self.pid,
                self.serial_number.as_ref().map_or("".to_owned(), |sn| format!(" SER={}", sn)),
                self.location.as_ref().map_or("".to_owned(), |loc| format!(" LOCATION={}", loc)))
    }

    pub fn description(&self, name: &str) -> String {
        self.interface.as_ref().map_or(self.product.as_ref().map_or(name.to_owned(),
                                                                    |p| p.to_owned()),
                                       |i| format!("{} - {}", i, self.product.as_ref().unwrap()))
    }
}

#[derive(Debug)]
pub enum ListPortType {
    UsbPort(UsbPortInfo),
    PnpPort,
    AmbaPort,
    NativePort, // Currently used for MacOS for non-USB ports.
    Unknown,
}

#[derive(Debug)]
pub struct ListPortInfo {
    pub device: PathBuf,
    pub name: String,
    pub description: String,
    pub hwid: String,
    pub port_type: ListPortType,
}

impl ListPortInfo {
    pub fn dump(&self) {
        println!("Device: {}", self.device.display());
        println!("             name: {}", self.name);
        println!("      description: {}", self.description);
        println!("             hwid: {}", self.hwid);
        match self.port_type {
            ListPortType::UsbPort(ref info)  => {
                println!("        port_type: UsbPort");
                println!("              vid: {:04x}", info.vid);
                println!("              pid: {:04x}", info.pid);
                println!("    serial_number: {}", option_string(&info.serial_number));
                println!("         location: {}", option_string(&info.location));
                println!("     manufacturer: {}", option_string(&info.manufacturer));
                println!("          product: {}", option_string(&info.product));
                println!("        interface: {}", option_string(&info.interface));
            },
            ListPortType::NativePort    => println!("        port_type: NativePort"),
            ListPortType::PnpPort       => println!("        port_type: PnpPort"),
            ListPortType::AmbaPort      => println!("        port_type: AmbaPort"),
            _                           => println!("        port_type: Unknown"),
        }
    }
}

fn option_string(opt_str: &Option<String>) -> &str {
    opt_str.as_ref().map_or("None", String::as_str)
}

#[derive(Default)]
pub struct ListPorts {
    ports: Vec<ListPortInfo>
}

impl ListPorts {
    pub fn iter(&self) -> Iter<ListPortInfo> {
        self.ports.iter()
    }
}

impl IntoIterator for ListPorts {
    type Item = ListPortInfo;
    type IntoIter = ::std::vec::IntoIter<ListPortInfo>;

    fn into_iter(self) -> Self::IntoIter {
        self.ports.into_iter()
    }
}

// TODO: Write tests. For the tests, I think I'll create a /dev tree
// and a /sys tree and have some hooks to allow /dev and /sys to point
// to the test tree instead of the real one.

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
