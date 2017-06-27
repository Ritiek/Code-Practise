fn backwards_prime(start: u64, stop: u64) -> Vec<u64> {
    let mut combs = Vec::new();

    for num in start..stop+1 {
        if is_prime(num) {
            let rev_num = num.to_string()
                             .chars()
                             .rev()
                             .collect::<String>()
                             .parse::<u64>()
                             .unwrap();

            if is_prime(rev_num) && rev_num != num {
                combs.push(num);
                if rev_num >= start && rev_num <= stop {
                    combs.push(rev_num);
                }
            }

        }
    }

    combs.sort();
    combs.dedup();
    combs
}

fn is_prime(num: u64) -> bool {
    for x in 2..num {
        if num % x == 0 {
            return false;
        }
    }
    true
}

fn main() {
    let result = backwards_prime(9900, 10000);
    println!("{:?}", result);
}
