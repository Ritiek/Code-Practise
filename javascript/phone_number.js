function createPhoneNumber(numbers) {
    var numbers = numbers.join('')
    var initial = '(' + numbers.slice(0, 3) + ') ';
    var middle = numbers.slice(3, 6) + '-';
    var last = numbers.slice(6);
    return initial + middle + last;
}

var result = createPhoneNumber([1, 2, 3, 4, 5, 6, 7, 8, 9, 0]);
console.log(result);
