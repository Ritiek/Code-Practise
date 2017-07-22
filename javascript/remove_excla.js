function remove(s){
    return s.replace(/!/g, '') + '!';
}

let result = remove("Hi!!!");
console.log(result);
