use std::{
    mem::{ManuallyDrop, MaybeUninit},
    ptr::addr_of_mut,
};

struct DropGuard<T> {
    ptr: *mut T,
    len: usize,
}

impl<T> Drop for DropGuard<T> {
    #[inline]
    fn drop(&mut self) {
        unsafe {
            core::ptr::slice_from_raw_parts_mut(self.ptr, self.len).drop_in_place();
        }
    }
}

#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct TaggedLen(usize);

impl TaggedLen {
    #[inline]
    pub const fn new(len: usize) -> Self {
        debug_assert!(len < isize::MAX as usize);
        TaggedLen(len << 1)
    }

    #[inline]
    pub const fn value(self) -> usize {
        self.0 >> 1
    }
}

//From https://github.com/servo/rust-smallvec/blob/v2/src/lib.rs
pub struct SmallVec<T, const N: usize> {
    len: TaggedLen,
    raw: ManuallyDrop<MaybeUninit<[T; N]>>,
}

impl<T: Clone, const N: usize> SmallVec<T, N> {
    pub fn new() -> Self {
        Self {
            raw: ManuallyDrop::new(MaybeUninit::uninit()),
            len: TaggedLen(0),
        }
    }

    fn as_mut_ptr_inline(&mut self) -> *mut T {
        addr_of_mut!(self.raw) as *mut T
    }

    #[inline]
    pub const fn capacity(&self) -> usize {
        N
    }

    #[inline]
    pub unsafe fn set_len(&mut self, new_len: usize) {
        debug_assert!(new_len <= self.capacity());
        self.len = TaggedLen::new(new_len);
    }

    pub fn from_elem(elem: T, n: usize) -> Self {
        let mut v = Self::new();
        if n > N {}

        unsafe {
            let ptr = v.as_mut_ptr_inline();
            let mut guard = DropGuard { ptr, len: 0 };

            // SAFETY: `n <= Self::inline_size()` so we can write `n` elements
            for i in 0..n {
                guard.len = i;
                ptr.add(i).write(elem.clone());
            }
            core::mem::forget(guard);
            // SAFETY: we just initialized `n` elements in the vector

            v.set_len(n);
        }
        v
    }
}
