/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

extern crate libc;

use std::ffi::CStr;
use std::io::Result;
use std::mem;
use std::os::unix::io::RawFd;

use platform::wrapper::*;
use util::{from_unix_ptr, from_unix_result, io_err};

#[derive(Default, Debug)]
pub struct UsbHid {
    pub in_len: usize,
    pub out_len: usize,
}

const IOR: u32 = 0x4000_0000;
const IOC_TYPESHIFT: u32 = 8;
const IOC_SIZESHIFT: u32 = 16;

macro_rules! ioctl {
    ($dir:expr, $name:ident, $ioty:expr, $nr:expr; $ty:ty) => {
        pub unsafe fn $name(fd: libc::c_int, val: *mut $ty) -> Result<libc::c_int> {
            let ioc = ($dir as u32)
                | ((mem::size_of::<$ty>() as u32 & IOCPARM_MASK) << IOC_SIZESHIFT)
                | (($ioty as u32) << IOC_TYPESHIFT)
                | ($nr as u32);
            from_unix_result(libc::ioctl(fd, ioc as libc::c_ulong, val))
        }
    };
}

// https://github.com/openbsd/src/blob/master/sys/dev/usb/usb.h
ioctl!(IOR, usb_get_deviceinfo_ioctl, b'U', 112; usb_device_info);
ioctl!(IOR, usb_get_report_id_ioctl, b'U', 25; i32);

unsafe fn usbhid_is_u2f(fd: RawFd) -> Result<bool> {
    // Get USB device info to make sure that it's a valid USB device.
    let mut udi: usb_device_info = mem::zeroed();
    let _ = usb_get_deviceinfo_ioctl(fd, &mut udi)?;

    let vendor = CStr::from_ptr(udi.udi_vendor.as_ptr()).to_string_lossy();
    let product = CStr::from_ptr(udi.udi_product.as_ptr()).to_string_lossy();
    debug!(
        "device vendor {} product {} ({:04x}:{:04x})",
        vendor, product, udi.udi_vendorNo, udi.udi_productNo
    );

    // Get report descriptor.
    let desc = from_unix_ptr(hid_get_report_desc(fd))?;

    // Start parsing of the HID collection.
    let kindset = 1 << hid_kind_hid_collection;
    let hdata = match from_unix_ptr(hid_start_parse(desc, kindset, -1)) {
        Ok(hdata) => hdata,
        Err(err) => {
            hid_dispose_report_desc(desc);
            return Err(err);
        }
    };

    // Find U2F/FIDO key.
    let mut is_fido = false;
    loop {
        let mut hitem: hid_item = mem::zeroed();
        if hid_get_item(hdata, &mut hitem) <= 0 {
            break;
        }
        if (hitem._usage_page & 0xFFFF_0000) == 0xF1D0_0000 {
            is_fido = true;
        }
    }

    // Cleanup.
    hid_end_parse(hdata);
    hid_dispose_report_desc(desc);

    Ok(is_fido)
}

unsafe fn usbhid_open_u2f(fd: RawFd) -> Result<UsbHid> {
    let max_report_len = 1024; // this should be size of ucr_data

    // Get USB device report ID.
    let mut usb_report_id = 0;
    let _ = usb_get_report_id_ioctl(fd, &mut usb_report_id)?;

    // Get report descriptor.
    let desc = from_unix_ptr(hid_get_report_desc(fd))?;

    // Get the input report size.
    let in_len = match hid_report_size(desc, hid_kind_hid_input, usb_report_id) {
        len if len > 0 && len <= max_report_len => len as usize,
        _ => {
            hid_dispose_report_desc(desc);
            return Err(io_err("bad output report size"));
        }
    };

    // Get the output report size.
    let out_len = match hid_report_size(desc, hid_kind_hid_output, usb_report_id) {
        len if len > 0 && len <= max_report_len => len as usize,
        _ => {
            hid_dispose_report_desc(desc);
            return Err(io_err("bad output report size"));
        }
    };

    // Cleanup.
    hid_dispose_report_desc(desc);

    Ok(UsbHid { in_len, out_len })
}

pub fn is_u2f_device(fd: RawFd) -> bool {
    match unsafe { usbhid_is_u2f(fd) } {
        Ok(val) => val,
        Err(err) => {
            debug!("usbhid error {}", err);
            false
        }
    }
}

pub fn open_u2f_device(fd: RawFd) -> Result<UsbHid> {
    unsafe { usbhid_open_u2f(fd) }
}
