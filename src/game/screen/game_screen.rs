use crate::{
    game::{player::Direction, GAME, INTERVAL},
    kernel::system::Key,
};

use super::Screen;

pub struct GameScreen;
impl Screen for GameScreen {
    fn draw() {
        unsafe {
            if let Some(game) = &mut GAME {
                game.player.draw();

                for i in 0..game.invaders.len() {
                    let invader = game.invaders.get(i);
                    invader.draw();
                }
            }
        }
    }

    fn update() {
        unsafe {
            if let Some(game) = &mut GAME {
                game.player.update();

                for i in 0..game.invaders.len() {
                    let invader = game.invaders.get(i);

                    if invader.dead {
                        continue;
                    }

                    game.player.check_collision(invader);

                    invader.update();
                }

                if game.ticks % INTERVAL == 0 {
                    game.movement_count += 1;
                    if game.movement_count >= 8 {
                        game.movement_count = 0;
                        game.movement_direction = match game.movement_direction {
                            Direction::Left => Direction::Right,
                            Direction::Right => Direction::Left,
                        };
                    }
                }
            }
        }
    }

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

    fn on_key_up(key: Key) {
        unsafe {
            if let Some(game) = &mut GAME {
                if let Key::Spacebar = key {
                    game.player.shoot();
                }
            }
        }
    }
}
