/*
let A = X = ROCK     = 0
let B = Y = PAPER    = 1
let C = Z = SCISSORS = 2
let DRAW = 0
let WIN  = 1
let LOSE = 2

part 1:
    All possible outcomes:
    A A = 0-0 = +0 = DRAW
    A B = 1-0 = +1 = WIN
    A C = 2-0 = +2 = LOSE
    B A = 0-1 = -1 = LOSE
    B B = 1-1 = +0 = DRAW
    B C = 2-1 = +1 = WIN
    C A = 0-2 = -2 = WIN
    C B = 1-2 = -1 = LOSE
    C C = 2-2 = +0 = DRAW

Mapping WIN/DRAW/LOSE to {-2, -1, 0, 1, 2}
    -2 rem 3 = 1 = WIN
    -1 rem 3 = 2 = LOSE
    +0 rem 3 = 0 = DRAW
    +1 rem 3 = 1 = WIN
    +2 rem 3 = 2 = LOSE

part 2:
    A X = ROCK(0), play SCISSORS(2) to LOSE(0) = 0 + LOSE - 1
    A Y = ROCK(0), play ROCK    (0) to DRAW(1) = 0 + DRAW - 1
    A Z = ROCK(0), play PAPER   (1) to WIN (2) = 0 + WIN  - 1
etc...

*/

use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    // part1();
    part2();
}

fn part1() {
    // # Create line iterator over input file
    let input_file = File::open("input.txt").unwrap();
    let line_iterator = io::BufReader::new(input_file).lines();

    // Iterate lines and calculate score
    let mut score = 0;
    for line in line_iterator {
        let line = line.unwrap();
        let mut char_iter = line.chars();

        // # Read choices
        // Read char representing the opponents choice
        let opponent_choice = char_iter.next().unwrap();
        // Read whitespace char which only serves as a separator
        assert_eq!(char_iter.next(), Some(' '));        
        // Read char representing "my" choice
        let my_choice = char_iter.next().unwrap();

        // # Normalise representation of choices
        let opponent_choice = (u32::from(opponent_choice) - u32::from('A')) as i32;
        let my_choice = (u32::from(my_choice) - u32::from('X')) as i32;

        // # Calculate score
        let diff =my_choice - opponent_choice;
        let outcome = diff.rem_euclid(3);
        score += match outcome {
            0 => 3,
            1 => 6,
            2 => 0,
            _ => panic!(),
        };
        score += my_choice + 1;
        println!("{} -> {}", line, score);
    }
}

fn part2() {
    // # Create line iterator over input file
    let input_file = File::open("input.txt").unwrap();
    let line_iterator = io::BufReader::new(input_file).lines();

    // Iterate lines and calculate score
    let mut score = 0;
    for line in line_iterator {
        let line = line.unwrap();
        let mut char_iter = line.chars();

        // # Read choices
        // Read char representing the opponents choice
        let opponent_choice = char_iter.next().unwrap();
        // Read whitespace char which only serves as a separator
        assert_eq!(char_iter.next(), Some(' '));        
        // Read char representing outcome
        let outcome = char_iter.next().unwrap();

        // # Normalise representation of choices
        let opponent_choice = (u32::from(opponent_choice) - u32::from('A')) as i32;
        let outcome = (u32::from(outcome) - u32::from('X')) as i32;

        // # Calculate "my choice"
        
        let my_choice = (opponent_choice + outcome - 1).rem_euclid(3);

        // # Calculate score
        let diff =my_choice - opponent_choice;
        let outcome = diff.rem_euclid(3);
        score += match outcome {
            0 => 3,
            1 => 6,
            2 => 0,
            _ => panic!(),
        };
        score += my_choice + 1;
        println!("{} -> {}", line, score);
    }
}