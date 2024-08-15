fn main() {
    let mut user_input: String = String::new();
    std::io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read user input!");
    let num_1: u128 = user_input
        .trim()
        .parse()
        .expect("Failed to parse user input!");

    user_input = String::new();

    std::io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read user input!");
    let num_2: u128 = user_input
        .trim()
        .parse()
        .expect("Failed to parse user input!");
    let gcd_vec_1 = factor(num_1);
    let gcd_vec_2 = factor(num_2);

    let mut final_gcd: u128 = 0;
    for i in &gcd_vec_1 {
        if gcd_vec_2.contains(i) {
            final_gcd = *i;
        }
    }

    println!("gcd: {final_gcd}");

    let mut multiplier: u128 = 1;
    loop {
        let multiplied = num_1 * multiplier;
        if multiplied % num_1 == 0 && multiplied % num_2 == 0 {
            println!("lcm: {multiplied}");
            break;
        } else {
            multiplier += 1;
        }
    }
}

fn factor(number: u128) -> Vec<u8> {
    let mut factor_vec: Vec<u8> = Vec::new();
    for i in 1..number {
        if number % i == 0 {
            factor_vec.push(i);
        }
    }
    factor_vec.push(number);

    factor_vec
}
