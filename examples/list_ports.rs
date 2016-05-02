extern crate serial_ports;

use serial_ports::ListPorts;

fn main() {
    let mut port_found = false;

    for port in ListPorts::new().iter() {
	port_found = true;
        port.dump();
        println!("");
    }
    if !port_found {
        println!("No ports found");
    }
}
