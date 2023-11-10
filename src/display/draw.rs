use super::glyphs;

static SCREEN_WIDTH: i32 = 320;
static SCREEN_HEIGHT: i32 = 200;

pub fn draw_rect(x: i32, y: i32, width: i32, height: i32) {
    unsafe {
        for i in 0..height {
            for j in 0..width {
                *((0xA0000 + (SCREEN_WIDTH * (y + i) + (x + j))) as *mut u8) = 0x0F;
            }
        }
    }
}

pub fn draw_bitmap(img: &[i32], x: i32, y: i32, width: i32, height: i32) {
    unsafe {
        for i in 0..height {
            for j in 0..width {
                if img[(i * width + j) as usize] == 1 {
                    *((0xA0000 + (SCREEN_WIDTH * (y + i) + (x + j))) as *mut u8) = 0x0F;
                }
            }
        }
    }
}

pub fn draw_char(c: char, x: i32, y: i32) {
    if c != ' ' {
        let glyph = glyphs::get_glyph(c);
        draw_bitmap(glyph, x, y, 5, 5);
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

pub fn clear_screen() {
    unsafe {
        for i in 0..SCREEN_HEIGHT {
            for j in 0..SCREEN_WIDTH {
                *((0xA0000 + (SCREEN_WIDTH * i + j)) as *mut u8) = 0x00;
            }
        }
    }
}
