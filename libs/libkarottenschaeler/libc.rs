pub use core::ffi::{CStr, c_void, c_int, c_char, c_long};
pub type c_size_t = usize;

pub const _SC_PAGE_SIZE: i32 = 30;
pub const PROT_READ: i32 = 0x1;
pub const PROT_WRITE: i32 = 0x2;

pub const MAP_SHARED: i32 = 0x1;
pub const MAP_PRIVATE: i32 = 0x2;
pub const MAP_ANONYMOUS: i32 = 0x20;

extern "C" {
    pub fn sysconf(name: i32) -> c_long;
    pub fn mmap(
        addr: *mut c_void,
        len: c_size_t,
        prot: c_int,
        flags: c_int,
        fd: c_int,
        offset: c_long,
    ) -> *mut c_void;
    pub fn munmap(addr: *mut c_void, len: c_size_t) -> c_int;
}

#[macro_export]
macro_rules! printf {
    ($fmt:literal $($args:tt)*) => {{
        use core::ffi::c_int;
        use core::ffi::c_char;
        extern "C" {
            #[link_name = "printf"]
            pub fn printf_raw(fmt: *const c_char, ...) -> c_int;
        }
        printf_raw($fmt.as_ptr() $($args)*)
        }};
}
