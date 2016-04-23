extern crate glob;

use glob::glob;
use std::borrow::Cow;
use std::io::{ BufRead, BufReader };
use std::ffi::OsStr;
use std::fs;
use std::path::{ Path, PathBuf };
use std::slice::Iter;

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
    pub fn new(usb_device_path: Option<PathBuf>) -> Self {
        UsbPortInfo {
            vid: read_hex_int(usb_device_path.clone(), "idVendor"),
            pid: read_hex_int(usb_device_path.clone(), "idProduct"),
            serial_number: read_option_string(usb_device_path.clone(), "serial"),
            location: usb_device_path.as_ref().and_then(|dp| dp.file_name()).map(OsStr::to_string_lossy).map(Cow::into_owned),
            manufacturer: read_option_string(usb_device_path.clone(), "manufacturer"),
            product: read_option_string(usb_device_path.clone(), "product"),
            interface: read_option_string(usb_device_path.clone(), "interface"),
        }
    }

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
    Unknown,
}

#[derive(Debug)]
pub struct ListPortInfo {
    pub device: PathBuf,
    pub name: String,
    pub device_path: Option<PathBuf>,
    pub subsystem: Option<String>,
    pub usb_device_path: Option<PathBuf>,
    pub description: String,
    pub hwid: String,
    pub port_type: ListPortType,
}

impl ListPortInfo {
    fn new(dev_name: PathBuf) -> Option<Self> {

        let basename = dev_name.file_name().map(OsStr::to_string_lossy).map(Cow::into_owned);
        if basename.is_none() {
            return None;
        }
        let basename = basename.unwrap();

        let device_path = fs::canonicalize(format!("/sys/class/tty/{}/device", basename)).ok();

        let subsystem_path = device_path.as_ref().and_then(|dp| {
            let mut subsystem_path = PathBuf::from(dp);
            subsystem_path.push("subsystem");
            fs::canonicalize(subsystem_path).ok()});
        let subsystem:Option<String> = subsystem_path.and_then(|pb| pb.file_name().map(OsStr::to_string_lossy).map(Cow::into_owned));

        if subsystem.as_ref().map(String::as_str) == Some("platform") {
            return None;
        }

        let usb_device_path = device_path.as_ref().and_then(|dp|
            match subsystem.as_ref().map(String::as_str) {
                Some("usb-serial")  => PathBuf::from(dp).parent().and_then(|p| p.parent()).map(|p| p.to_path_buf()),
                Some("usb")         => PathBuf::from(dp).parent().map(|p| p.to_path_buf()),
                _ => None,
            }
        );

        let (port_type, description, hwid) = match subsystem.as_ref().map(String::as_str) {
            Some("usb") | Some("usb-serial") => {
                let info = UsbPortInfo::new(usb_device_path.clone());
                (ListPortType::UsbPort(info.clone()), info.description(&basename), info.hwid())
            },
            Some("pnp") => (ListPortType::PnpPort,
                            basename.clone(),
                            read_option_string(device_path.clone(), "id").unwrap_or("".to_owned())),
            Some("amba") => (ListPortType::AmbaPort,
                             basename.clone(),
                             device_path.as_ref().and_then(|dp| dp.file_name()).map(OsStr::to_string_lossy).map(Cow::into_owned).unwrap()),
            _ => (ListPortType::Unknown, "".to_owned(), "".to_owned()),
        };

        let info = ListPortInfo {
            device: dev_name,
            name: basename,
            device_path: device_path,
            subsystem: subsystem,
            usb_device_path: usb_device_path,
            description: description,
            hwid: hwid,
            port_type: port_type,
        };

        Some(info)
    }

    pub fn dump(&self) {
        println!("Device: {}", self.device.display());
        println!("             name: {}", self.name);
        println!("      device_path: {}", option_pathbuf(&self.device_path));
        println!("        subsystem: {}", option_string(&self.subsystem));
        println!("  usb_device_path: {}", option_pathbuf(&self.usb_device_path));
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
            ListPortType::PnpPort       => println!("        port_type: PnpPort"),
            ListPortType::AmbaPort      => println!("        port_type: AmbaPort"),
            _                           => println!("        port_type: Unknown"),
        }
    }
}

fn read_hex_int(usb_device_path: Option<PathBuf>, filename: &str) -> u16 {
    read_option_string(usb_device_path, filename).map_or(0, |s| u16::from_str_radix(&s, 16).unwrap_or(0))
}

fn read_option_string(usb_device_path: Option<PathBuf>, filename: &str) -> Option<String> {
    if usb_device_path.is_none() {
        return None;
    }
    let mut pathname = usb_device_path.unwrap();
    pathname.push(filename);

    let mut line = String::new();
    fs::File::open(pathname).map(|f| BufReader::new(f).read_line(&mut line))
                            .map(|_| line.trim().to_owned()).ok()
}

fn option_string(opt_str: &Option<String>) -> &str {
    opt_str.as_ref().map_or("None", String::as_str)
}

fn option_pathbuf(opt_pb: &Option<PathBuf>) -> &str {
    opt_pb.as_ref().map(PathBuf::as_path).and_then(Path::to_str).unwrap_or("None")
}

#[derive(Default)]
pub struct ListPorts {
    ports: Vec<ListPortInfo>
}

impl ListPorts {
    pub fn new() -> Self {
        let mut ports = ListPorts {
            ports: Vec::new()
        };
        ports.add_ports_matching("/dev/ttyS*");
        ports.add_ports_matching("/dev/ttyUSB*");
        ports.add_ports_matching("/dev/ttyACM*");
        ports.add_ports_matching("/dev/ttyAMA*");
        ports.add_ports_matching("/dev/rfcomm*");
        ports
    }

    fn add_ports_matching(&mut self, pattern: &str) {
        for entry in glob(pattern).unwrap() {
            if let Ok(path_buf) = entry {
                if let Some(info) = ListPortInfo::new(path_buf) {
                    self.ports.push(info);
                }
            }
        }
    }

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
