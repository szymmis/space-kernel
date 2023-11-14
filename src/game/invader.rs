use super::{
    assets::{GOLIATH_SPRITE, INVADER_SPRITE, SQUID_SPRITE},
    player::Direction,
    GAME,
};
use crate::kernel::display::draw::draw_bitmap;

#[derive(Clone, Copy)]
pub struct Invader {
    pub x: i32,
    pub y: i32,
    pub dead: bool,
    ty: InvaderType,
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

    pub fn draw(&self) {
        if self.dead {
            return;
        }

        match self.ty {
            InvaderType::Invader => draw_bitmap(&INVADER_SPRITE, self.x, self.y, 11, 8),
            InvaderType::Goliath => draw_bitmap(&GOLIATH_SPRITE, self.x, self.y, 12, 8),
            InvaderType::Squid => draw_bitmap(&SQUID_SPRITE, self.x, self.y, 10, 8),
        }
    }

    pub fn update(&mut self) {
        if self.dead {
            return;
        }

        unsafe {
            GAME.player.check_collision(self);
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
