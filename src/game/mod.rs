mod assets;
mod player;
mod screen;

use crate::{
    game::{
        player::Player,
        screen::{ActiveScreen, GameOverScreen, GameScreen, MenuScreen, Screen},
    },
    kernel::{
        display::logger::cls,
        system::{Key, Keyboard, Timer},
    },
};

static mut GAME: Option<Game> = None;

pub struct Game {
    ticks: u32,
    screen: ActiveScreen,
    player: Player,
}

impl Game {
    pub fn init() {
        unsafe {
            GAME = Some(Game {
                ticks: 0,
                screen: ActiveScreen::MainMenu,
                player: Player::new(),
            });

            Keyboard::add_on_key_down_listener(|key| {
                if let Some(game) = &mut GAME {
                    game.on_key_up(key)
                }
            });

            Keyboard::add_on_key_up_listener(|key| {
                if let Some(game) = &mut GAME {
                    game.on_key_down(key)
                }
            });

            Timer::add_timer_listener(|| {
                if let Some(game) = &mut GAME {
                    game.on_tick();
                }
            })
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
