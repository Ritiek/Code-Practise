#include <math.h>
#include <stdio.h>
#include <string.h>
#include <stdlib.h>
#include <assert.h>
#include <limits.h>
#include <stdbool.h>

int main() {
    int *arr = malloc(sizeof(int) * 5);
    for(int arr_i = 0; arr_i < 5; arr_i++){
       scanf("%d",&arr[arr_i]);
    }

    long int sum = arr[0];
    long int smallest = arr[0];
    long int largest = arr[0];

    for (int i = 1; i < 5; i++) {
        if (arr[i] < smallest) {
            smallest = arr[i];
        }

        if (arr[i] > largest) {
            largest = arr[i];
        }

        sum += arr[i];
    }


    long int sum_smallest = sum - largest;
    long int sum_largest = sum - smallest;


    printf("%ld %ld", sum_smallest, sum_largest);
    return 0;
}
