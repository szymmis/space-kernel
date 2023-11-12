use crate::kernel::display::draw::draw_rect;

pub struct Projectile {
    pub x: i32,
    pub y: i32,
}

impl Projectile {
    pub fn new() -> Self {
        Self { x: 0, y: 0 }
    }

    pub fn draw(&self) {
        draw_rect(self.x, self.y, 1, 5, 0xF);
    }

    pub fn update(&mut self) {
        if self.y > -10 {
            self.y -= 5;
        }
    }

    fn can_launch(&self) -> bool {
        self.y < 0
    }

    pub fn launch(&mut self, (x, y): (i32, i32)) {
        if self.can_launch() {
            self.x = x;
            self.y = y;
        }
    }
}
