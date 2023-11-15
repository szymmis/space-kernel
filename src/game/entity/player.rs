use super::{invader::Invader, projectile::Projectile, Direction, Entity};
use crate::{game::GAME, kernel::display::draw::SCREEN_WIDTH};
use game::assets::PLAYER_SPRITE;
use kernel::display::draw::draw_bitmap;

const MOVE_SPEED: i32 = 3;

pub struct Player {
    x: i32,
    y: i32,
    projectile: Projectile,
    movement: Option<Direction>,
    pub lives: i32,
}

impl Entity for Player {
    fn draw(&self) {
        draw_bitmap(&PLAYER_SPRITE, self.x, self.y, 5, 5, 0xA);
        self.projectile.draw();
    }

    fn update(&mut self) {
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

    fn reset(&mut self) {
        self.x = 320 / 2 - 2;
        self.projectile.reset();
        self.movement = None;
    }
}

impl Player {
    pub fn new() -> Self {
        Self {
            x: 320 / 2 - 2,
            y: 175,
            projectile: Projectile::new(),
            movement: None,
            lives: 3,
        }
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
            unsafe {
                invader.dead = true;
                self.projectile.reset();
                GAME.explosion.explode(invader.x, invader.y);
                GAME.score += 1;
                GAME.swarm.destroyed_count += 1;
            }
        }
    }
}
