/* automatically generated by rust-bindgen */

pub const IOCPARM_MASK: u32 = 8191;
pub type __uint8_t = ::std::os::raw::c_uchar;
pub type __uint16_t = ::std::os::raw::c_ushort;
pub type __int32_t = ::std::os::raw::c_int;
pub type __uint32_t = ::std::os::raw::c_uint;
pub type u_int8_t = __uint8_t;
pub type u_int16_t = __uint16_t;
pub type u_int32_t = __uint32_t;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct usb_device_info {
    pub udi_bus: u_int8_t,
    pub udi_addr: u_int8_t,
    pub udi_product: [::std::os::raw::c_char; 127usize],
    pub udi_vendor: [::std::os::raw::c_char; 127usize],
    pub udi_release: [::std::os::raw::c_char; 8usize],
    pub udi_productNo: u_int16_t,
    pub udi_vendorNo: u_int16_t,
    pub udi_releaseNo: u_int16_t,
    pub udi_class: u_int8_t,
    pub udi_subclass: u_int8_t,
    pub udi_protocol: u_int8_t,
    pub udi_config: u_int8_t,
    pub udi_speed: u_int8_t,
    pub udi_port: u_int8_t,
    pub udi_power: ::std::os::raw::c_int,
    pub udi_nports: ::std::os::raw::c_int,
    pub udi_devnames: [[::std::os::raw::c_char; 16usize]; 4usize],
    pub udi_ports: [u_int32_t; 16usize],
    pub udi_serial: [::std::os::raw::c_char; 127usize],
}
#[test]
fn bindgen_test_layout_usb_device_info() {
    assert_eq!(
        ::std::mem::size_of::<usb_device_info>(),
        540usize,
        concat!("Size of: ", stringify!(usb_device_info))
    );
    assert_eq!(
        ::std::mem::align_of::<usb_device_info>(),
        4usize,
        concat!("Alignment of ", stringify!(usb_device_info))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<usb_device_info>())).udi_bus as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(usb_device_info),
            "::",
            stringify!(udi_bus)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<usb_device_info>())).udi_addr as *const _ as usize },
        1usize,
        concat!(
            "Offset of field: ",
            stringify!(usb_device_info),
            "::",
            stringify!(udi_addr)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<usb_device_info>())).udi_product as *const _ as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(usb_device_info),
            "::",
            stringify!(udi_product)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<usb_device_info>())).udi_vendor as *const _ as usize },
        129usize,
        concat!(
            "Offset of field: ",
            stringify!(usb_device_info),
            "::",
            stringify!(udi_vendor)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<usb_device_info>())).udi_release as *const _ as usize },
        256usize,
        concat!(
            "Offset of field: ",
            stringify!(usb_device_info),
            "::",
            stringify!(udi_release)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<usb_device_info>())).udi_productNo as *const _ as usize },
        264usize,
        concat!(
            "Offset of field: ",
            stringify!(usb_device_info),
            "::",
            stringify!(udi_productNo)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<usb_device_info>())).udi_vendorNo as *const _ as usize },
        266usize,
        concat!(
            "Offset of field: ",
            stringify!(usb_device_info),
            "::",
            stringify!(udi_vendorNo)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<usb_device_info>())).udi_releaseNo as *const _ as usize },
        268usize,
        concat!(
            "Offset of field: ",
            stringify!(usb_device_info),
            "::",
            stringify!(udi_releaseNo)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<usb_device_info>())).udi_class as *const _ as usize },
        270usize,
        concat!(
            "Offset of field: ",
            stringify!(usb_device_info),
            "::",
            stringify!(udi_class)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<usb_device_info>())).udi_subclass as *const _ as usize },
        271usize,
        concat!(
            "Offset of field: ",
            stringify!(usb_device_info),
            "::",
            stringify!(udi_subclass)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<usb_device_info>())).udi_protocol as *const _ as usize },
        272usize,
        concat!(
            "Offset of field: ",
            stringify!(usb_device_info),
            "::",
            stringify!(udi_protocol)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<usb_device_info>())).udi_config as *const _ as usize },
        273usize,
        concat!(
            "Offset of field: ",
            stringify!(usb_device_info),
            "::",
            stringify!(udi_config)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<usb_device_info>())).udi_speed as *const _ as usize },
        274usize,
        concat!(
            "Offset of field: ",
            stringify!(usb_device_info),
            "::",
            stringify!(udi_speed)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<usb_device_info>())).udi_port as *const _ as usize },
        275usize,
        concat!(
            "Offset of field: ",
            stringify!(usb_device_info),
            "::",
            stringify!(udi_port)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<usb_device_info>())).udi_power as *const _ as usize },
        276usize,
        concat!(
            "Offset of field: ",
            stringify!(usb_device_info),
            "::",
            stringify!(udi_power)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<usb_device_info>())).udi_nports as *const _ as usize },
        280usize,
        concat!(
            "Offset of field: ",
            stringify!(usb_device_info),
            "::",
            stringify!(udi_nports)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<usb_device_info>())).udi_devnames as *const _ as usize },
        284usize,
        concat!(
            "Offset of field: ",
            stringify!(usb_device_info),
            "::",
            stringify!(udi_devnames)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<usb_device_info>())).udi_ports as *const _ as usize },
        348usize,
        concat!(
            "Offset of field: ",
            stringify!(usb_device_info),
            "::",
            stringify!(udi_ports)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<usb_device_info>())).udi_serial as *const _ as usize },
        412usize,
        concat!(
            "Offset of field: ",
            stringify!(usb_device_info),
            "::",
            stringify!(udi_serial)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct report_desc {
    _unused: [u8; 0],
}
pub type report_desc_t = *mut report_desc;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct hid_data {
    _unused: [u8; 0],
}
pub type hid_data_t = *mut hid_data;
pub const hid_kind_hid_input: hid_kind = 0;
pub const hid_kind_hid_output: hid_kind = 1;
pub const hid_kind_hid_feature: hid_kind = 2;
pub const hid_kind_hid_collection: hid_kind = 3;
pub const hid_kind_hid_endcollection: hid_kind = 4;
pub type hid_kind = u32;
pub use self::hid_kind as hid_kind_t;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct hid_item {
    pub _usage_page: u32,
    pub logical_minimum: i32,
    pub logical_maximum: i32,
    pub physical_minimum: i32,
    pub physical_maximum: i32,
    pub unit_exponent: i32,
    pub unit: i32,
    pub report_size: i32,
    pub report_ID: i32,
    pub report_count: i32,
    pub usage: u32,
    pub usage_minimum: i32,
    pub usage_maximum: i32,
    pub designator_index: i32,
    pub designator_minimum: i32,
    pub designator_maximum: i32,
    pub string_index: i32,
    pub string_minimum: i32,
    pub string_maximum: i32,
    pub set_delimiter: i32,
    pub collection: i32,
    pub collevel: ::std::os::raw::c_int,
    pub kind: hid_kind,
    pub flags: u32,
    pub pos: u32,
    pub next: *mut hid_item,
}
#[test]
fn bindgen_test_layout_hid_item() {
    assert_eq!(
        ::std::mem::size_of::<hid_item>(),
        112usize,
        concat!("Size of: ", stringify!(hid_item))
    );
    assert_eq!(
        ::std::mem::align_of::<hid_item>(),
        8usize,
        concat!("Alignment of ", stringify!(hid_item))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<hid_item>()))._usage_page as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(hid_item),
            "::",
            stringify!(_usage_page)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<hid_item>())).logical_minimum as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(hid_item),
            "::",
            stringify!(logical_minimum)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<hid_item>())).logical_maximum as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(hid_item),
            "::",
            stringify!(logical_maximum)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<hid_item>())).physical_minimum as *const _ as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(hid_item),
            "::",
            stringify!(physical_minimum)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<hid_item>())).physical_maximum as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(hid_item),
            "::",
            stringify!(physical_maximum)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<hid_item>())).unit_exponent as *const _ as usize },
        20usize,
        concat!(
            "Offset of field: ",
            stringify!(hid_item),
            "::",
            stringify!(unit_exponent)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<hid_item>())).unit as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(hid_item),
            "::",
            stringify!(unit)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<hid_item>())).report_size as *const _ as usize },
        28usize,
        concat!(
            "Offset of field: ",
            stringify!(hid_item),
            "::",
            stringify!(report_size)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<hid_item>())).report_ID as *const _ as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(hid_item),
            "::",
            stringify!(report_ID)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<hid_item>())).report_count as *const _ as usize },
        36usize,
        concat!(
            "Offset of field: ",
            stringify!(hid_item),
            "::",
            stringify!(report_count)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<hid_item>())).usage as *const _ as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(hid_item),
            "::",
            stringify!(usage)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<hid_item>())).usage_minimum as *const _ as usize },
        44usize,
        concat!(
            "Offset of field: ",
            stringify!(hid_item),
            "::",
            stringify!(usage_minimum)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<hid_item>())).usage_maximum as *const _ as usize },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(hid_item),
            "::",
            stringify!(usage_maximum)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<hid_item>())).designator_index as *const _ as usize },
        52usize,
        concat!(
            "Offset of field: ",
            stringify!(hid_item),
            "::",
            stringify!(designator_index)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<hid_item>())).designator_minimum as *const _ as usize },
        56usize,
        concat!(
            "Offset of field: ",
            stringify!(hid_item),
            "::",
            stringify!(designator_minimum)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<hid_item>())).designator_maximum as *const _ as usize },
        60usize,
        concat!(
            "Offset of field: ",
            stringify!(hid_item),
            "::",
            stringify!(designator_maximum)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<hid_item>())).string_index as *const _ as usize },
        64usize,
        concat!(
            "Offset of field: ",
            stringify!(hid_item),
            "::",
            stringify!(string_index)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<hid_item>())).string_minimum as *const _ as usize },
        68usize,
        concat!(
            "Offset of field: ",
            stringify!(hid_item),
            "::",
            stringify!(string_minimum)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<hid_item>())).string_maximum as *const _ as usize },
        72usize,
        concat!(
            "Offset of field: ",
            stringify!(hid_item),
            "::",
            stringify!(string_maximum)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<hid_item>())).set_delimiter as *const _ as usize },
        76usize,
        concat!(
            "Offset of field: ",
            stringify!(hid_item),
            "::",
            stringify!(set_delimiter)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<hid_item>())).collection as *const _ as usize },
        80usize,
        concat!(
            "Offset of field: ",
            stringify!(hid_item),
            "::",
            stringify!(collection)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<hid_item>())).collevel as *const _ as usize },
        84usize,
        concat!(
            "Offset of field: ",
            stringify!(hid_item),
            "::",
            stringify!(collevel)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<hid_item>())).kind as *const _ as usize },
        88usize,
        concat!(
            "Offset of field: ",
            stringify!(hid_item),
            "::",
            stringify!(kind)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<hid_item>())).flags as *const _ as usize },
        92usize,
        concat!(
            "Offset of field: ",
            stringify!(hid_item),
            "::",
            stringify!(flags)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<hid_item>())).pos as *const _ as usize },
        96usize,
        concat!(
            "Offset of field: ",
            stringify!(hid_item),
            "::",
            stringify!(pos)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<hid_item>())).next as *const _ as usize },
        104usize,
        concat!(
            "Offset of field: ",
            stringify!(hid_item),
            "::",
            stringify!(next)
        )
    );
}
pub type hid_item_t = hid_item;
extern "C" {
    pub fn hid_get_report_desc(file: ::std::os::raw::c_int) -> report_desc_t;
}
extern "C" {
    pub fn hid_use_report_desc(
        data: *mut ::std::os::raw::c_uchar,
        size: ::std::os::raw::c_uint,
    ) -> report_desc_t;
}
extern "C" {
    pub fn hid_dispose_report_desc(arg1: report_desc_t);
}
extern "C" {
    pub fn hid_start_parse(
        d: report_desc_t,
        kindset: ::std::os::raw::c_int,
        id: ::std::os::raw::c_int,
    ) -> hid_data_t;
}
extern "C" {
    pub fn hid_end_parse(s: hid_data_t);
}
extern "C" {
    pub fn hid_get_item(s: hid_data_t, h: *mut hid_item_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn hid_report_size(
        d: report_desc_t,
        k: hid_kind,
        id: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn hid_locate(
        d: report_desc_t,
        usage: ::std::os::raw::c_uint,
        k: hid_kind,
        h: *mut hid_item_t,
        id: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn hid_usage_page(i: ::std::os::raw::c_int) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn hid_usage_in_page(u: ::std::os::raw::c_uint) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn hid_init(file: *const ::std::os::raw::c_char);
}
extern "C" {
    pub fn hid_start(file: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn hid_parse_usage_in_page(name: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn hid_parse_usage_page(name: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn hid_get_data(p: *const ::std::os::raw::c_void, h: *const hid_item_t) -> i32;
}
extern "C" {
    pub fn hid_set_data(p: *mut ::std::os::raw::c_void, h: *const hid_item_t, data: i32);
}
