#[cfg(all(target_os = "linux", feature = "binding-recompile"))]
extern crate bindgen;

#[cfg(all(target_os = "linux", feature = "binding-recompile"))]
use std::path::PathBuf;

#[cfg(all(target_os = "openbsd", feature = "binding-recompile"))]
use std::path::PathBuf;

#[cfg(any(
    all(not(target_os = "linux"), not(target_os = "openbsd")),
    all(not(feature = "binding-recompile"), not(target_os = "openbsd")),
))]
fn main() {}

#[cfg(all(target_os = "linux", feature = "binding-recompile"))]
fn main() {
    let bindings = bindgen::Builder::default()
        .header("src/linux/hidwrapper.h")
        .whitelist_var("_HIDIOCGRDESCSIZE")
        .whitelist_var("_HIDIOCGRDESC")
        .generate()
        .expect("Unable to get hidraw bindings");

    let out_path = PathBuf::new();
    let name = if cfg!(target_arch = "x86") {
        "ioctl_x86.rs"
    } else if cfg!(target_arch = "x86_64") {
        "ioctl_x86_64.rs"
    } else if cfg!(all(target_arch = "mips", target_endian = "big")) {
        "ioctl_mipsbe.rs"
    } else if cfg!(all(target_arch = "mips", target_endian = "little")) {
        "ioctl_mipsle.rs"
    } else if cfg!(all(target_arch = "powerpc", target_endian = "little")) {
        "ioctl_powerpcle.rs"
    } else if cfg!(all(target_arch = "powerpc", target_endian = "big")) {
        "ioctl_powerpcbe.rs"
    } else if cfg!(all(target_arch = "powerpc64", target_endian = "little")) {
        "ioctl_powerpc64le.rs"
    } else if cfg!(all(target_arch = "powerpc64", target_endian = "big")) {
        "ioctl_powerpc64be.rs"
    } else if cfg!(all(target_arch = "arm", target_endian = "little")) {
        "ioctl_armle.rs"
    } else if cfg!(all(target_arch = "arm", target_endian = "big")) {
        "ioctl_armbe.rs"
    } else if cfg!(all(target_arch = "aarch64", target_endian = "little")) {
        "ioctl_aarch64le.rs"
    } else if cfg!(all(target_arch = "aarch64", target_endian = "big")) {
        "ioctl_aarch64be.rs"
    } else {
        panic!("architecture not supported");
    };
    bindings
        .write_to_file(out_path.join("src").join("linux").join(name))
        .expect("Couldn't write hidraw bindings");
}

#[cfg(all(target_os = "openbsd", feature = "binding-recompile"))]
fn main() {
    println!("cargo:rustc-link-lib=usbhid");

    let bindings = bindgen::Builder::default()
        .header("src/openbsd/wrapper.h")
        .whitelist_var("^IOC.*")
        .whitelist_type("^usb_device_info")
        .whitelist_type("^hid_.*")
        .whitelist_function("^hid_.*")
        .generate()
        .expect("Unable to get usbhid bindings");

    let out_path = PathBuf::from("src/openbsd/bindings.rs");
    bindings
        .write_to_file(out_path)
        .expect("Couldn't write usbhid bindings");
}

#[cfg(all(target_os = "openbsd", not(feature = "binding-recompile")))]
fn main() {
    println!("cargo:rustc-link-search=native=/usr/lib");
    println!("cargo:rustc-link-lib=static=usbhid");
}
