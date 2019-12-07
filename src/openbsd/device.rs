/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

extern crate libc;

use std::ffi::{CString, OsString};
use std::io::{Read, Result, Write};
use std::mem;
use std::os::unix::prelude::*;

use consts::CID_BROADCAST;
use platform::usbhid;
use u2ftypes::U2FDevice;
use util::{from_unix_result, io_err};

#[derive(Debug)]
pub struct Device {
    path: OsString,
    fd: libc::c_int,
    cid: [u8; 4],
    usbhid: usbhid::UsbHid,
}

impl Device {
    pub fn new(path: OsString) -> Result<Self> {
        let cstr = CString::new(path.as_bytes())?;
        let fd = unsafe { libc::open(cstr.as_ptr(), libc::O_RDWR) };
        let fd = from_unix_result(fd)?;
        debug!("device found: {:?} fd {}", path, fd);
        Ok(Self {
            path,
            fd,
            cid: CID_BROADCAST,
            usbhid: Default::default(),
        })
    }

    pub fn is_u2f(&mut self) -> bool {
        if !usbhid::is_u2f_device(self.fd) {
            return false;
        }
        debug!("device {:?} is U2F/FIDO", self.path);
        match usbhid::open_u2f_device(self.fd) {
            Ok(usbhid) => {
                self.usbhid = usbhid;
            }
            Err(err) => {
                debug!("device {:?} error: {}", self.path, err);
                return false;
            }
        }

        // From OpenBSD's libfido2 in 6.6-current:
        // "OpenBSD (as of 201910) has a bug that causes it to lose
        // track of the DATA0/DATA1 sequence toggle across uhid device
        // open and close. This is a terrible hack to work around it."
        match self.ping() {
            Ok(_) => true,
            Err(err) => {
                debug!("device {:?} is not responding: {}", self.path, err);
                false
            }
        }
    }

    fn ping(&mut self) -> Result<()> {
        let capacity = 256;

        if capacity < self.usbhid.out_len + 1 {
            return Err(io_err("reported out len too small"));
        }

        for _ in 0..10 {
            let mut data = vec![0u8; capacity];

            // Send 1 byte ping
            data[1..=7].copy_from_slice(&[0xff, 0xff, 0xff, 0xff, 0x81, 0, 1]);
            self.write_all(&data[0..=self.usbhid.out_len])?;

            // Wait for response
            let mut pfd: libc::pollfd = unsafe { mem::zeroed() };
            pfd.fd = self.fd;
            pfd.events = libc::POLLIN;
            if from_unix_result(unsafe { libc::poll(&mut pfd, 1, 100) })? == 0 {
                debug!("device {:?} timeout", self.path);
                continue;
            }

            // Read response
            self.read_exact(&mut data[0..self.usbhid.out_len])?;

            return Ok(());
        }

        Err(io_err("no response from device"))
    }
}

impl Drop for Device {
    fn drop(&mut self) {
        // Close the fd, ignore any errors.
        let _ = unsafe { libc::close(self.fd) };
        debug!("device {:?} closed", self.path);
    }
}

impl PartialEq for Device {
    fn eq(&self, other: &Device) -> bool {
        self.path == other.path
    }
}

impl Read for Device {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        if buf.len() != self.usbhid.in_len {
            return Err(io_err("invalid read length"));
        }
        let buf_ptr = buf.as_mut_ptr() as *mut libc::c_void;
        let rv = unsafe { libc::read(self.fd, buf_ptr, buf.len()) };
        if from_unix_result(rv as usize)? != buf.len() {
            return Err(io_err("invalid read result"));
        }
        Ok(buf.len())
    }
}

impl Write for Device {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        if buf.len() != self.usbhid.out_len + 1 {
            return Err(io_err("invalid write length"));
        }
        let data = &buf[1..];
        let data_ptr = data.as_ptr() as *const libc::c_void;
        let rv = unsafe { libc::write(self.fd, data_ptr, data.len()) };
        if from_unix_result(rv as usize)? != data.len() {
            return Err(io_err("invalid write result"));
        }
        Ok(buf.len())
    }

    fn flush(&mut self) -> Result<()> {
        Ok(())
    }
}

impl U2FDevice for Device {
    fn get_cid(&self) -> &[u8; 4] {
        &self.cid
    }

    fn set_cid(&mut self, cid: [u8; 4]) {
        self.cid = cid;
    }
}
