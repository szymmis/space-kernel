mod assets;
mod entity;
mod screen;

use crate::{
    game::{
        entity::{Entity, Explosion, Player, Swarm},
        screen::{Screen, MENU_SCREEN},
    },
    kernel::{
        display::logger,
        mem::boxed::LazyBox,
        system::{Keyboard, Timer},
    },
};

static mut GAME: LazyBox<Game> = LazyBox::new(Game::new);

pub struct Game {
    ticks: u32,
    score: i32,
    screen: &'static dyn Screen,
    player: Player,
    swarm: Swarm,
    explosion: Explosion,
}

impl Game {
    fn new() -> Self {
        Game {
            ticks: 0,
            score: 0,
            screen: &MENU_SCREEN,
            player: Player::new(),
            swarm: Swarm::new(),
            explosion: Explosion::new(),
        }
    }

    pub fn init() {
        unsafe {
            Keyboard::add_on_key_down_listener(|key| GAME.screen.on_key_down(key));
            Keyboard::add_on_key_up_listener(|key| GAME.screen.on_key_up(key));
            Timer::add_timer_listener(|| GAME.on_tick());
            GAME.swarm.init();
        }
    }

    pub fn reset(&mut self) {
        self.ticks = 0;
        self.player.reset();
        self.swarm.reset();
        self.explosion.reset();
    }

    pub fn on_tick(&mut self) {
        self.ticks += 1;

        logger::cls();
        self.screen.draw();
        self.screen.update();
    }
}
