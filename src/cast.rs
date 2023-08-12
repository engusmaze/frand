use core::ptr;

#[inline(always)]
pub fn assume_is<From, To: Copy>(value: From) -> To {
    unsafe { *(&value as *const _ as *const To) }
}
#[inline(always)]
pub const fn assume_copy<From, To>(value: &From) -> To {
    unsafe { ptr::read_unaligned(value as *const _ as *const To) }
}
