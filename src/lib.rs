#![no_std]
#![no_main]

use core::panic::PanicInfo;

static mut VGA_BUFFER: *mut u8 = 0xb8000 as *mut u8;

#[no_mangle]
pub extern "C" fn main() {
    unsafe {
        for (i, char) in "Hello from Rust!".chars().enumerate() {
            *VGA_BUFFER.offset(i as isize * 2) = char as u8;
            *VGA_BUFFER.offset(i as isize * 2 + 1) = 0x07;
        }
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
