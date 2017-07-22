function isOpposite(s1, s2) {
    if (s1 == "" || s2 == "" || s1.length != s2.length) {
        return false;
    }

    for (var i = 0; i < s1.length; i++) {
        if (s1[i] == s1[i].toUpperCase()) {
            if (s2[i] == s2[i].toUpperCase()) {
                return false;
            }
        } else {
            if (s2[i] == s2[i].toLowerCase()) {
                return false;
            }
        }
    }

    return true;
}

var result = isOpposite("HiThErE", "hItheRe");
console.log(result);
