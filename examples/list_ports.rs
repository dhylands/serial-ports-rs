extern crate serial_ports;

use serial_ports::ListPorts;

fn main() {
    for port in ListPorts::new().iter() {
        port.dump();
        println!("");
    }
}
