use core::{
    mem::size_of,
    ops::{Deref, DerefMut},
    ptr::null_mut,
};

use super::alloc::{malloc, memcpy};

pub struct LazyBox<T> {
    data: *mut T,
    init: fn() -> T,
}

impl<T> LazyBox<T> {
    pub const fn new(init: fn() -> T) -> Self {
        Self {
            data: null_mut(),
            init,
        }
    }
}

impl<T> Deref for LazyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.data }
    }
}

impl<T> DerefMut for LazyBox<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        if self.data.is_null() {
            unsafe {
                self.data = malloc(size_of::<T>());
                memcpy(&mut (self.init)() as *mut T, self.data, size_of::<T>());
            }
        }

        unsafe { &mut *self.data }
    }
}
