#define VGA_BUFFER ((char* )0xb8000)

void print(char * str);

void main()
{
   print("Hello world!");
}

void print(char *str)
{
    short index = 0;
    while (str[index] != '\0')
    {
        VGA_BUFFER[index * 2] = str[index];
        VGA_BUFFER[index * 2 + 1] = 0xF;
        index++;
    }
}
