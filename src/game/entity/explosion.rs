use super::Entity;
use crate::game::assets::EXPLOSION_SPRITE;
use crate::kernel::display::draw::draw_bitmap;

pub struct Explosion {
    x: i32,
    y: i32,
    lifespan: i32,
}

impl Entity for Explosion {
    fn draw(&self) {
        if self.lifespan > 0 {
            draw_bitmap(&EXPLOSION_SPRITE, self.x, self.y, 13, 9);
        }
    }

    fn update(&mut self) {
        if self.lifespan > 0 {
            self.lifespan -= 1;
        }
    }

    fn reset(&mut self) {
        self.lifespan = 0;
    }
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
}
