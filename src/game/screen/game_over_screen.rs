use crate::{
    game::GAME,
    kernel::{
        display::{draw::draw_text, logger::measure_str},
        system::Key,
    },
};

use super::{ActiveScreen, Screen};

pub struct GameOverScreen;
impl Screen for GameOverScreen {
    fn draw() {
        let msg = "You have lost";
        draw_text(msg, 320 / 2 - measure_str(msg) / 2, 200 / 2 - 2)
    }

    fn update() {}

    fn on_key_down(key: Key) {
        if let Key::Enter = key {
            unsafe { GAME.screen = ActiveScreen::Game }
        }
    }

    fn on_key_up(_key: Key) {}
}
