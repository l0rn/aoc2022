use std::{
    io::{self},
    process::exit, collections::VecDeque,
};

const MESSAGE_LENGTH: usize = 14;

fn main() {
    for line in io::stdin().lines() {
        let mut buffer = VecDeque::with_capacity(MESSAGE_LENGTH);
        for (i, char) in line.unwrap().chars().enumerate() {
            if buffer.len() < MESSAGE_LENGTH {
                // we need at least 4 chars
                buffer.push_back(char);
                continue;
            } else {
                buffer.pop_front();
                buffer.push_back(char);
            }
            let has_repetition = buffer
                .iter()
                .any(|c| buffer.iter().filter(|i| *i == c).count() > 1);

            if !has_repetition {
                println!("Start of packet marker detected at: {}", i + 1);
                exit(0);
            }
        }
    }
}
