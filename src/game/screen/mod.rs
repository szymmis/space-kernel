mod game_over_screen;
mod game_screen;
mod menu_screen;

use crate::kernel::system::Key;

pub enum ActiveScreen {
    MainMenu,
    Game,
    #[allow(dead_code)]
    GameOver,
}

pub trait Screen {
    fn draw();
    fn update();
    fn on_key_down(key: Key);
    fn on_key_up(key: Key);
}

pub use self::game_over_screen::GameOverScreen;
pub use self::game_screen::GameScreen;
pub use self::menu_screen::MenuScreen;
