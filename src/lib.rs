#![no_std]
#![no_main]

use core::panic::PanicInfo;

mod display;
mod mem;
mod system;

use display::{draw, logger};
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

    let mut offset = 0;
    for i in 0..v.len() {
        let val = *v.get(i);
        print_a(val, 20, 20 + offset);
        offset += val + 10;
    }

    offset = 0;
    for i in 0..v2.len() {
        let val = v2.get(i).val;
        print_a(val, 80, 20 + offset);
        offset += val + 10;
    }

    v2.get(0).set(10);
    v2.get(1).set(24);

    offset = 0;
    for i in 0..v2.len() {
        let val = v2.get(i).val;
        print_a(val, 220, 20 + offset);
        offset += val + 10;
    }
}

fn print_a(val: i32, x: i32, y: i32) {
    draw::draw_rect(x, y, val, val);
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
