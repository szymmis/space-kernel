use super::{invader::Invader, projectile::Projectile};
use game::assets::PLAYER_SPRITE;
use kernel::display::draw::draw_bitmap;

const MOVE_SPEED: i32 = 2;

pub struct Player {
    x: i32,
    y: i32,
    projectile: Projectile,
}

impl Player {
    pub fn new() -> Self {
        Self {
            x: 320 / 2 - 2,
            y: 180,
            projectile: Projectile::new(),
        }
    }

    pub fn draw(&self) {
        draw_bitmap(&PLAYER_SPRITE, self.x, 180, 5, 5);
        self.projectile.draw();
    }

    pub fn update(&mut self) {
        self.projectile.update();
    }

    pub fn shoot(&mut self) {
        self.projectile.launch((self.x + 2, self.y - 5))
    }

    pub fn do_move(&mut self, direction: Direction) {
        self.x += match direction {
            Direction::Left => -MOVE_SPEED,
            Direction::Right => MOVE_SPEED,
        }
    }

    pub fn check_collision(&mut self, invader: &mut Invader) {
        if (self.projectile.x >= invader.x && self.projectile.x <= invader.x + 11)
            && (self.projectile.y >= invader.y && self.projectile.y <= invader.y + 8)
        {
            invader.dead = true;
            self.projectile.y = -10;
        }
    }
}

pub enum Direction {
    Left,
    Right,
}
