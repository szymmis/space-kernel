use super::{
    invader::{Invader, InvaderType},
    player::Direction,
    GAME,
};
use crate::kernel::{display::draw::SCREEN_WIDTH, mem::vec::Vec};

static SWARM_WIDTH: i32 = 11;
static SWARM_HEIGHT: i32 = 5;

static MOVE_SPEED: i32 = 4;
static mut MOVE_INTERVAL: u32 = 20;

pub struct Swarm {
    invaders: Vec<Invader>,
    pub movement_count: i32,
    direction: Direction,
}

impl Swarm {
    pub fn new() -> Self {
        Self {
            invaders: Vec::new(5 * 11),
            movement_count: 0,
            direction: Direction::Right,
        }
    }

    pub fn init(&mut self) {
        for i in 0..SWARM_HEIGHT {
            for j in 0..SWARM_WIDTH {
                let ty = if i == 0 {
                    InvaderType::Squid
                } else if i < 3 {
                    InvaderType::Invader
                } else {
                    InvaderType::Goliath
                };

                self.invaders.push(Invader::new(
                    (320 - 11 * 15) / 2 + (j * 15),
                    20 + 15 * i,
                    ty,
                ))
            }
        }
    }

    pub fn draw(&mut self) {
        for i in 0..self.invaders.len() {
            let invader = self.invaders.get(i);
            invader.draw();
        }
    }

    fn at_edge(&mut self) -> bool {
        match self.direction {
            Direction::Left => {
                for j in 0..SWARM_WIDTH {
                    for i in 0..SWARM_HEIGHT {
                        let invader = self.invaders.get((j + i * SWARM_WIDTH) as usize);
                        if !invader.dead {
                            return invader.x <= MOVE_SPEED * 2;
                        }
                    }
                }
            }
            Direction::Right => {
                for j in (0..SWARM_WIDTH).rev() {
                    for i in 0..SWARM_HEIGHT {
                        let invader = self.invaders.get((j + i * SWARM_WIDTH) as usize);
                        if !invader.dead {
                            return invader.x + 12 >= SCREEN_WIDTH - MOVE_SPEED * 2;
                        }
                    }
                }
            }
        }

        false
    }

    pub fn update(&mut self) {
        for i in 0..self.invaders.len() {
            let invader = self.invaders.get(i);
            invader.update();

            unsafe {
                if GAME.ticks % MOVE_INTERVAL == 0 {
                    invader.do_move(&self.direction, MOVE_SPEED);
                }
            }
        }

        unsafe {
            if GAME.ticks % MOVE_INTERVAL == 0 {
                self.movement_count += 1;
            }
        }

        if self.at_edge() {
            self.direction = self.direction.swap()
        }
    }
}
