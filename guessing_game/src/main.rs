fn random_int() -> u64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    let mut seed = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
    seed ^= seed << 21;
    seed ^= seed >> 35;
    seed ^= seed << 4;
    seed
}

fn main() {
    println!("Hello, it's guessing game! Try to guess number from 1 to 100...");
    let secret_number = (random_int() % 100 + 1) as u8;
    loop {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let input: u8 = input.trim().parse().unwrap();
        if input == secret_number {
            println!("Congratulations! You guessed a number!");
            break;
        } else if input < secret_number {
            println!("Secret number is bigger");
        } else {
            println!("Secret number is lower");
        }
    }
}
