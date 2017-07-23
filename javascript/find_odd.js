function findOdd(A) {
    A.sort();
    var j = 1;
    for (var i = 1; i < A.length; i++) {
        if (A[i] == A[i-1]) {
            j++;
        } else {
            if (j % 2 == 1) {
                return A[i-1];
            } else {
                j = 1;
            }
        }
    }
    return A[A.length-1];
}

let result = findOdd([1,1,2,-2,5,2,4,4,-1,-2,5]);
console.log(result);
