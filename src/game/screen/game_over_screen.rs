use crate::{
    game::GAME,
    kernel::{
        display::{draw::draw_text, logger::measure_str},
        system::Key,
    },
};

use super::{Screen, GAME_SCREEN};

pub struct GameOverScreen;
impl Screen for GameOverScreen {
    fn draw(&self) {
        let msg = "You have lost";
        draw_text(msg, 320 / 2 - measure_str(msg) / 2, 200 / 2 - 2)
    }

    fn update(&self) {}

    fn on_key_down(&self, key: Key) {
        if let Key::Enter = key {
            unsafe { GAME.screen = &GAME_SCREEN }
        }
    }

    fn on_key_up(&self, _key: Key) {}
}
