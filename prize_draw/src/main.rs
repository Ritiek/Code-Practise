fn rank(st: &str, we: Vec<i32>, n: usize) -> &str {
    if st == "" {
        return "No participants";
    } else if n > we.len() {
        return "Not enough participants";
    }

    let alphabets = "abcdefghijklmnopqrstuvwxyz";
    let lower_st = st.to_lowercase();
    let names = lower_st.split(',').collect::<Vec<&str>>();

    let mut ranks: Vec<usize> = Vec::new();
    for (n, name) in names.iter().enumerate() {
        let mut sum = name.len();
        for letter in name.chars() {
            for (i, alphabet) in alphabets.chars().enumerate() {
                if letter == alphabet {
                    sum += i+1
                }
            }
        }
        println!("pushing in {}*{}", sum, we[n]);
        ranks.push(sum*(we[n] as usize));
    }

    let mut ranks_sorted = ranks.clone();
    ranks_sorted.sort();
    ranks_sorted.reverse();
    let score = ranks_sorted[n-1];
    let index = ranks.iter().position(|&r| r == score).unwrap();
    let split_st = st.split(',').collect::<Vec<&str>>();

    split_st[index]
}

// abcdefghij k l m n o p q r s t u v w x y z
// 1234567891011121314151617181920212223242526

//5+12+9+1+2+5+20+8 * 5 (elizabeth)
//13+1+20+20+8+5+23 * 5 (matthew)
//1+2+9+7+1+9+12 * 4 (abigail)

fn main() {
    //let st = "Addison,Jayden,Sofia,Michael,Andrew,Lily,Benjamin";
    //let we = vec![4, 2, 1, 4, 3, 1, 2];
    //let n = 4;
    let st = "jah,Chloe,Elizabeth,Matthew,Natalie,Jayden";
    let we = vec![1, 3, 5, 5, 3, 6];
    let n = 2;

    let result = rank(st, we, n);
    println!("{}", result);
}
