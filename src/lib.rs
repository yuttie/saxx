extern crate libc;


use libc::c_int;


extern {
    pub fn esaxx_c8i8  (t: *const  u8, sa: *const  i8, l: *const  i8, r: *const  i8, d: *const  i8, n:  i8, k:  i8, m: *mut  i8) -> c_int;
    pub fn esaxx_c8i16 (t: *const  u8, sa: *const i16, l: *const i16, r: *const i16, d: *const i16, n: i16, k: i16, m: *mut i16) -> c_int;
    pub fn esaxx_c8i32 (t: *const  u8, sa: *const i32, l: *const i32, r: *const i32, d: *const i32, n: i32, k: i32, m: *mut i32) -> c_int;
    pub fn esaxx_c8i64 (t: *const  u8, sa: *const i64, l: *const i64, r: *const i64, d: *const i64, n: i64, k: i64, m: *mut i64) -> c_int;
    pub fn esaxx_c16i8 (t: *const u16, sa: *const  i8, l: *const  i8, r: *const  i8, d: *const  i8, n:  i8, k:  i8, m: *mut  i8) -> c_int;
    pub fn esaxx_c16i16(t: *const u16, sa: *const i16, l: *const i16, r: *const i16, d: *const i16, n: i16, k: i16, m: *mut i16) -> c_int;
    pub fn esaxx_c16i32(t: *const u16, sa: *const i32, l: *const i32, r: *const i32, d: *const i32, n: i32, k: i32, m: *mut i32) -> c_int;
    pub fn esaxx_c16i64(t: *const u16, sa: *const i64, l: *const i64, r: *const i64, d: *const i64, n: i64, k: i64, m: *mut i64) -> c_int;
    pub fn esaxx_c32i8 (t: *const u32, sa: *const  i8, l: *const  i8, r: *const  i8, d: *const  i8, n:  i8, k:  i8, m: *mut  i8) -> c_int;
    pub fn esaxx_c32i16(t: *const u32, sa: *const i16, l: *const i16, r: *const i16, d: *const i16, n: i16, k: i16, m: *mut i16) -> c_int;
    pub fn esaxx_c32i32(t: *const u32, sa: *const i32, l: *const i32, r: *const i32, d: *const i32, n: i32, k: i32, m: *mut i32) -> c_int;
    pub fn esaxx_c32i64(t: *const u32, sa: *const i64, l: *const i64, r: *const i64, d: *const i64, n: i64, k: i64, m: *mut i64) -> c_int;
    pub fn esaxx_c64i8 (t: *const u64, sa: *const  i8, l: *const  i8, r: *const  i8, d: *const  i8, n:  i8, k:  i8, m: *mut  i8) -> c_int;
    pub fn esaxx_c64i16(t: *const u64, sa: *const i16, l: *const i16, r: *const i16, d: *const i16, n: i16, k: i16, m: *mut i16) -> c_int;
    pub fn esaxx_c64i32(t: *const u64, sa: *const i32, l: *const i32, r: *const i32, d: *const i32, n: i32, k: i32, m: *mut i32) -> c_int;
    pub fn esaxx_c64i64(t: *const u64, sa: *const i64, l: *const i64, r: *const i64, d: *const i64, n: i64, k: i64, m: *mut i64) -> c_int;
}
