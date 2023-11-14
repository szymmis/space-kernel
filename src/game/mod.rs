mod assets;
mod invader;
mod player;
mod projectile;
mod screen;

use self::{
    invader::{Invader, InvaderType},
    player::Direction,
};
use crate::{
    game::{
        player::Player,
        screen::{ActiveScreen, GameOverScreen, GameScreen, MenuScreen, Screen},
    },
    kernel::{
        display::logger::cls,
        mem::{boxed::LazyBox, vec::Vec},
        system::{Key, Keyboard, Timer},
    },
};

static mut GAME: LazyBox<Game> = LazyBox::new(Game::new);
static mut INTERVAL: u32 = 40;

pub struct Game {
    ticks: u32,
    screen: ActiveScreen,
    player: Player,
    invaders: Vec<Invader>,

    movement_count: i32,
    movement_direction: Direction,
}

impl Game {
    fn new() -> Self {
        Game {
            ticks: 0,
            screen: ActiveScreen::MainMenu,
            player: Player::new(),
            invaders: Vec::new(5 * 11),
            movement_count: 4,
            movement_direction: Direction::Right,
        }
    }

    pub fn init() {
        unsafe {
            GAME.init_invaders();
            Keyboard::add_on_key_down_listener(|key| GAME.on_key_down(key));
            Keyboard::add_on_key_up_listener(|key| GAME.on_key_up(key));
            Timer::add_timer_listener(|| GAME.on_tick())
        }
    }

    fn init_invaders(&mut self) {
        for i in 0..5 {
            for j in 0..11 {
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
