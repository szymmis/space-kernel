#include <stddef.h>
#include "memory.h"
#include "vector.h"

struct Vector
{
    size_t length;
    void **__elements;
    size_t __element_size;
    size_t __capacity;
};

struct Vector *vec_create(size_t element_size, size_t capacity)
{
    struct Vector *v = (struct Vector *)malloc(sizeof(struct Vector *));
    v->length = 0;
    v->__element_size = element_size;
    vec_expand(v, capacity);
    return v;
}

void vec_expand(struct Vector *v, size_t capacity)
{
    void **old_ptr = v->__elements;
    void **ptr = malloc((v->__capacity + capacity) * v->__element_size);
    v->__capacity = v->__capacity + capacity;

    for (char i = 0; i < v->length; i++)
    {
        ptr[i * v->__element_size] = old_ptr[i * v->__element_size];
    }
}

void *vec_get(struct Vector *v, size_t index)
{
    return v->__elements[index * v->__element_size];
}

void vec_push(struct Vector *v, void *element)
{
    if (v->length + 1 > v->__capacity)
    {
        vec_expand(v, 5);
    }

    v->__elements[v->__element_size * v->length] = element;
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
        v->__elements[v->__element_size * i] = v->__elements[v->__element_size * (i + 1)];
    }

    v->length = v->length - 1;
}
