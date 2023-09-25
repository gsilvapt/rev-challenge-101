use std::io::{stdin, stdout, Write};

fn main() {
    let part1: String = String::from("ZmxhZ3tyM3Yzc");

    loop {
        let mut input = String::new();
        println!("Please provide password for flag");
        let _ = stdout().flush();
        stdin().read_line(&mut input).expect("Please provide a valid string");

        if let Some('\n') = input.chars().next_back() {
            input.pop();
        }

        if let Some('\r') = input.chars().next_back() {
            input.pop();
        }

        if input.len() == 10 && input.chars().nth(0).unwrap() == '0' && input.chars().nth(5).unwrap().to_digit(10).unwrap() % 4 == 0 {
            println!("Succeess! Here's your key:");
            let part2: String = String::from("jNzIW5nX2whazNfNF9iMHNzfQ");
            println!("{}{}", part1, part2);
            break;
        } 

        println!("Input did not match secret - try again");
    }
}
