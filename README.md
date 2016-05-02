# serial-ports-rs
Rust crate to enumerate serial ports (ala python's serial.tools.list_port.comports)

This crate currently supports Linux and OSX.

Running the ```list_ports.rs``` example on my linux computer with a few USB serial ports plugged in
produces the following output:
```
Device: /dev/ttyUSB0
             name: ttyUSB0
      description: USB-Serial Controller
             hwid: USB VID:PID=067B:2303 LOCATION=3-3.3
        port_type: UsbPort
              vid: 067b
              pid: 2303
    serial_number: None
         location: 3-3.3
     manufacturer: Prolific Technology Inc.
          product: USB-Serial Controller
        interface: None

Device: /dev/ttyACM0
             name: ttyACM0
      description: ttyACM0
             hwid: USB VID:PID=0658:0200 LOCATION=3-3.2
        port_type: UsbPort
              vid: 0658
              pid: 0200
    serial_number: None
         location: 3-3.2
     manufacturer: None
          product: None
        interface: None

Device: /dev/ttyACM1
             name: ttyACM1
      description: Pyboard Virtual Comm Port in FS Mode
             hwid: USB VID:PID=F055:9800 SER=3650326B3432 LOCATION=3-3.4
        port_type: UsbPort
              vid: f055
              pid: 9800
    serial_number: 3650326B3432
         location: 3-3.4
     manufacturer: MicroPython
          product: Pyboard Virtual Comm Port in FS Mode
        interface: None

Device: /dev/rfcomm0
             name: rfcomm0
      description: 
             hwid: 
        port_type: Unknown
```

and on my Mac Mini:

```
Device: /dev/cu.Bluetooth-Incoming-Port
             name: 
      description: n/a
             hwid: n/a
        port_type: NativePort

Device: /dev/cu.Bluetooth-Modem
             name: 
      description: n/a
             hwid: n/a
        port_type: NativePort

Device: /dev/cu.usbmodem26442
             name: 
      description: Pyboard Virtual Comm Port in FS Mode
             hwid: USB VID:PID=F055:9800 SER=3650326B3432 LOCATION=38-4.4
        port_type: UsbPort
              vid: f055
              pid: 9800
    serial_number: 3650326B3432
         location: 38-4.4
     manufacturer: MicroPython
          product: Pyboard Virtual Comm Port in FS Mode
        interface: None

Device: /dev/cu.usbmodem26421
             name: 
      description: 
             hwid: USB VID:PID=0658:0200 LOCATION=38-4.2
        port_type: UsbPort
              vid: 0658
              pid: 0200
    serial_number: None
         location: 38-4.2
     manufacturer: None
          product: None
        interface: None
```
