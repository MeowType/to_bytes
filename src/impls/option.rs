use crate::*;

#[allow(deprecated)]
impl<T: ToSimpleDataBytes> ToBytes for Option<T> {
    #[inline(always)]
    fn to_bytes(self) -> Bytes {
        unsafe { Bytes::from_raw_bytes(self.to_raw_bytes()) }
    }
}
#[allow(deprecated)]
impl<T: ToComplexDataBytes + Default> ToBytes for Option<AsIs<T>> {
    #[inline(always)]
    fn to_bytes(self) -> Bytes {
        unsafe {
            let raw = self.to_raw_bytes();
            Bytes::from_raw_with_drops(raw, Some(vec![(0, the_drop::<Option<AsIs<T>>>)]))
        }
    }
}

//\////////////////////////////////////////////////////////////////////////////////////////////////////

#[allow(deprecated)]
impl<'a, T> ReadBack<'a> for Option<T>
where
    Self: ToBytes,
{
    #[inline(always)]
    unsafe fn read_back(ptr: *const u8) -> &'a Self {
        &*(ptr as *const Self)
    }
}
#[allow(deprecated)]
impl<'a, T> ReadBackMut<'a> for Option<T>
where
    Self: ToBytes,
{
    #[inline(always)]
    unsafe fn read_back_mut(ptr: *mut u8) -> &'a mut Self {
        &mut *(ptr as *mut Self)
    }
}
#[allow(deprecated)]
impl<T: TransmuteBack> TransmuteBack for Option<T>
where
    Self: ToBytes,
{
    #[inline(always)]
    unsafe fn transmute_back(ptr: *const u8) -> Self {
        let full = core::mem::size_of::<Self>();
        let sub = core::mem::size_of::<T>();
        let offset = full - sub;
        let r = Self::read_back(ptr);
        match r {
            Some(_) => Some(T::transmute_back(ptr.add(offset))),
            None => None,
        }
    }
}
