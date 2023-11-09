#ifndef GAME_H
#define GAME_H

extern struct Player* player;
extern struct Vector* invaders;

extern int active_screen;
extern int tick_count;

enum Screen
{
    MAIN_MENU,
    GAME,
    GAME_OVER
};

void init_invaders();

#endif
