use crate::{
    game::{player::Direction, GAME},
    kernel::system::Key,
};

use super::Screen;

pub struct GameScreen;
impl Screen for GameScreen {
    fn draw() {
        unsafe {
            if let Some(game) = &mut GAME {
                game.player.draw();
            }
        }
    }

    fn update() {}

    fn on_key_down(key: Key) {
        unsafe {
            if let Some(game) = &mut GAME {
                match key {
                    Key::ArrowLeft => game.player.do_move(Direction::Left),
                    Key::ArrowRigth => game.player.do_move(Direction::Right),
                    _ => (),
                }
            }
        }
    }

    fn on_key_up(key: Key) {}
}
