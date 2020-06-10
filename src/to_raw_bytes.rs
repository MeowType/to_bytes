pub trait ToRawBytes {
    unsafe fn to_raw_bytes(self) -> Vec<u8>;
}

//\////////////////////////////////////////////////////////////////////////////////////////////////////

impl<T> ToRawBytes for T {
    #[inline]
    unsafe fn to_raw_bytes(self) -> Vec<u8> {
        let size: usize = std::mem::size_of::<Self>();
        let mut v = vec![];
        let p = &self as *const Self as *const u8;
        for i in 0..size {
            v.push(*(p.add(i)))
        }
        core::mem::forget(self);
        v
    }
}
