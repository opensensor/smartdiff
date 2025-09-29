#include <stdio.h>
#include <stdlib.h>

int add(int a, int b) {
    return a + b;
}

void print_hello() {
    printf("Hello, World!\n");
}

int main() {
    int result = add(5, 3);
    print_hello();
    return 0;
}
