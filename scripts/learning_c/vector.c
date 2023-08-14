#include <stdio.h>
#include <stdlib.h>
#include <assert.h>
#include <string.h>

typedef struct Vector
{
    size_t length;
    size_t capacity;
    size_t element_length;
    void *data;
} Vector;

void vector_init(Vector *vector, size_t element_length)
{
    vector->element_length = element_length;
    vector->capacity = 20;
    vector->length = 0;
    vector->data = malloc(vector->capacity * element_length);
}

void vector_push(Vector *vector, void *data)
{
    assert(vector != NULL && data != NULL);

    if (vector->length == vector->capacity)
    {
        size_t size = vector->length * 2;
        vector->capacity = size;
        vector->data = malloc(size);
    }

    void *offset = vector->data + (vector->length * vector->element_length);
    memcpy(offset, data, vector->element_length);
    vector->length++;
}

void *vector_get(Vector *vector, size_t index)
{
    assert(vector != NULL && index < vector->length);

    void *offset = vector->data + (index * vector->element_length);
    return offset;
}

int main()
{
    Vector vector;
    vector_init(&vector, sizeof(char *));

    for (int i = 0; i < 10; i++)
    {
        char *str = malloc(16);
        sprintf(str, "Hello World #%d", i);
        vector_push(&vector, &str);
    }

    for (int i = 0; i < vector.length; i++)
    {
        char **val = vector_get(&vector, i);
        printf("%s\n", *val);
    }

    return 0;
}