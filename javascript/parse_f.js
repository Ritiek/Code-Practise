function parseF(s) {
  let result = parseFloat(s);
  if (isNaN(result)) {
    return null;
  } else {
    return result;
  }
}
