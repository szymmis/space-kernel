#include "../system/keyboard.h"

#include "game.h"
#include "player.h"

void on_key_down(int scancode)
{
    if (scancode == ARROW_LEFT)
    {
        player->x -= 2;
    }
    else if (scancode == ARROW_RIGHT)
    {
        player->x += 2;
    }
}

void on_key_up(int scancode)
{
    if (scancode == SPACEBAR)
    {
        shoot(player);
    }
    else if (scancode == ENTER)
    {
        active_screen = GAME;
        tick_count = 0;
    }
}
