// count_string.c

unsigned int mystrlen(char *str) {
    //printf("String Received by C is %c \n", *str);
    unsigned int c; 
    for (c=0; *str != '\0'; c++, *str++); 
    return c; 
} 