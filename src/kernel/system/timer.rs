use crate::kernel::mem::vec::Vec;

static mut TIMER_HANDLERS: Option<Vec<fn()>> = None;

pub struct Timer;

impl Timer {
    pub fn init() {
        unsafe {
            TIMER_HANDLERS = Some(Vec::new(5));
        }
    }

    pub fn add_timer_listener(f: fn()) {
        unsafe {
            if let Some(v) = &mut TIMER_HANDLERS {
                v.push(f);
            }
        }
    }
}

#[no_mangle]
unsafe extern "C" fn timer_intr_handler() {
    if let Some(v) = &mut TIMER_HANDLERS {
        for i in 0..v.len() {
            (*v).get(i)();
        }
    }
}
