use crate::mem::alloc::malloc;

pub struct Vec<T>
where
    T: Copy,
{
    data: *mut T,
    length: usize,
    capacity: usize,
    element_size: usize,
}

impl<T> Vec<T>
where
    T: Copy,
{
    pub fn new(capacity: usize) -> Self {
        unsafe {
            let element_size = core::mem::size_of::<T>();
            Self {
                data: malloc(element_size * capacity),
                length: 0,
                capacity,
                element_size,
            }
        }
    }

    pub fn get(&mut self, i: usize) -> &mut T {
        if i >= self.length {
            panic!();
        }

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

    pub fn remove(&mut self, i: usize) {
        if i >= self.length {
            return;
        }

        unsafe {
            for i in i..self.length {
                *self.data.add(i) = *self.data.add(i + 1);
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
                *ptr.add(i) = *old_ptr.add(i);
            }

            self.data = ptr;
            self.capacity += 5;
        }
    }
}
