use crate::*;

/// Serialize to bytes as-is for non-primitive types
#[derive(Clone, Copy, Ord, PartialOrd, Eq, PartialEq, Hash, Default)]
pub struct AsIs<T: Default>(T);
impl<T: Default> AsIs<T> {
    pub fn new(t: T) -> Self {
        Self(t)
    }
    #[inline(always)]
    pub fn unwrap(self) -> T {
        self.0
    }
}
impl<T: Default> ToBytes for AsIs<T> {
    #[inline(always)]
    fn to_bytes(self) -> Bytes {
        unsafe {
            let raw = self.to_raw_bytes();
            Bytes::from_raw_with_drops(raw, Some(vec![(0, the_drop::<T>)]))
        }
    }
}
impl<T: Default> AsRef<T> for AsIs<T> {
    #[inline(always)]
    fn as_ref(&self) -> &T {
        &self.0
    }
}
impl<T: Default> AsMut<T> for AsIs<T> {
    #[inline(always)]
    fn as_mut(&mut self) -> &mut T {
        &mut self.0
    }
}
impl<T: Default> Deref for AsIs<T> {
    type Target = T;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<T: Default> From<T> for AsIs<T> {
    #[inline]
    fn from(v: T) -> Self {
        Self(v)
    }
}
impl<T: Debug + Default> Debug for AsIs<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Debug::fmt(&self.0, f)
    }
}
impl<T: Display + Default> Display for AsIs<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Display::fmt(&self.0, f)
    }
}

impl<'a, T: Default> ReadBack<'a> for AsIs<T> {
    #[inline(always)]
    unsafe fn read_back(ptr: *const u8) -> &'a Self {
        &*(ptr as *const Self)
    }
}
impl<'a, T: Default> ReadBackMut<'a> for AsIs<T> {
    #[inline(always)]
    unsafe fn read_back_mut(ptr: *mut u8) -> &'a mut Self {
        &mut *(ptr as *mut Self)
    }
}
impl<T: Default + TransmuteBack> TransmuteBack for AsIs<T> {
    #[inline(always)]
    unsafe fn transmute_back(ptr: *const u8) -> Self {
        Self(T::transmute_back(ptr))
    }
}
#[allow(deprecated)]
unsafe impl<T: Default> ToSimpleDataBytesAsIs for AsIs<T> {}

//\////////////////////////////////////////////////////////////////////////////////////////////////////

pub type BoxIs<T> = AsIs<Box<T>>;

//\////////////////////////////////////////////////////////////////////////////////////////////////////

pub trait IntoAsIs: Sized + Default {
    #[inline]
    fn into_asis(self) -> AsIs<Self> {
        AsIs(self)
    }
}
impl<T: Default> IntoAsIs for T {}
