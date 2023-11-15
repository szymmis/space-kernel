use super::Screen;
use crate::{
    game::{
        assets::PLAYER_SPRITE,
        entity::{Direction, Entity},
        GAME,
    },
    kernel::{
        display::{
            draw::{draw_bitmap, draw_number, draw_rect, draw_text, SCREEN_HEIGHT, SCREEN_WIDTH},
            logger::{print, print_num},
        },
        system::Key,
    },
};

pub struct GameScreen;
impl Screen for GameScreen {
    fn draw() {
        unsafe {
            GAME.player.draw();
            GAME.swarm.draw();
            GAME.explosion.draw();

            draw_text("SCORE", 5, 5);
            draw_number(GAME.score, 5, 13);

            draw_rect(0, SCREEN_HEIGHT - 15, SCREEN_WIDTH, 1, 0xA);
            draw_number(GAME.player.lives, 5, SCREEN_HEIGHT - 10);
            for i in 0..GAME.player.lives {
                draw_bitmap(&PLAYER_SPRITE, 15 + 10 * i, SCREEN_HEIGHT - 10, 5, 5, 0xA);
            }
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
