use crate::kernel::display::draw::draw_bitmap;

use super::assets::EXPLOSION_SPRITE;

pub struct Explosion {
    x: i32,
    y: i32,
    lifespan: i32,
}

impl Explosion {
    pub fn new() -> Self {
        Self {
            x: 0,
            y: -10,
            lifespan: 0,
        }
    }

    pub fn explode(&mut self, x: i32, y: i32) {
        self.x = x;
        self.y = y;
        self.lifespan = 3;
    }

    pub fn draw(&self) {
        if self.lifespan > 0 {
            draw_bitmap(&EXPLOSION_SPRITE, self.x, self.y, 13, 9);
        }
    }

    pub fn update(&mut self) {
        if self.lifespan > 0 {
            self.lifespan -= 1;
        }
    }
}
