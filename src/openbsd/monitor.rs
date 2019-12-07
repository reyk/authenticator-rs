/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use std::collections::HashMap;
use std::ffi::{CString, OsStr, OsString};
use std::os::unix::ffi::OsStrExt;
use std::sync::Arc;
use std::thread;
use std::time::Duration;
use std::{fs, io};

use runloop::RunLoop;
use util::from_unix_result;

const POLL_TIMEOUT: u64 = 500;

pub struct Monitor<F>
where
    F: Fn(OsString, &dyn Fn() -> bool) + Sync,
{
    runloops: HashMap<OsString, RunLoop>,
    new_device_cb: Arc<F>,
}

impl<F> Monitor<F>
where
    F: Fn(OsString, &dyn Fn() -> bool) + Send + Sync + 'static,
{
    pub fn new(new_device_cb: F) -> Self {
        Self {
            runloops: HashMap::new(),
            new_device_cb: Arc::new(new_device_cb),
        }
    }

    pub fn run(&mut self, alive: &dyn Fn() -> bool) -> io::Result<()> {
        // Loop until we're stopped by the controlling thread, or fail.
        while alive() {
            // Iterate all existing devices.
            for dev in fs::read_dir("/dev")?.filter_map(|dev| dev.ok()) {
                let path = dev.path();
                let name = path.file_name().unwrap_or(OsStr::new(""));
                if !name.to_string_lossy().starts_with("uhid") {
                    continue;
                }

                let os_path = path.as_os_str().to_os_string();
                let cstr = CString::new(os_path.as_bytes())?;

                // Try to open the device.
                let fd = unsafe { libc::open(cstr.as_ptr(), libc::O_RDWR) };
                match from_unix_result(fd) {
                    Ok(fd) => {
                        // The device is available if it can be opened.
                        unsafe { libc::close(fd) };
                        self.add_device(os_path);
                    }
                    Err(ref err) if err.raw_os_error() == Some(libc::EBUSY) => {
                        // The device is available but currently in use.
                        self.add_device(os_path);
                    }
                    _ => {
                        // libc::ENODEV or any other error.
                        self.remove_device(os_path);
                    }
                }
            }

            thread::sleep(Duration::from_millis(POLL_TIMEOUT));
        }

        // Remove all tracked devices.
        self.remove_all_devices();

        Ok(())
    }

    fn add_device(&mut self, path: OsString) {
        if !self.runloops.contains_key(&path) {
            let f = self.new_device_cb.clone();
            let key = path.clone();

            let runloop = RunLoop::new(move |alive| {
                if alive() {
                    f(path, alive);
                }
            });

            if let Ok(runloop) = runloop {
                self.runloops.insert(key, runloop);
            }
        }
    }

    fn remove_device(&mut self, path: OsString) {
        if let Some(runloop) = self.runloops.remove(&path) {
            runloop.cancel();
        }
    }

    fn remove_all_devices(&mut self) {
        while !self.runloops.is_empty() {
            let path = self.runloops.keys().next().unwrap().clone();
            self.remove_device(path);
        }
    }
}
