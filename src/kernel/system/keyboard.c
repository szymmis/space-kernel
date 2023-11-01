#include "../display/draw.h"
#include "../display/print.h"

#include "keyboard.h"

void handle_keyboard_input(int scancode)
{
    // Key was released if the highest bit is set
    // Seven lower bits form the base code
    // We use 01111111 bit mask to extract it
    if (scancode >= 128)
    {
        on_key_up(0x7F & scancode);
    }
    else
    {
        on_key_down(scancode);
    }
}

void on_key_down(int keycode)
{
    clear_screen();
    if (keycode == ARROW_LEFT)
    {
        print("Left", 10, 10);
    }
    else if (keycode == ARROW_RIGHT)
    {
        print("Right", 10, 10);
    }
    else if (keycode == ARROW_UP)
    {
        print("Up", 10, 10);
    }
    else if (keycode == ARROW_DOWN)
    {
        print("Down", 10, 10);
    }
    else if (keycode == ENTER)
    {
        print("Enter", 10, 10);
    }
    else if (keycode == SPACEBAR)
    {
        print("Space", 10, 10);
    }
    else
    {
        print("Key Down", 10, 10);
    }
}

void on_key_up(int keycode)
{
    clear_screen();
    if (keycode == SPACEBAR)
    {
        print("Space Up", 10, 10);
    }
    else if (keycode == ENTER)
    {
        print("Enter Up", 10, 10);
    }
    else
    {
        print("Key Up", 10, 10);
    }
}
