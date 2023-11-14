use super::{invader::Invader, projectile::Projectile, GAME};
use crate::kernel::display::draw::SCREEN_WIDTH;
use game::assets::PLAYER_SPRITE;
use kernel::display::draw::draw_bitmap;

const MOVE_SPEED: i32 = 3;

pub struct Player {
    x: i32,
    y: i32,
    projectile: Projectile,
    movement: Option<Direction>,
}

impl Player {
    pub fn new() -> Self {
        Self {
            x: 320 / 2 - 2,
            y: 180,
            projectile: Projectile::new(),
            movement: None,
        }
    }

    pub fn draw(&self) {
        draw_bitmap(&PLAYER_SPRITE, self.x, 180, 5, 5);
        self.projectile.draw();
    }

    pub fn update(&mut self) {
        match self.movement {
            Some(Direction::Left) => {
                if self.x - MOVE_SPEED > 0 {
                    self.x += -MOVE_SPEED;
                } else {
                    self.x = 0;
                }
            }
            Some(Direction::Right) => {
                if self.x + MOVE_SPEED < SCREEN_WIDTH - 5 {
                    self.x += MOVE_SPEED;
                } else {
                    self.x = SCREEN_WIDTH - 5;
                }
            }
            None => (),
        };

        self.projectile.update();
    }

    pub fn shoot(&mut self) {
        self.projectile.launch((self.x + 2, self.y - 5))
    }

    pub fn do_move(&mut self, direction: Option<Direction>) {
        self.movement = direction;
    }

    pub fn check_collision(&mut self, invader: &mut Invader) {
        if (self.projectile.x >= invader.x && self.projectile.x <= invader.x + 11)
            && (self.projectile.y >= invader.y && self.projectile.y <= invader.y + 8)
        {
            invader.dead = true;
            unsafe {
                GAME.explosion.explode(invader.x, invader.y);
            }
            self.projectile.y = -10;
        }
    }
}

pub enum Direction {
    Left,
    Right,
}

impl Direction {
    pub fn swap(&self) -> Self {
        match self {
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}
