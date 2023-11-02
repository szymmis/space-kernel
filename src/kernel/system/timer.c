#include "timer.h"

#include "../std/vector.h"

struct Vector *timer_handlers;

void init_timer()
{
    timer_handlers = vec_create(5);
}

void on_timer_int(void (*f)(int))
{
    vec_push(timer_handlers, f);
}

void handle_timer_interrupt()
{
    for (int i = 0; i < timer_handlers->length; i++)
    {
        ((void (*)())vec_get(timer_handlers, i))();
    }
}
