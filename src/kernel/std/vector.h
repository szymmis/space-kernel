#ifndef VECTOR_H
#define VECTOR_H

#include <stddef.h>

struct Vector
{
    size_t length;
    size_t __element_size;
    size_t __capacity;
    void **__elements;
};

struct Vector *vec_create(size_t capacity);
void vec_expand(struct Vector *v, size_t capacity);
void *vec_get(struct Vector *v, size_t index);
void vec_push(struct Vector *v, void *element);
void vec_remove(struct Vector *v, size_t index);

#endif
