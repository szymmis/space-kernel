use core::mem::size_of;

use crate::kernel::mem::alloc::malloc;

use super::alloc::memcpy;

pub struct Vec<T> {
    data: *mut T,
    length: usize,
    capacity: usize,
    element_size: usize,
}

impl<T> Vec<T> {
    pub fn new(capacity: usize) -> Self {
        unsafe {
            let element_size = size_of::<T>();
            Self {
                data: malloc(element_size * capacity),
                length: 0,
                capacity,
                element_size,
            }
        }
    }

    pub fn get_const(&self, i: usize) -> &T {
        unsafe { &*self.data.add(i) }
    }

    pub fn get(&mut self, i: usize) -> &mut T {
        unsafe { &mut *self.data.add(i) }
    }

    pub fn push(&mut self, val: T) {
        unsafe {
            if self.length == self.capacity {
                self.realloc()
            }

            *self.data.add(self.length) = val;
            self.length += 1;
        }
    }

    #[allow(dead_code)]
    pub fn remove(&mut self, i: usize) {
        if i >= self.length {
            return;
        }

        unsafe {
            for i in i..(self.length - 1) {
                memcpy(self.data.add(i + 1), self.data.add(i), self.element_size)
            }
            self.length -= 1;
        }
    }

    pub fn len(&self) -> usize {
        self.length
    }

    fn realloc(&mut self) {
        unsafe {
            let old_ptr = self.data;
            let ptr: *mut T = malloc(self.element_size * (self.capacity + 5));

            for i in 0..self.length {
                memcpy(old_ptr.add(i + 1), ptr.add(i), self.element_size)
            }

            self.data = ptr;
            self.capacity += 5;
        }
    }
}
