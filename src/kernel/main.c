#include "interrupts.c"
#include "draw.c"
#include "print.c"
#include "memory.c"

struct Message {
    char* str;
};

void main()
{
    struct Message* msg = malloc(sizeof(struct Message));
    msg->str = "Hello from struct";

    print("Hello world", 10, 10);
    print("abcdefghijklmnoprstuwxyz", 10, 20);
    print("ABCDEFGHIJKLMNOPRSTUWXYZ", 10, 30);
    print("1234567890", 10, 40);
    
    printi(12345, 10, 50);
    printi(67890, 35, 50);

    print(msg->str, 10, 60);
}
