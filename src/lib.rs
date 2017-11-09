extern crate libc;


use libc::c_int;


extern "C" {
    pub fn esaxx_c8i8  (t: *const  u8, sa: *mut  i8, l: *mut  i8, r: *mut  i8, d: *mut  i8, n:  i8, k:  i8, m: *mut  i8) -> c_int;
    pub fn esaxx_c8i16 (t: *const  u8, sa: *mut i16, l: *mut i16, r: *mut i16, d: *mut i16, n: i16, k: i16, m: *mut i16) -> c_int;
    pub fn esaxx_c8i32 (t: *const  u8, sa: *mut i32, l: *mut i32, r: *mut i32, d: *mut i32, n: i32, k: i32, m: *mut i32) -> c_int;
    pub fn esaxx_c8i64 (t: *const  u8, sa: *mut i64, l: *mut i64, r: *mut i64, d: *mut i64, n: i64, k: i64, m: *mut i64) -> c_int;
    pub fn esaxx_c16i8 (t: *const u16, sa: *mut  i8, l: *mut  i8, r: *mut  i8, d: *mut  i8, n:  i8, k:  i8, m: *mut  i8) -> c_int;
    pub fn esaxx_c16i16(t: *const u16, sa: *mut i16, l: *mut i16, r: *mut i16, d: *mut i16, n: i16, k: i16, m: *mut i16) -> c_int;
    pub fn esaxx_c16i32(t: *const u16, sa: *mut i32, l: *mut i32, r: *mut i32, d: *mut i32, n: i32, k: i32, m: *mut i32) -> c_int;
    pub fn esaxx_c16i64(t: *const u16, sa: *mut i64, l: *mut i64, r: *mut i64, d: *mut i64, n: i64, k: i64, m: *mut i64) -> c_int;
    pub fn esaxx_c32i8 (t: *const u32, sa: *mut  i8, l: *mut  i8, r: *mut  i8, d: *mut  i8, n:  i8, k:  i8, m: *mut  i8) -> c_int;
    pub fn esaxx_c32i16(t: *const u32, sa: *mut i16, l: *mut i16, r: *mut i16, d: *mut i16, n: i16, k: i16, m: *mut i16) -> c_int;
    pub fn esaxx_c32i32(t: *const u32, sa: *mut i32, l: *mut i32, r: *mut i32, d: *mut i32, n: i32, k: i32, m: *mut i32) -> c_int;
    pub fn esaxx_c32i64(t: *const u32, sa: *mut i64, l: *mut i64, r: *mut i64, d: *mut i64, n: i64, k: i64, m: *mut i64) -> c_int;
    pub fn esaxx_c64i8 (t: *const u64, sa: *mut  i8, l: *mut  i8, r: *mut  i8, d: *mut  i8, n:  i8, k:  i8, m: *mut  i8) -> c_int;
    pub fn esaxx_c64i16(t: *const u64, sa: *mut i16, l: *mut i16, r: *mut i16, d: *mut i16, n: i16, k: i16, m: *mut i16) -> c_int;
    pub fn esaxx_c64i32(t: *const u64, sa: *mut i32, l: *mut i32, r: *mut i32, d: *mut i32, n: i32, k: i32, m: *mut i32) -> c_int;
    pub fn esaxx_c64i64(t: *const u64, sa: *mut i64, l: *mut i64, r: *mut i64, d: *mut i64, n: i64, k: i64, m: *mut i64) -> c_int;
}
