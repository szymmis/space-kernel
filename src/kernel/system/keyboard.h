#define ENTER 0x1C
#define SPACEBAR 0x39

#define ARROW_UP 0x48
#define ARROW_DOWN 0x50
#define ARROW_LEFT 0x4B
#define ARROW_RIGHT 0x4D

void handle_keyboard_input(int scancode);

void on_key_down(int keycode);
void on_key_up(int keycode);
