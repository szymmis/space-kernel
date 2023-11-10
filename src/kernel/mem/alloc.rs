const HEAP_START: usize = 0x2000000;
static mut HEAP_OFFSET: usize = 0;

pub unsafe fn malloc<T>(size: usize) -> *mut T {
    let ptr = (HEAP_START + HEAP_OFFSET) as *mut T;
    HEAP_OFFSET += size;
    ptr
}
