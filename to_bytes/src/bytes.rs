use crate::*;

#[derive(Clone)]
pub struct Bytes {
    pub(crate) inner: Option<Vec<u8>>, // never None
    pub(crate) drops: BytesDrops,
}
type BytesDrops = Option<Vec<(usize, unsafe fn(&mut Bytes, usize))>>;
impl Bytes {
    #[inline]
    pub fn new() -> Self {
        Self { inner: Some(vec![]), drops: None }
    }
    #[inline]
    pub unsafe fn from_raw_bytes(raw: Vec<u8>) -> Self {
        Self { inner: Some(raw), drops: None }
    }
    #[inline]
    pub unsafe fn from_raw_with_drops(raw: Vec<u8>, drops: BytesDrops) -> Self {
        Self { inner: Some(raw), drops }
    }
    #[inline]
    pub unsafe fn push(&mut self, bytes: impl AsRef<[u8]>, drops: BytesDrops) {
        let inner = self.inner.as_mut().unwrap();
        let now_offst = inner.len();
        inner.extend_from_slice(bytes.as_ref());
        if let Some(d) = drops {
            if let Some(ref mut s) = self.drops {
                let d = d.into_iter().map(|(o, f)| (o + now_offst, f));
                s.extend(d);
            } else {
                self.drops = Some(d)
            }
        }
    }
    #[inline]
    pub fn append(&mut self, mut bytes: Self) {
        unsafe { self.push(bytes.inner.take().unwrap(), bytes.drops.take()) }
    }
}
impl Drop for Bytes {
    #[inline]
    fn drop(&mut self) {
        if let Some(d) = self.drops.take() {
            for (o, f) in d {
                unsafe { f(self, o) }
            }
        }
    }
}
impl Default for Bytes {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}
impl Deref for Bytes {
    type Target = [u8];

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.as_ref()
    }
}
impl DerefMut for Bytes {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.as_mut()
    }
}
impl AsRef<[u8]> for Bytes {
    #[inline]
    fn as_ref(&self) -> &[u8] {
        self.inner.as_ref().unwrap()
    }
}
impl AsMut<[u8]> for Bytes {
    #[inline]
    fn as_mut(&mut self) -> &mut [u8] {
        self.inner.as_mut().unwrap()
    }
}
impl Debug for Bytes {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Bytes").field("inner", self.inner.as_ref().unwrap()).field("drops", &self.drops.as_ref().map(|v| v.len()).unwrap_or(0)).finish()
    }
}
impl Display for Bytes {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.inner.fmt(f)
    }
}
impl PartialEq for Bytes {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.inner == other.inner
    }
}
impl Eq for Bytes {}
impl Hash for Bytes {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.inner.hash(state)
    }
}
impl PartialOrd for Bytes {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.inner.partial_cmp(&other.inner)
    }
}
impl Ord for Bytes {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        self.inner.cmp(&other.inner)
    }
}
impl<'a> Bytes {
    #[inline(always)]
    pub unsafe fn read_back<T: ReadBack<'a>>(&'a self) -> &'a T {
        T::read_back(self.as_ptr())
    }
    #[inline(always)]
    pub unsafe fn read_back_at<T: ReadBack<'a>>(&'a self, offset: usize) -> &'a T {
        T::read_back(self.as_ptr().add(offset))
    }
    #[inline(always)]
    pub unsafe fn read_back_mut<T: ReadBackMut<'a>>(&'a mut self) -> &'a mut T {
        T::read_back_mut(self.as_mut_ptr())
    }
    #[inline(always)]
    pub unsafe fn read_back_mut_at<T: ReadBackMut<'a>>(&'a mut self, offset: usize) -> &'a mut T {
        T::read_back_mut(self.as_mut_ptr().add(offset))
    }
}
impl Bytes {
    #[inline(always)]
    pub unsafe fn transmute_back<R: TransmuteBack>(&self) -> R {
        R::transmute_back(self.as_ptr())
    }
    #[inline(always)]
    pub unsafe fn transmute_back_at<R: TransmuteBack>(&self, offset: usize) -> R {
        R::transmute_back(self.as_ptr().add(offset))
    }
}
impl ToBytes for Bytes {
    fn to_bytes(self) -> Bytes {
        self
    }
}