use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    // # Create line iterator over input file
    let input_file = File::open("input.txt").unwrap();
    let line_iterator = io::BufReader::new(input_file).lines();

    // Iterate lines and calculate max
    let mut sum: u64 = 0;
    let mut max: u64 = 0;
    for line in line_iterator {
        let line = line.unwrap();
        if line == "" {
            println!("Current: {}; Max: {}", sum, max);
            max = max.max(sum);
            sum = 0;
        }
        else {
            sum += line.parse::<u64>().unwrap();
        }
        
    }
    println!("Max: {}", max);
}
