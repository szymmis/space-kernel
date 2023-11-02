#include <stddef.h>
#include "memory.h"
#include "vector.h"

struct Vector
{
    size_t length;
    size_t __element_size;
    size_t __capacity;
    void **__elements;
};

struct Vector *vec_create(size_t capacity)
{
    struct Vector *v = (struct Vector *)malloc(sizeof(struct Vector));
    v->length = 0;
    v->__element_size = sizeof(void *);
    v->__elements = (void **)malloc(v->__element_size * capacity);
    v->__capacity = capacity;
    return v;
}

void vec_expand(struct Vector *v, size_t capacity)
{
    void **old_ptr = v->__elements;
    void **ptr = (void **)malloc((v->__capacity + capacity) * v->__element_size);

    for (char i = 0; i < v->length; i++)
    {
        ptr[i] = old_ptr[i];
    }

    v->__elements = ptr;
    v->__capacity = v->__capacity + capacity;
}

void *vec_get(struct Vector *v, size_t index)
{
    return v->__elements[index];
}

void vec_push(struct Vector *v, void *element)
{
    if (v->length + 1 > v->__capacity)
    {
        vec_expand(v, 5);
    }

    v->__elements[v->length] = element;
    v->length = v->length + 1;
}

void vec_remove(struct Vector *v, size_t index)
{
    if (index > v->length || index < 0)
    {
        return;
    }

    for (char i = index; i < v->length - 1; i++)
    {
        v->__elements[i] = v->__elements[(i + 1)];
    }

    v->length = v->length - 1;
}
