#![no_std]
#![no_main]

mod display;
mod system;

use core::panic::PanicInfo;
use display::{draw, logger};

#[no_mangle]
pub extern "C" fn main() {
    logger::cls();
    logger::print("0123456789ABCDEFGHIJKLMNOPRSTUWXYZ");
    logger::print("Hello world");

    draw::draw_rect(270, 10, 10, 32);
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
