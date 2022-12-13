use std::collections::{HashSet, HashMap};
use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    part_2();
}

fn part_1() {
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

fn part_2() {
    // # Create line iterator over input file
    let input_file = File::open("input.txt").unwrap();
    let mut line_iterator = io::BufReader::new(input_file).lines();

    let mut priority_sum = 0;
    'outer: loop {
        let mut char_counts = HashMap::new();
        for _ in 0..3 {
            let mut line_chars = HashSet::new();
            let line = match line_iterator.next() {
                Some(line) => line.unwrap(),
                None => break 'outer
            };
            for c in line.chars() {
                if !line_chars.contains(&c) {
                    char_counts.entry(c).and_modify(|c| *c += 1).or_insert(1);
                }
                line_chars.insert(c);
            }
        }
        let mut common_chars = char_counts.iter().filter(|(_k, v)| **v == 3);
        let (common_char, _) = common_chars.next().unwrap();
        assert!(common_chars.count() == 0);
        priority_sum += char_to_priority(*common_char)
    }
    println!("{}", priority_sum);
}

fn char_to_priority(c: char) -> u32 {
    let p = u32::from(c);
    if p >= u32::from('a') { p - u32::from('a') + 1 }
    else { p - u32::from('A') + 27 }
}