use std::collections::HashSet;
use std::fs;
use std::io;
use std::io::BufRead;

const KNOT_COUNT: usize = 10;
fn main() {
    // # Parse file into series of Movements
    let movements = io::BufReader::new(fs::File::open("input.txt").unwrap())
        .lines()
        .map(|l| parse_line(l.unwrap()));

    let mut knot_positions = [Position { x: 0, y: 0 }; KNOT_COUNT];
    let mut tail_positions: HashSet<Position> = HashSet::new();
    for movement in movements {
        for _ in 0..movement.count {
            knot_positions[0].x += movement.dx;
            knot_positions[0].y += movement.dy;
            for knot_idx in 1..knot_positions.len() {
                let cur = knot_positions[knot_idx];
                let prev = knot_positions[knot_idx - 1];
                let dx = prev.x - cur.x;
                let dy = prev.y - cur.y;
                if dx.abs() > 1 || dy.abs() > 1 {
                    knot_positions[knot_idx].x += dx.signum();
                    knot_positions[knot_idx].y += dy.signum();
                }
            }
            tail_positions.insert(*knot_positions.last().unwrap());
        }
    }
    println!("Tail occupied {} unique positions", tail_positions.len());
}

fn parse_line(line: String) -> Movement {
    let mut tokens = line.split(' ');
    let direction = match tokens.next() {
        Some("U") => (0, -1),
        Some("D") => (0, 1),
        Some("L") => (-1, 0),
        Some("R") => (1, 0),
        _ => panic!(),
    };
    let count: u32 = tokens.next().unwrap().parse().unwrap();
    Movement {
        dx: direction.0,
        dy: direction.1,
        count,
    }
}

#[derive(Debug)]
struct Movement {
    dx: i32,
    dy: i32,
    count: u32,
}

#[derive(Debug, Eq, Hash, PartialEq, Clone, Copy)]
struct Position {
    x: i32,
    y: i32,
}
