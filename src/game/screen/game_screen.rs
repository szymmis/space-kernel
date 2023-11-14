use crate::{
    game::{player::Direction, GAME},
    kernel::system::Key,
};

use super::Screen;

pub struct GameScreen;
impl Screen for GameScreen {
    fn draw() {
        unsafe {
            GAME.player.draw();
            GAME.swarm.draw();
        }
    }

    fn update() {
        unsafe {
            GAME.player.update();
            GAME.swarm.update();
        }
    }

    fn on_key_down(key: Key) {
        unsafe {
            match key {
                Key::ArrowLeft => GAME.player.do_move(Some(Direction::Left)),
                Key::ArrowRigth => GAME.player.do_move(Some(Direction::Right)),
                _ => (),
            }
        }
    }

    fn on_key_up(key: Key) {
        unsafe {
            match key {
                Key::Spacebar => {
                    GAME.player.shoot();
                }
                Key::ArrowLeft | Key::ArrowRigth => GAME.player.do_move(None),
                _ => (),
            }
        }
    }
}
