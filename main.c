#include <stdio.h>
int foo();

int main() {
    int v = foo();
    printf("func foo returns %d\n", v);
    return 0;
}
