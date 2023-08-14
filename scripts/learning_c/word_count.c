#include <stdio.h>
#include <stdlib.h>

typedef int bool;
typedef char byte;

void main(int argc, char *argv[])
{
    char *name = argv[1];
    printf("File: %s\n", name);

    FILE *handle = fopen(name, "rb");

    if (handle == 0)
    {
        printf("Error opening file");
        exit(-1);
    }

    fseek(handle, 0, SEEK_END);
    size_t length = ftell(handle);
    rewind(handle);

    printf("Length: %d bytes\n", length);

    byte *file = malloc(length + 1);
    fread(file, length, 1, handle);
    fclose(handle);

    file[length] = 0;

    size_t words = 1;
    bool last_was_word;
    for (size_t i; i < length; i++)
    {
        char byte = file[i];
        bool is_space = byte == ' ' || byte == '\n';
        words += is_space && !last_was_word;
        last_was_word = is_space;
    }

    free(file);

    printf("Words: %d", words);
}
