#include "../kernel/system/keyboard.h"
#include "../kernel/system/timer.h"
#include "../kernel/std/vector.h"
#include "../kernel/display/draw.h"
#include "../kernel/display/print.h"

#include "input.c"
#include "invader.c"
#include "player.c"

#include "game.h"

struct Player *player;
struct Vector *invaders;

int active_screen;

int tick_count;
int movement_count;
int movement_direction;
int interval;

void draw()
{
    clear_screen();

    switch (active_screen)
    {
    case MAIN_MENU:
        if ((tick_count % 40) < 25)
        {
            char *str = "press enter to start";
            draw_text(str, SCREEN_WIDTH / 2 - (measure_str(str) / 2), SCREEN_HEIGHT / 2 - GLYPH_HEIGHT);
        }
        break;

    case GAME:
        draw_player(player);
        for (int i = 0; i < invaders->length; i++)
        {
            struct Invader *inv = (struct Invader *)vec_get(invaders, i);
            draw_invader(inv);
        }

        break;
    }
}

void update()
{
    switch (active_screen)
    {
    case GAME:
        if (player->projectile->y > -10)
        {
            player->projectile->y -= 8;
        }
        for (int i = 0; i < invaders->length; i++)
        {
            struct Invader *inv = (struct Invader *)vec_get(invaders, i);

            if (inv->dead == 0)
            {
                int x = player->projectile->x;
                int y = player->projectile->y;
                if (x >= inv->x && x <= inv->x + 11)
                {
                    if (y >= inv->y && y <= inv->y + 8)
                    {
                        inv->dead = 1;
                        player->projectile->y = -10;
                        continue;
                    }
                }

                if (tick_count % interval == 0)
                {
                    inv->x += 10 * movement_direction;
                    if (movement_count >= 7)
                    {
                        inv->y += 15;
                    }
                }
            }
        }

        if (tick_count % interval == 0)
        {
            if (++movement_count >= 8)
            {
                movement_count = 0;
                movement_direction *= -1;
                if (interval > 20)
                {
                    interval -= 5;
                }
            }
        }

        break;
    }
}

void on_tick()
{
    tick_count++;

    draw();
    update();
}

void init_player()
{
    player = create_player();
    player->projectile->y = -10;
}

void init_invaders()
{
    invaders = vec_create(5 * 11);
    movement_count = 4;
    movement_direction = 1;
    interval = 40;

    for (int i = 0; i < 5; i++)
    {
        for (int j = 0; j < 11; j++)
        {
            char type;
            if (i == 0)
            {
                type = SQUID;
            }
            else if (i < 3)
            {
                type = INVADER;
            }
            else
            {
                type = GOLIATH;
            }

            struct Invader *inv = create_invader((320 - 11 * 15) / 2 + (j * 15), 20 + 15 * i, type);
            vec_push(invaders, inv);
        }
    }
}

void init_game()
{
    add_key_down_listener(&on_key_down);
    add_key_up_listener(&on_key_up);
    add_timer_int_listener(&on_tick);

    init_player();
    init_invaders();
}
