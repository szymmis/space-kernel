use core::arch::asm;

use crate::mem::vec::Vec;

static mut KEY_DOWN_HANDLERS: Option<Vec<fn(Key)>> = None;
static mut KEY_UP_HANDLERS: Option<Vec<fn(Key)>> = None;

pub struct Keyboard;

impl Keyboard {
    pub fn init() {
        unsafe {
            KEY_DOWN_HANDLERS = Some(Vec::new(5));
            KEY_UP_HANDLERS = Some(Vec::new(5));
        }
    }

    pub fn add_on_key_down_listener(f: fn(Key)) {
        unsafe {
            if let Some(v) = &mut KEY_DOWN_HANDLERS {
                v.push(f);
            }
        }
    }

    pub fn add_on_key_up_listener(f: fn(Key)) {
        unsafe {
            if let Some(v) = &mut KEY_UP_HANDLERS {
                v.push(f);
            }
        }
    }
}

#[no_mangle]
unsafe extern "C" fn kb_intr_handler() {
    unsafe fn get_scan_code() -> u8 {
        let val: u8;
        asm! {
            "in al, 0x60", out("al") val
        }
        val
    }

    unsafe fn handler(scan_code: u8) {
        if scan_code >= 128 {
            if let Some(v) = &mut KEY_UP_HANDLERS {
                for i in 0..v.len() {
                    (*v).get(i)((scan_code & 0x7F).into());
                }
            }
        } else if let Some(v) = &mut KEY_DOWN_HANDLERS {
            for i in 0..v.len() {
                (*v).get(i)((scan_code).into());
            }
        }
    }

    let scan_code = get_scan_code();
    if scan_code >= 0xE0 {
        handler(get_scan_code());
    } else {
        handler(scan_code);
    }
}

pub enum Key {
    Enter,
    Spacebar,

    ArrowUp,
    ArrowDown,
    ArrowLeft,
    ArrowRigth,

    Unkown(u8),
}

impl From<u8> for Key {
    fn from(value: u8) -> Self {
        match value {
            0x1C => Key::Enter,
            0x39 => Key::Spacebar,
            0x48 => Key::ArrowUp,
            0x50 => Key::ArrowDown,
            0x4B => Key::ArrowLeft,
            0x4D => Key::ArrowRigth,
            _ => Key::Unkown(value),
        }
    }
}
