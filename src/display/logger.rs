use super::draw;

static mut OFFSET_X: i32 = 0;
static mut OFFSET_Y: i32 = 0;

pub fn print(text: &str) {
    unsafe {
        draw::draw_text(text, OFFSET_X, OFFSET_Y);

        OFFSET_X = 0;
        OFFSET_Y += 8;
    }
}

pub fn cls() {
    draw::clear_screen();
    unsafe {
        OFFSET_X = 0;
        OFFSET_Y = 0;
    }
}
