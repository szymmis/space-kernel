#include "../std/memory.h"
#include "../display/draw.h"

struct Player 
{
    int x;
    int y;
    struct Projectile* projectile;
};

struct Projectile 
{
    int x;
    int y;
};


struct Player* init_player()
{
    struct Player* player = malloc(sizeof(struct Player*));
    player->x = 150;
    player->y = 180;
    player->projectile = malloc(sizeof(struct Projectile*));

    return player;
}

void draw_player(struct Player* p)
{
    char sprite[] = { 
        0, 0, 1, 0, 0,
        0, 1, 1, 1, 0,
        0, 1, 1, 1, 0,
        1, 1, 1, 1, 1,
        1, 1, 1, 1, 1,
    };
    
    draw_img(sprite, p->x, p->y, 5, 5);
    draw_rect(p->projectile->x, p->projectile->y, 1, 5);
}

void shoot(struct Player* p)
{
    if(p->projectile->y < 0) 
    {
        p->projectile->x = p->x + 2;
        p->projectile->y = p->y - 5;
    }
}
