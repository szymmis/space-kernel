use game::assets::PLAYER_SPRITE;
use kernel::display::draw::draw_bitmap;

const MOVE_SPEED: i32 = 2;

pub struct Player {
    x: i32,
}

impl Player {
    pub fn new() -> Self {
        Self { x: 320 / 2 - 2 }
    }

    pub fn draw(&self) {
        draw_bitmap(&PLAYER_SPRITE, self.x, 180, 5, 5);
    }

    pub fn do_move(&mut self, direction: Direction) {
        self.x += match direction {
            Direction::Left => -MOVE_SPEED,
            Direction::Right => MOVE_SPEED,
        }
    }
}

pub enum Direction {
    Left,
    Right,
}
