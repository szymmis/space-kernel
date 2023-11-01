#include "memory.h"

int heap_size;

void* malloc(int size)
{
    void* ptr = (void*)(HEAP_START + heap_size);
    // TODO: investigate why this offset is needed
    // otherwise allocating memory seems to be corrupted
    // e.g. overlaps in vectors
    heap_size += size + 0x10;
    return ptr;
}
