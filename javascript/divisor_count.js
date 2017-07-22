function getDivisorsCnt(n) {
    var count = 0
    for (var i = n; i>0; i--) {
        if (n%i == 0) {
            count++;
        }
    }
    return count;
}

var result = getDivisorsCnt(10);
console.log(result);
