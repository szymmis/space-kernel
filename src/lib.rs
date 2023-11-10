#![no_std]
#![no_main]

mod game;
mod kernel;

use core::panic::PanicInfo;

use game::init_game;
use kernel::system::{Keyboard, Timer};

#[no_mangle]
pub extern "C" fn main() {
    Keyboard::init();
    Timer::init();

    init_game();
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
