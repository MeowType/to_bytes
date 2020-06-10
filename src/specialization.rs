//! This module exists only because the garbage rust has not yet had a generic specialization
use crate::*;

/// a Flag, for impl ToBytes` whit all fields are primitive types or ToSimpleDataBytes
#[allow(deprecated)]
#[deprecated]
pub unsafe trait ToSimpleDataBytesAsIs: ToBytes {}
/// a Flag, for impl ToBytes` whit all fields are primitive types or ToSimpleDataBytes but not AsIs
#[allow(deprecated)]
#[deprecated]
pub unsafe trait ToSimpleDataBytes: ToBytes {}
/// a Flag, for `impl ToBytes` whit has any not primitive types or ToSimpleDataBytes fields
#[deprecated]
pub unsafe trait ToComplexDataBytes {}


//\////////////////////////////////////////////////////////////////////////////////////////////////////

#[allow(deprecated)]
unsafe impl ToSimpleDataBytes for () {}
#[allow(deprecated)]
unsafe impl ToSimpleDataBytes for bool {}
#[allow(deprecated)]
unsafe impl ToSimpleDataBytes for u8 {}
#[allow(deprecated)]
unsafe impl ToSimpleDataBytes for u16 {}
#[allow(deprecated)]
unsafe impl ToSimpleDataBytes for u32 {}
#[allow(deprecated)]
unsafe impl ToSimpleDataBytes for u64 {}
#[allow(deprecated)]
unsafe impl ToSimpleDataBytes for u128 {}
#[allow(deprecated)]
unsafe impl ToSimpleDataBytes for i8 {}
#[allow(deprecated)]
unsafe impl ToSimpleDataBytes for i16 {}
#[allow(deprecated)]
unsafe impl ToSimpleDataBytes for i32 {}
#[allow(deprecated)]
unsafe impl ToSimpleDataBytes for i64 {}
#[allow(deprecated)]
unsafe impl ToSimpleDataBytes for i128 {}
#[allow(deprecated)]
unsafe impl ToSimpleDataBytes for char {}
#[allow(deprecated)]
unsafe impl ToSimpleDataBytes for usize {}
#[allow(deprecated)]
unsafe impl ToSimpleDataBytes for isize {}
#[allow(deprecated)]
unsafe impl ToSimpleDataBytes for f32 {}
#[allow(deprecated)]
unsafe impl ToSimpleDataBytes for f64 {}
#[allow(deprecated)]
unsafe impl<T> ToSimpleDataBytes for *mut T {}
#[allow(deprecated)]
unsafe impl<T> ToSimpleDataBytes for *const T {}

//\////////////////////////////////////////////////////////////////////////////////////////////////////

#[allow(deprecated)]
unsafe impl<T: ToSimpleDataBytes> ToSimpleDataBytesAsIs for T {}

//\////////////////////////////////////////////////////////////////////////////////////////////////////

#[allow(deprecated)]
unsafe impl<T> ToComplexDataBytes for Box<T> {}
#[allow(deprecated)]
unsafe impl<T> ToComplexDataBytes for std::rc::Rc<T> {}
#[allow(deprecated)]
unsafe impl<T> ToComplexDataBytes for std::rc::Weak<T> {}
#[allow(deprecated)]
unsafe impl<T> ToComplexDataBytes for std::sync::Arc<T> {}
#[allow(deprecated)]
unsafe impl<T> ToComplexDataBytes for std::sync::Weak<T> {}