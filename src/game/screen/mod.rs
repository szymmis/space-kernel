mod game;
mod game_over;
mod menu;

use crate::kernel::system::Key;

pub enum ActiveScreen {
    MainMenu,
    Game,
    GameOver,
}

pub trait Screen {
    fn draw();
    fn update();
    fn on_key_down(key: Key);
    fn on_key_up(key: Key);
}

pub use self::game::GameScreen;
pub use self::game_over::GameOverScreen;
pub use self::menu::MenuScreen;
