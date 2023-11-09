#ifndef PLAYER_H
#define PLAYER_H

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

struct Player* create_player();
void draw_player(struct Player* p);
void shoot(struct Player* p);

#endif
