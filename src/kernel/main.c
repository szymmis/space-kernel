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

void main()
{
    print("Hello world");
    print("abcdefghijklmnoprstuwxyz");
    print("ABCDEFGHIJKLMNOPRSTUWXYZ");

    print("1234567890");
    printi(12345);
    printi(67890);

    struct Vector *v = vec_create(sizeof(struct Message), 3);

    struct Message *m1 = malloc(sizeof(struct Message));
    m1->msg = "Message 1";
    struct Message *m2 = malloc(sizeof(struct Message));
    m2->msg = "Message 2";
    struct Message *m3 = malloc(sizeof(struct Message));
    m3->msg = "Message 3";

    vec_push(v, m1);
    vec_push(v, m2);
    vec_push(v, m3);
    vec_push(v, m1);
    vec_push(v, m2);
    vec_push(v, m3);

    vec_remove(v, 4);
    vec_remove(v, 4);

    for (char i = 0; i < v->length; i++)
    {
        print(((struct Message *)vec_get(v, i))->msg);
    }
}
