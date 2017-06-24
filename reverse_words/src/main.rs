fn reverse_words(str:&str) -> String {
    let mut array = str.split_whitespace()
                       .collect::<Vec<&str>>();
    array.reverse();
    array.join(" ")
}

fn main() {
    let result = reverse_words("one two three");
    println!("{}", result);
}
