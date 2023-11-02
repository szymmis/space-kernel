#include "../display/draw.h"
#include "../display/print.h"
#include "../std/vector.h"

#include "keyboard.h"

struct Vector *key_down_listeners;
struct Vector *key_up_listeners;

void init_keyboard()
{
    key_down_listeners = vec_create(5);
    key_up_listeners = vec_create(5);
}

void on_key_down(void (*f)(int))
{
    vec_push(key_down_listeners, f);
}

void on_key_up(void (*f)(int))
{
    vec_push(key_up_listeners, f);
}

void handle_keyboard_input(int scancode)
{
    // Key was released if the highest bit is set
    // Seven lower bits form the base code
    // We use 01111111 bit mask to extract it
    if (scancode >= 128)
    {
        for (int i = 0; i < key_up_listeners->length; i++)
        {
            ((void (*)(int))vec_get(key_up_listeners, i))(0x7F & scancode);
        }
    }
    else
    {
        for (int i = 0; i < key_down_listeners->length; i++)
        {
            ((void (*)(int))vec_get(key_down_listeners, i))(scancode);
        }
    }
}
