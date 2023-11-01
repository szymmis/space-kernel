#include "display/draw.c"
#include "display/print.c"
#include "game/player.c"
#include "game/invader.c"
#include "std/memory.c"
#include "std/vector.c"
#include "system/interrupts.c"
#include "system/keyboard.c"
#include "system/timer.c"

struct Player *player;
struct Vector *invaders;

int started;

int tick_count;
int movement_count;
int movement_direction;
int interval;

void key_down(int scancode)
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

void key_up(int scancode)
{
    if (scancode == SPACEBAR)
    {
        shoot(player);
    }
    else if (scancode == ENTER)
    {
        started = 1;
        tick_count = 0;
    }
}

void tick()
{
    tick_count++;

    if (started == 0)
    {
        clear_screen();
        if ((tick_count % 40) > 10)
        {
            draw_text("press enter to start", SCREEN_WIDTH / 2 - (20 / 2 * GLYPH_WIDTH), SCREEN_HEIGHT / 2 - GLYPH_HEIGHT);
        }
        return;
    }

    clear_screen();
    if (player->projectile->y > -10)
    {
        player->projectile->y -= 8;
    }
    draw_player(player);
    for (int i = 0; i < invaders->length; i++)
    {
        struct Invader *inv = (struct Invader *)vec_get(invaders, i);

        if (inv->dead == 0)
        {
            draw_invader(inv);

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
}

void main()
{
    init_keyboard();
    on_key_down(&key_down);
    on_key_up(&key_up);

    player = init_player();
    invaders = vec_create(sizeof(struct Invader *), 10);

    movement_count = 4;
    movement_direction = 1;
    interval = 40;

    int started = 0;

    for (int i = 0; i < 5; i++)
    {
        for (int j = 0; j < 11; j++)
        {
            char type = 0;
            if (i == 0)
            {
                type = 1;
            }
            else if (i >= 3)
            {
                type = 2;
            }
            struct Invader *inv = create_invader((320 - 11 * 15) / 2 + (j * 15), 20 + 15 * i, type);
            vec_push(invaders, inv);
        }
    }

    init_timer();
    on_timer_int(&tick);
}
