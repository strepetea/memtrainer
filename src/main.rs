use std::io;
use std::usize;

use rand::Rng;
/// Returns a random string of size length.
fn gen_random_string(mut difficulty: usize) -> String {
    let mut rng = rand::thread_rng();
    //const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";

    let (length, charset) = loop {
        match difficulty {
            0 => {
                break (6, "ABCDEFGHIJKLMNOPQRSTUVWXYZ".as_bytes());
            }
            1 => {
                break (
                    10,
                    "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789".as_bytes(),
                );
            }
            _ => {
                println!("You've specified invalid difficulty.");
                difficulty = set_difficulty();
            }
        }
    };

    (0..length)
        .map(|_| {
            let idx = rng.gen_range(0..charset.len());
            charset[idx] as char
        })
        .collect()
}

fn set_difficulty() -> usize {
    loop {
        let mut input: String = String::with_capacity(1);
        println!("Please input the difficulty:\n'0' - easy\n'1' - medium\n'2' - hard");
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        match input.trim().parse::<usize>() {
            Ok(num) => {
                if [0, 1, 2].contains(&num) {
                    return num;
                } else {
                    println!("Invalid input, try one of these: '0', '1', '2'!");
                    continue;
                }
            }
            Err(_) => {
                println!("Invalid input, try one of these: '0', '1', '2'!");
                continue;
            }
        };
    }
}

fn main() {
    println!("Welcome to the Short-term memory trainer!\n");
    let mstring: String;
    let difficulty = set_difficulty();

    mstring = gen_random_string(difficulty);

    println!("Here is your string:");

    println!("{mstring}");
}
