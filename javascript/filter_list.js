function filter_list(l) {
    var nl = [];
    for (var i = 0; i < l.length; i++) {
        if (typeof l[i] == "number") {
            nl.push(l[i]);
        }
    }
    return nl;
}

let result = filter_list([1,'a','b',0,15]);
console.log(result);
