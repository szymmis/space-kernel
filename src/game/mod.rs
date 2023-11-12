mod assets;
mod invader;
mod player;
mod projectile;
mod screen;

use crate::{
    game::{
        player::Player,
        screen::{ActiveScreen, GameOverScreen, GameScreen, MenuScreen, Screen},
    },
    kernel::{
        display::logger::cls,
        mem::vec::Vec,
        system::{Key, Keyboard, Timer},
    },
};

use self::{
    invader::{Invader, InvaderType},
    player::Direction,
};

static mut GAME: Option<Game> = None;
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
    pub fn init() {
        unsafe {
            let mut game = Game {
                ticks: 0,
                screen: ActiveScreen::Game,
                player: Player::new(),
                invaders: Vec::new(5 * 11),

                movement_count: 4,
                movement_direction: Direction::Right,
            };
            game.init_invaders();

            GAME = Some(game);

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
