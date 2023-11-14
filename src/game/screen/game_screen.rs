use crate::{
    game::{player::Direction, GAME},
    kernel::{
        display::logger::{print, print_num},
        system::Key,
    },
};

use super::Screen;

pub struct GameScreen;
impl Screen for GameScreen {
    fn draw() {
        unsafe {
            GAME.player.draw();
            GAME.swarm.draw();
            GAME.explosion.draw();

            print("SCORE");
            print_num(GAME.score);
        }
    }

    fn update() {
        unsafe {
            GAME.player.update();
            GAME.swarm.update();
            GAME.explosion.update();

            if GAME.swarm.destroyed() {
                GAME.reset();
            }
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
