fn prime_factors(mut n: i64) -> String {
    let mut result = String::new();
    let mut count = 1;
    let mut last_divider = 1;
    while n != 1 {
        for divider in 2..n+1 {
            if n % divider == 0 {
                n /= divider;

                if divider == last_divider {
                    count += 1;
                } else if last_divider != 1 {
                    if count == 1 {
                        result = format!("{}({})", result, last_divider);
                    } else {
                        result = format!("{}({}**{})", result, last_divider, count);
                        count = 1;
                    }
                }

                if n == 1 {
                    if count == 1 {
                        result = format!("{}({})", result, divider);
                    } else {
                        result = format!("{}({}**{})", result, divider, count);
                    }
                } else {
                    last_divider = divider;
                    break;
                }

            }
        }
    }
    result
}

fn main() {
    let result = prime_factors(86240);
	println!("{}", result);
}
