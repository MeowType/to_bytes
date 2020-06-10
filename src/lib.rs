mod bytes;
mod impls;
pub mod specialization;
#[cfg(test)]
mod tests;
mod to_raw_bytes;
mod to_sized_array;
mod utils;

pub use bytes::*;
pub use impls::*;
pub use specialization::*;
use std::cmp::Ordering;
use std::fmt;
use std::fmt::{Debug, Display};
use std::hash::{Hash, Hasher};
use std::mem::transmute;
use std::ops::*;
#[cfg(feature = "to_bytes_derive")]
pub use to_bytes_derive::*;

pub mod not_important {
    pub use crate::to_raw_bytes::*;
    pub use crate::to_sized_array::*;
    pub use crate::utils::*;
}
pub use not_important::*;

//\////////////////////////////////////////////////////////////////////////////////////////////////////

pub trait ToBytes {
    fn to_bytes(self) -> Bytes;
}

//\////////////////////////////////////////////////////////////////////////////////////////////////////

pub trait ReadBack<'a>: ToBytes {
    unsafe fn read_back(ptr: *const u8) -> &'a Self;
}
pub trait ReadBackMut<'a>: ToBytes {
    unsafe fn read_back_mut(ptr: *mut u8) -> &'a mut Self;
}
pub trait TransmuteBack: ToBytes {
    unsafe fn transmute_back(ptr: *const u8) -> Self;
}

//\////////////////////////////////////////////////////////////////////////////////////////////////////

#[macro_export]
macro_rules! bytes {
    { } => { Bytes::new() };
    { $e:expr } => { $crate::ToBytes::to_bytes($e) };
    { $($e:expr)* } => {{
        #[allow(unused_mut)]
        let mut b = Bytes::new();
        $(b.append($crate::ToBytes::to_bytes($e));)*
        b
    }};
}
