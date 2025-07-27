use core::time;
use rand::Rng;
use std::io;
use std::thread::sleep;

/// Generates a random string of length and with charset based on the given difficulty.
fn gen_random_string(mut difficulty: usize) -> String {
    let mut rng = rand::thread_rng();

    let (length, charset) = loop {
        match difficulty {
            0 => {
                break (6, "ABCDEFGHIJKLMNOPQRSTUVWXYZ".as_bytes());
            }
            1 => {
                break (
                    8,
                    "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789".as_bytes(),
                );
            }
            _ => {
                println!("You've specified invalid difficulty.");
                difficulty = get_difficulty();
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

/// Returns difficulty level from user with stdin.
fn get_difficulty() -> usize {
    loop {
        println!("Please input the difficulty:\n'0' - easy\n'1' - medium\n'2' - hard");
        let input = get_user_input().parse::<usize>();
        match input {
            Ok(input) => {
                if [0, 1, 2].contains(&input) {
                    return input;
                } else {
                    println!("Invalid input, try one of these: '0', '1', '2'!");
                    continue;
                }
            }
            Err(e) => {
                println!("ERROR: function get_difficulty() received bad input. Error: '{e}'");
                continue;
            }
        }
    }
}

fn get_user_input() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input.trim().to_lowercase().to_string()
}

fn countdown(mut seconds: u8) {
    while seconds != 0 {
        println!("{seconds}...");
        sleep(time::Duration::from_secs(1));
        seconds -= 1;
    }
}

fn main() {
    println!("Welcome to the Short-term memory trainer!\n");
    let mut score: u128 = 0;
    let mut difficulty = get_difficulty();

    'outer: loop {
        println!("'0' - Start\n'1' - Change difficulty\n'Q' - Quit");
        let input = get_user_input();

        match input.as_str() {
            "0" => loop {
                let s = gen_random_string(difficulty);
                println!(
                    "Score: {}\n\n{}\n\nType 'Quit' to exit the application.",
                    score, &s
                );
                countdown(3);
                let i = get_user_input();
                if s.to_lowercase() == i {
                    println!("Correct!");
                    score = score.wrapping_add(1);
                    continue;
                } else if i == "quit" {
                    break 'outer;
                } else {
                    println!("Incorrect!");
                    continue;
                }
            },
            "1" => {
                difficulty = get_difficulty();
            }
            "q" => {
                break 'outer;
            }
            _ => {
                println!("Invalid input");
            }
        }
    }
}
