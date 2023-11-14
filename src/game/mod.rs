mod assets;
mod invader;
mod player;
mod projectile;
mod screen;
mod swarm;

use self::swarm::Swarm;
use crate::{
    game::{
        player::Player,
        screen::{ActiveScreen, GameOverScreen, GameScreen, MenuScreen, Screen},
    },
    kernel::{
        display::logger::cls,
        mem::boxed::LazyBox,
        system::{Key, Keyboard, Timer},
    },
};

static mut GAME: LazyBox<Game> = LazyBox::new(Game::new);

pub struct Game {
    ticks: u32,
    screen: ActiveScreen,
    player: Player,
    swarm: Swarm,
}

impl Game {
    fn new() -> Self {
        Game {
            ticks: 0,
            screen: ActiveScreen::MainMenu,
            player: Player::new(),
            swarm: Swarm::new(),
        }
    }

    pub fn init() {
        unsafe {
            Keyboard::add_on_key_down_listener(|key| GAME.on_key_down(key));
            Keyboard::add_on_key_up_listener(|key| GAME.on_key_up(key));
            Timer::add_timer_listener(|| GAME.on_tick());
            GAME.swarm.init();
        }
    }

    fn draw(&mut self) {
        match self.screen {
            ActiveScreen::MainMenu => MenuScreen::draw(),
            ActiveScreen::Game => GameScreen::draw(),
            ActiveScreen::GameOver => GameOverScreen::draw(),
        }
    }

    fn update(&self) {
        match self.screen {
            ActiveScreen::MainMenu => MenuScreen::update(),
            ActiveScreen::Game => GameScreen::update(),
            ActiveScreen::GameOver => GameOverScreen::update(),
        }
    }

    pub fn on_tick(&mut self) {
        self.ticks += 1;

        cls();
        self.draw();
        self.update();
    }

    fn on_key_down(&mut self, key: Key) {
        match self.screen {
            ActiveScreen::MainMenu => MenuScreen::on_key_down(key),
            ActiveScreen::Game => GameScreen::on_key_down(key),
            ActiveScreen::GameOver => GameOverScreen::on_key_down(key),
        }
    }

    fn on_key_up(&mut self, key: Key) {
        match self.screen {
            ActiveScreen::MainMenu => MenuScreen::on_key_up(key),
            ActiveScreen::Game => GameScreen::on_key_up(key),
            ActiveScreen::GameOver => GameOverScreen::on_key_up(key),
        }
    }
}
