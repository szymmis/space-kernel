use kernel::{
    display::logger::{cls, print, print_num},
    system::{Keyboard, Timer},
};

static mut GAME: Option<Game> = None;

pub struct Game {
    ticks: u32,
}

impl Game {
    pub fn init() {
        unsafe {
            GAME = Some(Game { ticks: 0 });
        }

        Keyboard::add_on_key_down_listener(|_| {
            cls();
            print("Key down");
        });

        Keyboard::add_on_key_up_listener(|_| {
            cls();
            print("Key up");
        });

        Timer::add_timer_listener(|| unsafe {
            if let Some(game) = &mut GAME {
                game.on_tick();
            }
        })
    }

    pub fn on_tick(&mut self) {
        self.ticks += 1;

        self.draw();
        self.update();
    }

    fn draw(&self) {
        cls();
        print_num(self.ticks as i32);
    }

    fn update(&self) {}
}
