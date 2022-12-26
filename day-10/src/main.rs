use std::io;
use std::fs;
use std::io::BufRead;

const INSTR_LATENCY: i32 = 2;
const SCREEN_WIDTH: i32 = 40;

fn main() {
    let instrs = get_instrs();

    let mut current_column = 0;
    let mut x = 1;
    for (cycles, increment) in instrs {
        for _ in 0..cycles {
            if (x - current_column as i32).abs() > 1 {
                print!(".");
            } else {
                print!("#");
            }
            current_column += 1;
            if current_column >= SCREEN_WIDTH {
                current_column = 0;
                println!();
            }
        }
        x += increment;
    }
}

fn get_instrs() -> impl Iterator<Item = (i32, i32)> {
    io::BufReader::new(fs::File::open("input.txt").unwrap())
        .lines()
        .map(|l| parse_line(l.unwrap()))
}

fn parse_line(line: String) -> (i32, i32) {
    let mut tokens = line.split(' ');
    match tokens.next() {
        Some("addx") => (INSTR_LATENCY, tokens.next().unwrap().parse().unwrap()),
        Some("noop") => (1, 0),
        _ => panic!()
    }
}
