#![no_std]
#![no_main]

mod game;
mod kernel;

use core::panic::PanicInfo;

use game::Game;
use kernel::system::{Keyboard, Timer};

#[no_mangle]
pub extern "C" fn main() {
    Keyboard::init();
    Timer::init();

    Game::init();
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
