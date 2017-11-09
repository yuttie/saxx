extern crate libc;


use std::result;
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


pub type Error = i8;

pub type Result<T> = result::Result<T, Error>;

pub struct Esa<T> {
    pub sa: Vec<T>,
    pub l: Vec<T>,
    pub r: Vec<T>,
    pub d: Vec<T>,
    pub m: T,
}


pub trait Esaxx<Index> {
    fn esaxx(t: &Self) -> Result<Esa<Index>>;
}


impl Esaxx<i8> for [u8] {
    fn esaxx(t: &[u8]) -> Result<Esa<i8>> {
        let length = t.len();
        let max_char: u8 = *t.iter().max().unwrap_or(&0);
        assert!(length <= i8::max_value() as usize);
        assert!(max_char <= i8::max_value() as u8);

        let n: i8 = length as i8;
        let k: i8 = max_char as i8;
        let mut sa: Vec<i8> = Vec::new();
        let mut  l: Vec<i8> = Vec::new();
        let mut  r: Vec<i8> = Vec::new();
        let mut  d: Vec<i8> = Vec::new();
        let mut  m: i8 = 0;

        let ret = unsafe {
            esaxx_c8i8(t.as_ptr(),
                       sa.as_mut_ptr(),
                       l.as_mut_ptr(),
                       r.as_mut_ptr(),
                       d.as_mut_ptr(),
                       n as i8,
                       k as i8,
                       &mut m as *mut i8) as Error
        };

        if ret == 0 {
            Ok(Esa { sa, l, r, d, m })
        }
        else {
            Err(ret)
        }
    }
}
