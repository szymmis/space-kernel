pub mod explosion;
pub mod invader;
pub mod player;
pub mod projectile;
pub mod swarm;

pub use self::explosion::Explosion;
pub use self::invader::Invader;
pub use self::player::Player;
pub use self::projectile::Projectile;
pub use self::swarm::Swarm;

pub trait Entity {
    fn draw(&self);
    fn update(&mut self);
    fn reset(&mut self);
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
