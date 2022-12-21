use std::fs::File;
use std::io::{self, BufRead};
use std::iter::Peekable;

fn main() {
    // part_1();
    part_2();
}

fn part_1() {
    let input_file = File::open("input.txt").unwrap();
    let mut line_iterator = io::BufReader::new(input_file).lines();

    let mut stacks = parse_stacks(&mut line_iterator);
    println!("Stacks initial state: {:?}", stacks);

    // # Consume blank line which separates initial stack state from crane instructions
    line_iterator.next();

    // # Manipulate stacks from file input
    let stack_op_iter = line_iterator.map(|line| parse_stack_op(line.unwrap()));
    for stack_op in stack_op_iter {
        for i in 0..stack_op.count {
            let t = stacks[stack_op.from].pop().unwrap();
            stacks[stack_op.to].push(t);
        }
    }
    println!("{:?}", stacks);
}

fn part_2() {
    let input_file = File::open("input.txt").unwrap();
    let mut line_iterator = io::BufReader::new(input_file).lines();

    let mut stacks = parse_stacks(&mut line_iterator);
    println!("Stacks initial state: {:?}", stacks);

    // # Consume blank line which separates initial stack state from crane instructions
    line_iterator.next();

    // # Manipulate stacks from file input
    let stack_op_iter = line_iterator.map(|line| parse_stack_op(line.unwrap()));
    for stack_op in stack_op_iter {
        println!("{:?}", stacks);
        let new_len = stacks[stack_op.from].len() - stack_op.count;
        let stack_to_move: Vec<char> = stacks[stack_op.from].drain(new_len..).collect();
        stacks[stack_op.to].extend(stack_to_move);
    }
    println!("{:?}", stacks);
}

fn parse_stacks<T: Iterator<Item = Result<String, std::io::Error>>>(lines: &mut T) -> Vec<Vec<char>> {
    let mut stacks = vec![];
    'outer: loop {
        let line = lines.next().unwrap().unwrap();
        // Iterate create IDs in line. If a digit is encountered, we have reached the end of the crate IDs
        for (i, c) in line.chars().skip(1).step_by(4).enumerate() {
            if c.is_ascii_digit() { break 'outer; }
            if c == ' ' { continue; }
            while i >= stacks.len() { stacks.push(Vec::new()); }
            stacks[i].push(c);
        }
    }
    for stack in &mut stacks {
        stack.reverse();
    }
    stacks
}

/// Parses a stack operation of the form "move 2 from 8 to 4"
fn parse_stack_op(line: String) -> StackOperation {
    let mut scanner = Scanner::new(line.chars());
    scanner.next_word();
    let count = scanner.next_int();
    scanner.next_word();
    let from = scanner.next_int();
    scanner.next_word();
    let to = scanner.next_int();
    let result = StackOperation { 
        count: count as usize, 
        from: (from - 1) as usize,
        to: (to - 1) as usize, 
    };
    result
}

#[derive(Debug)]
struct StackOperation {
    count: usize,
    from: usize,
    to: usize,
}

struct Scanner<T: Iterator<Item = char>> {
    chars: Peekable<T>,
}

impl<T: Iterator<Item = char>> Scanner<T> {
    fn new(chars: T) -> Scanner<T> where T: Iterator<Item = char> {
        Scanner { chars: chars.peekable() }
    }

    fn next_word(&mut self) -> () {
        loop {
            let c = self.chars.next();
            match c {
                None => break,
                Some(c) => {
                    if c.is_ascii_whitespace() { break }
                },
            }
        }
    }

    fn next_int(&mut self) -> i32 {
        let mut value = String::new();
        loop {
            let c = self.chars.next();
            match c {
                None => break,
                Some(c) => {
                    if c.is_ascii_digit() {
                        value.push(c)
                    }
                    else {
                        break;
                    }
                },
            }
        }
        value.parse().unwrap()

    }
}
