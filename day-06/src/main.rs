use std::fs::File;
use std::io::{self};
use utf8_chars::BufReadCharsExt;

const HISTORY_LEN: usize = 14;

fn main() {
    let input_file = File::open("input.txt").unwrap();
    let mut history = ['\0'; HISTORY_LEN];
    for (i, c) in io::BufReader::new(input_file).chars().enumerate() {
        let c = c.unwrap();
        history[i % HISTORY_LEN] = c;
        println!("{:?}", history);

        // # Check if end condition met
        let mut end = i>= HISTORY_LEN;
        for i in 0..HISTORY_LEN {
            for j in 0..HISTORY_LEN {
                if i != j && history[i] == history[j] {
                    end = false;
                }
            }
        }

        if end {
            println!("ended at {}", i + 1);
            break;
        }
    }
}
