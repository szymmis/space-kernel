#include "display/draw.c"
#include "display/print.c"
#include "std/memory.c"
#include "std/vector.c"
#include "system/interrupts.c"
#include "system/keyboard.c"

struct Message
{
    char *msg;
};

void key_down(int keycode)
{
    cls();
    if (keycode == ARROW_LEFT)
    {
        print("left arrow down");
    }
    else if (keycode == SPACEBAR)
    {
        print("spacebar down");
    }
    else
    {
        print("Key down");
    }
}

void key_up(int keycode)
{
    cls();
    if (keycode == ARROW_LEFT)
    {
        print("left arrow up");
    }
    else if (keycode == SPACEBAR)
    {
        print("spacebar up");
    }
    else
    {
        print("Key up");
    }
}

void main()
{
    init_keyboard();

    print("Hello world");
    print("abcdefghijklmnoprstuwxyz");
    print("ABCDEFGHIJKLMNOPRSTUWXYZ");

    print("1234567890");
    printi(12345);
    printi(67890);

    struct Vector *v = vec_create(sizeof(struct Message *), 2);
    struct Vector *v2 = vec_create(sizeof(struct Message *), 2);

    struct Message *m1 = malloc(sizeof(struct Message));
    m1->msg = "Message 1";
    struct Message *m2 = malloc(sizeof(struct Message));
    m2->msg = "Message 2";
    struct Message *m3 = malloc(sizeof(struct Message));
    m3->msg = "Message 3";

    vec_push(v, m1);
    vec_push(v, m1);
    vec_push(v, m2);
    vec_push(v, m3);
    vec_push(v, m1);
    vec_push(v, m2);
    vec_push(v, m3);

    vec_remove(v, 2);

    vec_push(v2, m2);
    vec_push(v2, m1);
    vec_push(v2, m2);
    vec_push(v2, m3);
    vec_push(v2, m2);
    vec_push(v2, m1);
    vec_push(v2, m1);
    vec_push(v2, m3);

    for (char i = 0; i < v->length; i++)
    {
        print(((struct Message *)vec_get(v, i))->msg);
    }

    print("");

    for (char i = 0; i < v2->length; i++)
    {
        print(((struct Message *)vec_get(v2, i))->msg);
    }

    on_key_down(&key_down);
    on_key_up(&key_up);
}
