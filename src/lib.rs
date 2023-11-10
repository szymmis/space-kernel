#![no_std]
#![no_main]

use core::panic::PanicInfo;

mod display;
mod mem;
mod system;

use display::{draw::draw_rect, logger};
use mem::vec::Vec;
use system::{
    keyboard::{Key, Keyboard},
    timer::Timer,
};

#[derive(Clone, Copy)]
struct Number {
    val: i32,
}

impl Number {
    fn set(&mut self, val: i32) {
        self.val = val;
    }
}

static mut MY_BOX: Option<Number> = Option::None;

#[no_mangle]
pub extern "C" fn main() {
    Keyboard::init();
    Timer::init();

    logger::cls();
    logger::print("0123456789ABCDEFGHIJKLMNOPRSTUWXYZ");
    logger::print("Hello world");
    logger::printi(123);

    fn test(_: Key) {
        logger::cls();
        logger::print("Print");
        unsafe {
            if let Some(val) = &mut MY_BOX {
                val.set(213);
                logger::printi(val.val);
            }
        }
    }

    fn test2(_: Key) {
        logger::print("Print2");
        unsafe {
            if let Some(val) = &mut MY_BOX {
                val.set(23);
                logger::printi(val.val);
            }
        }
    }

    unsafe {
        MY_BOX = Some(Number { val: 100 });

        Keyboard::add_on_key_down_listener(test);
        Keyboard::add_on_key_down_listener(test2);
        Keyboard::add_on_key_down_listener(|val| {
            logger::print("From closure");
            logger::print("");
            match val {
                Key::Enter => logger::print("Enter"),
                Key::Spacebar => logger::print("Spacebar"),
                Key::Unkown(val) => logger::printi(val as i32),
                _ => logger::print("Key down"),
            }
        });

        Keyboard::add_on_key_up_listener(|val| {
            logger::cls();

            match val {
                Key::Enter => logger::print("Enter up"),
                Key::Spacebar => logger::print("Spacebar up"),
                Key::Unkown(val) => logger::printi(val as i32),
                _ => logger::print("Key up"),
            }
        });
    }

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
        logger::printi(v2.get(i).val);
    }

    v2.get(0).set(10);
    v2.get(1).set(24);

    for i in 0..v2.len() {
        logger::printi(v2.get(i).val);
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    draw_rect(0, 0, 320, 200, 0x4);
    loop {}
}
