#include "draw.c"
#include "interrupts.c"
#include "keyboard.c"
#include "memory.c"
#include "print.c"
#include "vector.c"

struct Message
{
    char *msg;
};

void main()
{
    print("Hello world", 10, 10);
    print("abcdefghijklmnoprstuwxyz", 10, 20);
    print("ABCDEFGHIJKLMNOPRSTUWXYZ", 10, 30);

    print("1234567890", 10, 40);
    printi(12345, 10, 50);
    printi(67890, 35, 50);

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
        print(((struct Message *)vec_get(v, i))->msg, 180, 10 + 10 * i);
    }
}
