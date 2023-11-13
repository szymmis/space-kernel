use crate::{
    game::{player::Direction, GAME, INTERVAL},
    kernel::system::Key,
};

use super::Screen;

pub struct GameScreen;
impl Screen for GameScreen {
    fn draw() {
        unsafe {
            GAME.player.draw();

            for i in 0..GAME.invaders.len() {
                let invader = GAME.invaders.get(i);
                invader.draw();
            }
        }
    }

    fn update() {
        unsafe {
            GAME.player.update();

            for i in 0..GAME.invaders.len() {
                let invader = GAME.invaders.get(i);

                if invader.dead {
                    continue;
                }

                GAME.player.check_collision(invader);

                invader.update();
            }

            if GAME.ticks % INTERVAL == 0 {
                GAME.movement_count += 1;
                if GAME.movement_count >= 8 {
                    GAME.movement_count = 0;
                    GAME.movement_direction = match GAME.movement_direction {
                        Direction::Left => Direction::Right,
                        Direction::Right => Direction::Left,
                    };
                }
            }
        }
    }

    fn on_key_down(key: Key) {
        unsafe {
            match key {
                Key::ArrowLeft => GAME.player.do_move(Direction::Left),
                Key::ArrowRigth => GAME.player.do_move(Direction::Right),
                _ => (),
            }
        }
    }

    fn on_key_up(key: Key) {
        unsafe {
            if let Key::Spacebar = key {
                GAME.player.shoot();
            }
        }
    }
}
