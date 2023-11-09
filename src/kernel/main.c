#include "../game/game.c"

#include "display/draw.c"
#include "display/print.c"
#include "std/memory.c"
#include "std/vector.c"
#include "system/interrupts.c"
#include "system/keyboard.c"
#include "system/timer.c"

void main()
{
    init_keyboard();
    init_timer();

    init_game();
}
