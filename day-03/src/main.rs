use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    // # Create line iterator over input file
    let input_file = File::open("input.txt").unwrap();
    let line_iterator = io::BufReader::new(input_file).lines();

    /*
     * - Iterate each line
     *   - Get number of chars in each line
     *   - Iterate characters
     *      - convert into priority
     *      - If from first half, read into set
     *      - Otherwise check if duplicate from set
     */
    let mut priority_sum = 0;
    'lines: for line in line_iterator {
        let line = line.unwrap();
        // WARNING: This is only correct if the string doesn't contain any multibyte characters.
        let char_count = line.len();
        let mut current_char = 0;
        let mut compartment_1_priorities = HashSet::new();
        for c in line.chars() {
            assert!(c.is_ascii(), "Non ASCII input.");
            let priority = char_to_priority(c);
            if current_char < char_count / 2 {
                compartment_1_priorities.insert(priority);
            }
            else {
                if compartment_1_priorities.contains(&priority) {
                    priority_sum += priority;
                    println!("{}", priority_sum);
                    continue 'lines;
                }
            }
            current_char += 1;
        }
    }
    println!("{}", priority_sum);
}

fn char_to_priority(c: char) -> u32 {
    let p = u32::from(c);
    if p >= u32::from('a') { p - u32::from('a') + 1 }
    else { p - u32::from('A') + 27 }
}