use core::str;

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

pub fn print_num(n: i32) {
    let mut str = [b'\0'; 12];

    let mut value = n;
    let mut length: usize = 0;
    while value > 0 {
        str[length] = (48 + value % 10) as u8;
        value /= 10;
        length += 1;
    }

    let mut reverse_str = [b'\0'; 12];
    for i in 0..length {
        reverse_str[i] = str[length - 1 - i];
    }

    unsafe {
        print(str::from_utf8_unchecked(&reverse_str[0..length]));
    }
}

pub fn cls() {
    draw::clear_screen();
    unsafe {
        OFFSET_X = 0;
        OFFSET_Y = 0;
    }
}
