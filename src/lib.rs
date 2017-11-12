extern crate libc;


use std::result;
pub use libc::c_int;


/// Raw bindings to C functions `esaxx_*()`.
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


/// An error type for representing error values returned from the underlying C++ function `esaxx()`.
///
/// A value of this type can be either -1 or -2.
pub type Error = i8;

/// A specialized `Result` type with the error type `saxx::Error`.
pub type Result<T> = result::Result<T, Error>;

/// Representation of a computed extended suffix array.
pub struct Esa<T> {
    pub sa: Vec<T>,
    pub l: Vec<T>,
    pub r: Vec<T>,
    pub d: Vec<T>,
    pub m: T,
}


/// A type that can be passed to `esaxx()`.
pub trait Esaxx<Index> {
    fn esaxx(t: &Self) -> Result<Esa<Index>>;
}


macro_rules! impl_esaxx_for_slice {
    ( $char: ty, $index: ty, $esaxx_fn: expr ) => {
        impl Esaxx<$index> for [$char] {
            fn esaxx(t: &[$char]) -> Result<Esa<$index>> {
                let length = t.len();
                let max_char: $char = *t.iter().max().unwrap_or(&0);

                assert!(length <= <$index>::max_value() as usize);
                assert!(max_char as u64 <= <$index>::max_value() as u64);

                // These conversions are safe because of the above assertions
                let n: $index = length as $index;
                let k: $index = max_char as $index;

                let mut sa: Vec<$index> = Vec::new();
                let mut  l: Vec<$index> = Vec::new();
                let mut  r: Vec<$index> = Vec::new();
                let mut  d: Vec<$index> = Vec::new();
                let mut  m: $index = 0;

                let ret = unsafe {
                    $esaxx_fn(t.as_ptr(),
                              sa.as_mut_ptr(),
                              l.as_mut_ptr(),
                              r.as_mut_ptr(),
                              d.as_mut_ptr(),
                              n as $index,
                              k as $index,
                              &mut m as *mut $index) as Error
                };

                if ret == 0 {
                    Ok(Esa { sa, l, r, d, m })
                }
                else {
                    Err(ret)
                }
            }
        }
    };
}

impl_esaxx_for_slice!( u8,  i8, esaxx_c8i8);
impl_esaxx_for_slice!( u8, i16, esaxx_c8i16);
impl_esaxx_for_slice!( u8, i32, esaxx_c8i32);
impl_esaxx_for_slice!( u8, i64, esaxx_c8i64);
impl_esaxx_for_slice!(u16,  i8, esaxx_c16i8);
impl_esaxx_for_slice!(u16, i16, esaxx_c16i16);
impl_esaxx_for_slice!(u16, i32, esaxx_c16i32);
impl_esaxx_for_slice!(u16, i64, esaxx_c16i64);
impl_esaxx_for_slice!(u32,  i8, esaxx_c32i8);
impl_esaxx_for_slice!(u32, i16, esaxx_c32i16);
impl_esaxx_for_slice!(u32, i32, esaxx_c32i32);
impl_esaxx_for_slice!(u32, i64, esaxx_c32i64);
impl_esaxx_for_slice!(u64,  i8, esaxx_c64i8);
impl_esaxx_for_slice!(u64, i16, esaxx_c64i16);
impl_esaxx_for_slice!(u64, i32, esaxx_c64i32);
impl_esaxx_for_slice!(u64, i64, esaxx_c64i64);
