use super::{draw, glyphs};

static mut OFFSET_X: i32 = 0;
static mut OFFSET_Y: i32 = 0;

pub fn print(text: &str) {
    unsafe {
        draw::draw_text(text, OFFSET_X, OFFSET_Y);

        OFFSET_X = 0;
        OFFSET_Y += 8;
    }
}

pub fn printi(n: i32) {
    unsafe {
        let mut str = ['\0'; 12];

        let mut value = n;
        let mut length: usize = 0;
        while value > 0 {
            str[length] = ((48 + value % 10) as u8) as char;
            value /= 10;
            length += 1;
        }

        let mut reverse_str = ['\0'; 12];
        for i in 0..length {
            reverse_str[i] = str[length - 1 - i];
        }

        let mut offset = 0;
        for c in reverse_str.iter().take(length) {
            draw::draw_char(*c, OFFSET_X + offset, OFFSET_Y);
            offset += glyphs::get_glyph_width(*c);
        }

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
