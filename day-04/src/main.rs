use std::io::{BufReader, BufRead};
use std::fs::File;
use std::iter::Peekable;

fn main() {
    let input_reader = BufReader::new(File::open("input.txt").unwrap());
    
    let mut contained_range_count = 0;
    let mut overlapping_range_count = 0;
    for line in input_reader.lines() {
        let line = line.unwrap();
        let (mut range_1, mut range_2)  = parse_line(&line);

        // Order ranges so that range_1 is the leftmost lower bound
        if range_2.0 <= range_1.0 {
            (range_1, range_2) = (range_2, range_1);
        }

        // Determine of one range contains the other
        if range_1.0 == range_2.0 || range_2.1 <= range_1.1 {
            contained_range_count += 1;
        }

        // Determine if the ranges overlap
        if range_1.1 >= range_2.0 {
            overlapping_range_count += 1;
        }
    }

    println!("Part 1: {}", contained_range_count);
    println!("Part 2: {}", overlapping_range_count);
}

fn parse_line(line: &String) -> ((i32, i32), (i32, i32)) { 
    let mut chars = line.chars().peekable();

    let range_1_lower = next_int(&mut chars);
    assert!(chars.next().unwrap() == '-');
    let range_1_upper = next_int(&mut chars);

    assert!(chars.next().unwrap() == ',');

    let range_2_lower = next_int(&mut chars);
    assert!(chars.next().unwrap() == '-');
    let range_2_upper = next_int(&mut chars);
    ((range_1_lower, range_1_upper,), (range_2_lower, range_2_upper,),)
}

fn next_value<T: Iterator<Item = char>>(chars: &mut Peekable<T>) -> i32 {
    let mut result = String::new();
    while matches!(chars.peek(), Some(c) if c.is_ascii_digit()) {
        result.push(chars.next().unwrap());
    }
    result.parse().unwrap()
}