use crate::{
    game::GAME,
    kernel::{
        display::{draw::draw_text, logger::measure_str},
        system::Key,
    },
};

use super::{ActiveScreen, Screen};

pub struct MenuScreen;
impl Screen for MenuScreen {
    fn draw() {
        unsafe {
            if let Some(game) = &mut GAME {
                if (game.ticks % 40) < 25 {
                    let msg = "Press enter to start";
                    draw_text(msg, 320 / 2 - measure_str(msg) / 2, 200 / 2 - 2)
                }
            }
        }
    }

    fn update() {}

    fn on_key_down(key: Key) {
        if let Key::Enter = key {
            unsafe {
                if let Some(game) = &mut GAME {
                    game.screen = ActiveScreen::Game
                }
            }
        }
    }

    fn on_key_up(key: Key) {}
}
