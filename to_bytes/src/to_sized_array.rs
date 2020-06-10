use crate::*;

#[deprecated]
/// wait cont number `<const N: usize>`
pub trait ToSizedArray<T> {
    /// must be array `[T; N]`
    type OutArr: Sized + AsRef<[T]> + Copy;
    fn to_sized_array(self) -> Self::OutArr;
}

//\////////////////////////////////////////////////////////////////////////////////////////////////////

#[allow(deprecated)]
impl<T: ToSizedArray<u8>> ToBytes for T {
    #[inline(always)]
    fn to_bytes(self) -> Bytes {
        Bytes { inner: Some(self.to_sized_array().as_ref().to_vec()), drops: None }
    }
}

#[allow(deprecated)]
impl<'a, T: ToSizedArray<u8>> ReadBack<'a> for T {
    #[inline(always)]
    unsafe fn read_back(ptr: *const u8) -> &'a Self {
        &*(ptr as *const T)
    }
}

#[allow(deprecated)]
impl<'a, T: ToSizedArray<u8>> ReadBackMut<'a> for T {
    #[inline(always)]
    unsafe fn read_back_mut(ptr: *mut u8) -> &'a mut Self {
        &mut *(ptr as *mut T)
    }
}

#[allow(deprecated)]
impl<T: ToSizedArray<u8> + Copy> TransmuteBack for T {
    #[inline(always)]
    unsafe fn transmute_back(ptr: *const u8) -> Self {
        *(ptr as *const T)
    }
}
