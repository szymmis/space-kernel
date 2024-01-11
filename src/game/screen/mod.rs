mod game_over_screen;
mod game_screen;
mod menu_screen;

use crate::kernel::system::Key;

pub static MENU_SCREEN: MenuScreen = MenuScreen;
pub static GAME_SCREEN: GameScreen = GameScreen;
#[allow(dead_code)]
pub static GAME_OVER_SCREEN: GameOverScreen = GameOverScreen;

pub trait Screen {
    fn draw(&self);
    fn update(&self);
    fn on_key_down(&self, key: Key);
    fn on_key_up(&self, key: Key);
}

pub use self::game_over_screen::GameOverScreen;
pub use self::game_screen::GameScreen;
pub use self::menu_screen::MenuScreen;
