#include "memory.h"

int heap_size;

void* malloc(int size)
{
    void* ptr = (void*)(HEAP_START + heap_size);
    heap_size += size;
    return ptr;
}
