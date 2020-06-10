use crate::*;

#[allow(deprecated)]
impl ToSizedArray<u8> for () {
    type OutArr = [u8; 0];
    #[inline(always)]
    fn to_sized_array(self) -> Self::OutArr {
        []
    }
}

#[allow(deprecated)]
impl ToSizedArray<u8> for u8 {
    type OutArr = [u8; 1];
    #[inline(always)]
    fn to_sized_array(self) -> Self::OutArr {
        [self]
    }
}

#[allow(deprecated)]
impl ToSizedArray<u8> for i8 {
    type OutArr = [u8; 1];
    #[inline(always)]
    fn to_sized_array(self) -> Self::OutArr {
        [self as _]
    }
}

#[allow(deprecated)]
impl ToSizedArray<u8> for bool {
    type OutArr = [u8; 1];
    #[inline(always)]
    fn to_sized_array(self) -> Self::OutArr {
        [self as _]
    }
}

#[allow(deprecated)]
impl ToSizedArray<u8> for u16 {
    type OutArr = [u8; 2];
    #[inline(always)]
    fn to_sized_array(self) -> Self::OutArr {
        unsafe { transmute(self) }
    }
}

#[allow(deprecated)]
impl ToSizedArray<u8> for i16 {
    type OutArr = [u8; 2];
    #[inline(always)]
    fn to_sized_array(self) -> Self::OutArr {
        unsafe { transmute(self) }
    }
}

#[allow(deprecated)]
impl ToSizedArray<u8> for u32 {
    type OutArr = [u8; 4];
    #[inline(always)]
    fn to_sized_array(self) -> Self::OutArr {
        unsafe { transmute(self) }
    }
}

#[allow(deprecated)]
impl ToSizedArray<u8> for i32 {
    type OutArr = [u8; 4];
    #[inline(always)]
    fn to_sized_array(self) -> Self::OutArr {
        unsafe { transmute(self) }
    }
}

#[allow(deprecated)]
impl ToSizedArray<u8> for char {
    type OutArr = [u8; 4];
    #[inline(always)]
    fn to_sized_array(self) -> Self::OutArr {
        unsafe { transmute(self) }
    }
}

#[allow(deprecated)]
impl ToSizedArray<u8> for f32 {
    type OutArr = [u8; 4];
    #[inline(always)]
    fn to_sized_array(self) -> Self::OutArr {
        unsafe { transmute(self) }
    }
}

#[allow(deprecated)]
impl ToSizedArray<u8> for u64 {
    type OutArr = [u8; 8];
    #[inline(always)]
    fn to_sized_array(self) -> Self::OutArr {
        unsafe { transmute(self) }
    }
}

#[allow(deprecated)]
impl ToSizedArray<u8> for i64 {
    type OutArr = [u8; 8];
    #[inline(always)]
    fn to_sized_array(self) -> Self::OutArr {
        unsafe { transmute(self) }
    }
}

#[allow(deprecated)]
impl ToSizedArray<u8> for f64 {
    type OutArr = [u8; 8];
    #[inline(always)]
    fn to_sized_array(self) -> Self::OutArr {
        unsafe { transmute(self) }
    }
}

#[allow(deprecated)]
impl ToSizedArray<u8> for u128 {
    type OutArr = [u8; 16];
    #[inline(always)]
    fn to_sized_array(self) -> Self::OutArr {
        unsafe { transmute(self) }
    }
}

#[allow(deprecated)]
impl ToSizedArray<u8> for i128 {
    type OutArr = [u8; 16];
    #[inline(always)]
    fn to_sized_array(self) -> Self::OutArr {
        unsafe { transmute(self) }
    }
}


//\////////////////////////////////////////////////////////////////////////////////////////////////////

impl ToBytes for usize {
    #[inline(always)]
    fn to_bytes(self) -> Bytes {
        unsafe { Bytes::from_raw_bytes(self.to_raw_bytes()) }
    }
}

impl ToBytes for isize {
    #[inline(always)]
    fn to_bytes(self) -> Bytes {
        unsafe { Bytes::from_raw_bytes(self.to_raw_bytes()) }
    }
}

//\////////////////////////////////////////////////////////////////////////////////////////////////////

impl<'a> ReadBack<'a> for usize {
    #[inline(always)]
    unsafe fn read_back(ptr: *const u8) -> &'a Self {
        &*(ptr as *const Self)
    }
}
impl<'a> ReadBackMut<'a> for usize {
    #[inline(always)]
    unsafe fn read_back_mut(ptr: *mut u8) -> &'a mut Self {
        &mut *(ptr as *mut Self)
    }
}
impl TransmuteBack for usize {
    #[inline(always)]
    unsafe fn transmute_back(ptr: *const u8) -> Self {
        *(ptr as *const Self)
    }
}

impl<'a> ReadBack<'a> for isize {
    #[inline(always)]
    unsafe fn read_back(ptr: *const u8) -> &'a Self {
        &*(ptr as *const Self)
    }
}
impl<'a> ReadBackMut<'a> for isize {
    #[inline(always)]
    unsafe fn read_back_mut(ptr: *mut u8) -> &'a mut Self {
        &mut *(ptr as *mut Self)
    }
}
impl TransmuteBack for isize {
    #[inline(always)]
    unsafe fn transmute_back(ptr: *const u8) -> Self {
        *(ptr as *const Self)
    }
}


//\////////////////////////////////////////////////////////////////////////////////////////////////////

impl<T> ToBytes for *const T {
    #[inline(always)]
    fn to_bytes(self) -> Bytes {
        (self as usize).to_bytes()
    }
}

impl<T> ToBytes for *mut T {
    #[inline(always)]
    fn to_bytes(self) -> Bytes {
        (self as usize).to_bytes()
    }
}

//\////////////////////////////////////////////////////////////////////////////////////////////////////

impl<'a, T> ReadBack<'a> for *const T {
    #[inline(always)]
    unsafe fn read_back(ptr: *const u8) -> &'a Self {
        &*(ptr as *const Self)
    }
}
impl<'a, T> ReadBackMut<'a> for *const T {
    #[inline(always)]
    unsafe fn read_back_mut(ptr: *mut u8) -> &'a mut Self {
        &mut *(ptr as *mut Self)
    }
}
impl<T> TransmuteBack for *const T {
    #[inline(always)]
    unsafe fn transmute_back(ptr: *const u8) -> Self {
        *(ptr as *const Self)
    }
}

impl<'a, T> ReadBack<'a> for *mut T {
    #[inline(always)]
    unsafe fn read_back(ptr: *const u8) -> &'a Self {
        &*(ptr as *const *mut T)
    }
}
impl<'a, T> ReadBackMut<'a> for *mut T {
    #[inline(always)]
    unsafe fn read_back_mut(ptr: *mut u8) -> &'a mut Self {
        &mut *(ptr as *mut *mut T)
    }
}
impl<T> TransmuteBack for *mut T {
    #[inline(always)]
    unsafe fn transmute_back(ptr: *const u8) -> Self {
        *(ptr as *const *mut T)
    }
}