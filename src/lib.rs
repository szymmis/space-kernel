#![no_std]
#![no_main]

use core::panic::PanicInfo;

mod display;
mod mem;
mod system;

use display::logger;
use mem::vec::Vec;

#[derive(Clone, Copy)]
struct Number {
    val: i32,
}

impl Number {
    fn set(&mut self, val: i32) {
        self.val = val;
    }
}

#[no_mangle]
pub extern "C" fn main() {
    logger::cls();
    logger::print("0123456789ABCDEFGHIJKLMNOPRSTUWXYZ");
    logger::print("Hello world");

    let mut v = Vec::new(2);

    v.push(5);
    v.push(10);
    v.push(15);
    v.push(20);
    v.push(25);
    v.push(30);
    v.push(35);
    v.push(40);

    let mut v2: Vec<Number> = Vec::new(5);
    v2.push(Number { val: 128 });
    v2.push(Number { val: 45 });

    v.remove(3);
    v.remove(3);
    v.remove(3);

    for i in 0..v.len() {
        let val = *v.get(i);
        logger::printi(val);
    }

    for i in 0..v2.len() {
        logger::printi(v2.get(i).val as usize);
    }

    v2.get(0).set(10);
    v2.get(1).set(24);

    for i in 0..v2.len() {
        logger::printi(v2.get(i).val as usize);
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
