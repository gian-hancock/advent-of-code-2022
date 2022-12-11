use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    // # Create line iterator over input file
    let input_file = File::open("input.txt").unwrap();
    let line_iterator = io::BufReader::new(input_file).lines();

    // Iterate lines and calculate max
    let mut sums = vec![0];
    for line in line_iterator {
        let curr = sums.len() - 1;
        let line = line.unwrap();
        if line == "" {
            println!("Elf {} calories: {}", curr, sums[curr]);
            sums.push(0);
        }
        else {
            sums[curr] += line.parse::<u64>().unwrap();
        }
        
    }

    sums.sort();
    println!("Total calories of top 3 elves: {:?}", sums.into_iter().rev().take(3).sum::<u64>());
}