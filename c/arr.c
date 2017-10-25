#include <stdio.h>

int print_matrix(int x[100][100]) {
    for (int i=0; i<3; i++) {
        for (int j=0; j<3; j++) {
            printf("%d ", x[i][j]);
        }
        printf("\n");
    }
}

int main() {
    int mat[100][100];
    for (int i=0; i<3; i++) {
        for (int j=0; j<3; j++) {
            scanf("%d", &mat[i][j]);
        }
        printf("\n");
    }
    print_matrix(mat);
    return 0;
}
