#include <stdio.h>

int swap(int *m, int *n) {
    *m = *m + *n;
    *n = *m - *n;
    *m = *m - *n;
    return 0;
}

int main() {
    int x = 10;
    int y = 15;
    swap(&x, &y);
    printf("%d %d", x, y);
    return 0;
}
