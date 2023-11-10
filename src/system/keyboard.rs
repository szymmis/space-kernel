use core::arch::asm;

use crate::display::logger;

#[no_mangle]
extern "C" fn kb_intr_handler() {
    fn handler(scan_code: i32) {
        if scan_code < 128 {
            logger::cls();
            logger::print("Press up")
        } else {
            logger::cls();
            logger::print("Press down")
        }
    }

    let scan_code = get_scan_code();
    if scan_code >= 0xE0 {
        handler(get_scan_code());
    } else {
        handler(scan_code);
    }
}

fn get_scan_code() -> i32 {
    unsafe {
        let val: i32;
        asm! {
            "in {0}, 0x60", out(reg) val
        }
        val
    }
}
