#ifndef INVADER_H
#define INVADER_H

enum InvaderType {
    INVADER,
    SQUID,
    GOLIATH,
};

struct Invader
{
    int x;
    int y;
    char dead;
    char type;
};

struct Invader* create_invader(int x, int y, char type);
void draw_invader(struct Invader* i);

#endif
