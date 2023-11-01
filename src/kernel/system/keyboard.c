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
    cls();
    if (keycode == ARROW_LEFT)
    {
        print("Left");
    }
    else if (keycode == ARROW_RIGHT)
    {
        print("Right");
    }
    else if (keycode == ARROW_UP)
    {
        print("Up");
    }
    else if (keycode == ARROW_DOWN)
    {
        print("Down");
    }
    else if (keycode == ENTER)
    {
        print("Enter");
    }
    else if (keycode == SPACEBAR)
    {
        print("Space");
    }
    else
    {
        print("Key Down");
    }
}

void on_key_up(int keycode)
{
    cls();
    if (keycode == SPACEBAR)
    {
        print("Space Up");
    }
    else if (keycode == ENTER)
    {
        print("Enter Up");
    }
    else
    {
        print("Key Up");
    }
}
