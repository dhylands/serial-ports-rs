# serial-ports-rs
Rust crate to enumerate serial ports (ala python's serial.tools.list_port.comports)

Currently only linux is supports, but I plan on adding implementations for OSX and Windows.

Running the ```list_ports.rs``` example on my computer with a few USB serial ports plugged in
produces the following output:
```
Device: /dev/ttyUSB0
             name: ttyUSB0
      device_path: /sys/devices/pci0000:00/0000:00:14.0/usb3/3-3/3-3.3/3-3.3:1.0/ttyUSB0
        subsystem: usb-serial
  usb_device_path: /sys/devices/pci0000:00/0000:00:14.0/usb3/3-3/3-3.3
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
      device_path: /sys/devices/pci0000:00/0000:00:14.0/usb3/3-3/3-3.2/3-3.2:1.0
        subsystem: usb
  usb_device_path: /sys/devices/pci0000:00/0000:00:14.0/usb3/3-3/3-3.2
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
      device_path: /sys/devices/pci0000:00/0000:00:14.0/usb3/3-3/3-3.4/3-3.4:1.1
        subsystem: usb
  usb_device_path: /sys/devices/pci0000:00/0000:00:14.0/usb3/3-3/3-3.4
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
      device_path: None
        subsystem: None
  usb_device_path: None
      description: 
             hwid: 
        port_type: Unknown
```
