use crate::{
    game::{
        assets::{GOLIATH_SPRITE, INVADER_2_SPRITE, INVADER_SPRITE, SQUID_2_SPRITE, SQUID_SPRITE},
        GAME,
    },
    kernel::{
        display::{
            draw::{draw_bitmap_scaled, draw_text, draw_text_scaled, SCREEN_WIDTH},
            logger::measure_str,
        },
        system::Key,
    },
};

use super::{ActiveScreen, Screen};

pub struct MenuScreen;
impl Screen for MenuScreen {
    fn draw() {
        unsafe {
            let title = "Space Kernel";
            draw_text_scaled(
                title,
                SCREEN_WIDTH / 2 - (measure_str(title) * 3) / 2,
                68,
                0xA,
                3,
            );

            draw_bitmap_scaled(&INVADER_SPRITE, 80, 34, 11, 8, 0xE, 2);
            draw_bitmap_scaled(
                &GOLIATH_SPRITE,
                SCREEN_WIDTH / 2 - 24 / 2,
                28,
                12,
                8,
                0x9,
                2,
            );
            draw_bitmap_scaled(&SQUID_2_SPRITE, SCREEN_WIDTH - 90 - 10, 34, 10, 8, 0xD, 2);
            draw_bitmap_scaled(&SQUID_SPRITE, 90, 102, 10, 8, 0xB, 2);
            draw_bitmap_scaled(
                &INVADER_2_SPRITE,
                SCREEN_WIDTH - 100 - 11,
                102,
                11,
                8,
                0xC,
                2,
            );

            if (GAME.ticks % 40) < 25 {
                let msg = "Press enter to start";
                draw_text(msg, SCREEN_WIDTH / 2 - measure_str(msg) / 2, 156);
            }
        }
    }

    fn update() {}

    fn on_key_down(key: Key) {
        if let Key::Enter = key {
            unsafe { GAME.screen = ActiveScreen::Game }
        }
    }

    fn on_key_up(key: Key) {}
}
