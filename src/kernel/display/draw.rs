use core::str;

use super::glyphs;

pub static SCREEN_WIDTH: i32 = 320;
pub static SCREEN_HEIGHT: i32 = 200;

pub fn draw_rect(x: i32, y: i32, width: i32, height: i32, color: u8) {
    unsafe {
        for i in 0..height {
            for j in 0..width {
                *((0xA0000 + (SCREEN_WIDTH * (y + i) + (x + j))) as *mut u8) = color;
            }
        }
    }
}

pub fn draw_bitmap(img: &[u8], x: i32, y: i32, width: i32, height: i32, color: u8) {
    unsafe {
        for i in 0..height {
            for j in 0..width {
                if img[(i * width + j) as usize] == 1 {
                    *((0xA0000 + (SCREEN_WIDTH * (y + i) + (x + j))) as *mut u8) = color;
                }
            }
        }
    }
}

pub fn draw_char(c: char, x: i32, y: i32) {
    if c != ' ' {
        let glyph = glyphs::get_glyph(c);
        draw_bitmap(glyph, x, y, 5, 5, 0xF);
    }
}

pub fn draw_text(text: &str, x: i32, y: i32) {
    let mut offset = 0;

    for c in text.chars() {
        if c != ' ' {
            draw_char(c, x + offset, y)
        }
        offset += glyphs::get_glyph_width(c);
    }
}

pub fn draw_number(num: i32, x: i32, y: i32) {
    let mut str = [b'\0'; 12];

    let mut value = num;
    let mut length: usize = 0;
    if value == 0 {
        str[0] = b'0';
        length = 1;
    } else {
        while value > 0 {
            str[length] = (48 + value % 10) as u8;
            value /= 10;
            length += 1;
        }
    }

    let mut reverse_str = [b'\0'; 12];
    for i in 0..length {
        reverse_str[i] = str[length - 1 - i];
    }

    let mut offset = 0;

    unsafe {
        for c in str::from_utf8_unchecked(&reverse_str[0..length]).chars() {
            if c != ' ' {
                draw_char(c, x + offset, y)
            }
            offset += glyphs::get_glyph_width(c);
        }
    }
}

pub fn clear_screen() {
    unsafe {
        for i in 0..SCREEN_HEIGHT {
            for j in 0..SCREEN_WIDTH {
                *((0xA0000 + (SCREEN_WIDTH * i + j)) as *mut u8) = 0x00;
            }
        }
    }
}
