use crate::game::{
    assets::{
        GOLIATH_2_SPRITE, GOLIATH_SPRITE, INVADER_2_SPRITE, INVADER_SPRITE, SQUID_2_SPRITE,
        SQUID_SPRITE,
    },
    entity::Direction,
    GAME,
};
use crate::kernel::display::draw::draw_bitmap;

use super::Entity;

#[derive(Clone, Copy)]
pub struct Invader {
    pub x: i32,
    pub y: i32,
    pub dead: bool,
    ty: InvaderType,
}

impl Entity for Invader {
    fn draw(&self) {
        if self.dead {
            return;
        }

        unsafe {
            let odd_step = GAME.swarm.movement_count % 2 == 0;
            match self.ty {
                InvaderType::Invader => draw_bitmap(
                    if odd_step {
                        &INVADER_2_SPRITE
                    } else {
                        &INVADER_SPRITE
                    },
                    self.x,
                    self.y,
                    11,
                    8,
                    0xF,
                ),
                InvaderType::Goliath => draw_bitmap(
                    if odd_step {
                        &GOLIATH_2_SPRITE
                    } else {
                        &GOLIATH_SPRITE
                    },
                    self.x,
                    self.y,
                    12,
                    8,
                    0xF,
                ),
                InvaderType::Squid => draw_bitmap(
                    if odd_step {
                        &SQUID_2_SPRITE
                    } else {
                        &SQUID_SPRITE
                    },
                    self.x,
                    self.y,
                    10,
                    8,
                    0xF,
                ),
            }
        }
    }

    fn update(&mut self) {
        if self.dead {
            return;
        }

        unsafe {
            GAME.player.check_collision(self);
        }
    }

    fn reset(&mut self) {}
}

impl Invader {
    pub fn new(x: i32, y: i32, ty: InvaderType) -> Self {
        Self {
            x,
            y,
            ty,
            dead: false,
        }
    }

    pub fn do_move(&mut self, direction: &Direction, speed: i32) {
        self.x += match direction {
            Direction::Left => -speed,
            Direction::Right => speed,
        };
    }
}

#[derive(Clone, Copy)]
pub enum InvaderType {
    Invader,
    Goliath,
    Squid,
}
