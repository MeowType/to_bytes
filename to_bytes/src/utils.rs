use crate::*;

pub unsafe fn the_drop<T: Default>(b: &mut Bytes, offset: usize) {
    core::mem::take(&mut *(b.as_mut_ptr().add(offset) as *mut T));
}
