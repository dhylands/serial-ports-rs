#![allow(non_upper_case_globals)]

use cf::*;
use IOKit_sys::*;
use libc::{ c_char, c_void };
use mach::port::{ MACH_PORT_NULL };
use mach::kern_return::KERN_SUCCESS;
use std::ffi::{ CStr, CString };
use std::mem;
use std::path::PathBuf;
use std::ptr;

pub fn get_ioservices_by_type(service_type: *const c_char) -> Vec<io_object_t> {

    let classes_to_match = unsafe { IOServiceMatching(service_type) };
    if classes_to_match.is_null() {
        panic!("IOServiceMatching returned a NULL dictionary.");
    }

    let mut matching_services: io_iterator_t = unsafe { mem::uninitialized() };

    let kern_result = unsafe { IOServiceGetMatchingServices(kIOMasterPortDefault, classes_to_match, &mut matching_services) };
    if kern_result != KERN_SUCCESS {
        panic!("ERROR: {}", kern_result);
    }

    let mut services = Vec::new();
    loop {
        let service = unsafe { IOIteratorNext(matching_services) };
        if service == MACH_PORT_NULL {
            break;
        }
        services.push(service);
    }
    unsafe { IOObjectRelease(matching_services); }

    services
}

fn get_parent_device_by_type(device: io_object_t, parent_type: *const c_char) -> Option<io_registry_entry_t> {
    let mut device = device;
    loop {
        let mut class_name: [c_char; 128] = unsafe { mem::uninitialized() };
        unsafe { IOObjectGetClass(device, &mut class_name[0]) };
        if unsafe { CStr::from_ptr(&class_name[0]) == CStr::from_ptr(parent_type) } {
            return Some(device);
        }
        let mut parent: io_registry_entry_t = unsafe { mem::uninitialized() };
        if unsafe { IORegistryEntryGetParentEntry(device, kIOServiceClass(), &mut parent) != KERN_SUCCESS } {
            return None;
        }
        device = parent;
    }
}

fn get_int_property(device_type: io_registry_entry_t, property: &str, cf_number_type: CFNumberType) -> u32 {
    let key = unsafe { CFStringCreateWithCString(kCFAllocatorDefault, CString::new(property).unwrap().as_ptr(), kCFStringEncodingUTF8) };
    let container = unsafe { IORegistryEntryCreateCFProperty(device_type, key, kCFAllocatorDefault, 0) };
    if container == ptr::null() {
        return 0;
    }
    let num = match cf_number_type {
        kCFNumberSInt16Type => {
            let mut num:u16 = 0;
            let num_ptr: *mut c_void = &mut num as *mut _ as *mut c_void;
            unsafe { CFNumberGetValue(container as CFNumberRef, cf_number_type, num_ptr) };
            num as u32
        }
        kCFNumberSInt32Type => {
            let mut num:u32 = 0;
            let num_ptr: *mut c_void = &mut num as *mut _ as *mut c_void;
            unsafe { CFNumberGetValue(container as CFNumberRef, cf_number_type, num_ptr) };
            num
        }
        _ => 0
    };
    unsafe { CFRelease(container) };

    num
}

fn get_string_property(device_type: io_registry_entry_t, property: &str) -> Option<String> {
    let key = unsafe { CFStringCreateWithCString(kCFAllocatorDefault, CString::new(property).unwrap().as_ptr(), kCFStringEncodingUTF8) };
    let container = unsafe { IORegistryEntryCreateCFProperty(device_type, key, kCFAllocatorDefault, 0) };
    if container == ptr::null() {
        return None;
    }

    let str_ptr = unsafe { CFStringGetCStringPtr(container as CFStringRef, kCFStringEncodingMacRoman) };
    if str_ptr == ptr::null() {
        unsafe { CFRelease(container) };
        return None;
    }
    let opt_str = unsafe { CStr::from_ptr(str_ptr) }.to_str().ok().map(String::from);

    unsafe { CFRelease(container) };

    opt_str
}

fn location_to_string(location_id: u32) -> Option<String> {
    let mut location_id = location_id;
    let mut loc_str = format!("{}-{}", location_id >> 24, (location_id >> 20) & 0xf);
    location_id <<= 4;
    while (location_id & 0xf00000) != 0 {
        loc_str.push_str(".");
        loc_str.push_str(&format!("{}", (location_id >> 20) & 0xf));
        location_id <<= 4;
    }
    return Some(loc_str);
}

fn get_interface(location_id: u32) -> Option<String> {
    let services = get_ioservices_by_type(kIOSerialBSDServiceValue());
    for service in services {
        if let Some(_) = get_string_property(service, "IOCalloutDevice") {
            if let Some(usb_device) = get_parent_device_by_type(service, kIOUSBInterfaceClassName()) {
                let intf_name = get_string_property(usb_device, "USB Interface Name");
                let intf_location = get_int_property(usb_device, "locationID", kCFNumberSInt32Type);
                if intf_location == location_id {
                    return intf_name;
                }
            }
        }
    }
    None
}

impl ::ListPortInfo {
    fn new(dev_name: &str, service: io_object_t) -> Self {
        if let Some(usb_device) = get_parent_device_by_type(service, kIOUSBDeviceClassName()) {

            let location_id = get_int_property(usb_device, "locationID", kCFNumberSInt32Type);

            let info = ::UsbPortInfo {
                vid: get_int_property(usb_device, "idVendor", kCFNumberSInt16Type) as u16,
                pid: get_int_property(usb_device, "idProduct", kCFNumberSInt16Type) as u16,
                serial_number: get_string_property(usb_device, "USB Serial Number"),
                location: location_to_string(location_id),
                manufacturer: get_string_property(usb_device, "USB Vendor Name"),
                product: get_string_property(usb_device, "USB Product Name"),
                interface: get_interface(location_id),
            };

            ::ListPortInfo {
                device: PathBuf::from(dev_name),
                name: "".to_owned(),
                description: info.description(""),
                hwid: info.hwid(),
                port_type: ::ListPortType::UsbPort(::UsbPortInfo {
                    vid: get_int_property(usb_device, "idVendor", kCFNumberSInt16Type) as u16,
                    pid: get_int_property(usb_device, "idProduct", kCFNumberSInt16Type) as u16,
                    serial_number: get_string_property(usb_device, "USB Serial Number"),
                    location: location_to_string(location_id),
                    manufacturer: get_string_property(usb_device, "USB Vendor Name"),
                    product: get_string_property(usb_device, "USB Product Name"),
                    interface: None
                }),
            }
        } else {
            ::ListPortInfo {
                device: PathBuf::from(dev_name),
                name: "".to_owned(),
                description: "n/a".to_owned(),
                hwid: "n/a".to_owned(),
                port_type: ::ListPortType::NativePort,
            }
        }
    }
}

impl ::ListPorts {
    pub fn new() -> Self {
        let mut list_ports = ::ListPorts {
            ports: Vec::new()
        };
        let services = get_ioservices_by_type(kIOSerialBSDServiceValue());
        for service in services {
            if let Some(device) = get_string_property(service, "IOCalloutDevice") {
                list_ports.ports.push(::ListPortInfo::new(&device, service));
            }
        }
        list_ports
    }
}
