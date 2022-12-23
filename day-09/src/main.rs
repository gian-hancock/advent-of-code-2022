use std::collections::HashSet;
use std::fs;
use std::io;
use std::io::BufRead;

fn main() {
    // # Parse file into series of Movements
    let movements = io::BufReader::new(fs::File::open("input.txt").unwrap())
        .lines()
        .map(|l| parse_line(l.unwrap()));

    let mut pos_head = Position {x: 0, y: 0};
    let mut pos_tail = Position {x: 0, y: 0};
    let mut tail_positions: HashSet<Position> = HashSet::new();
    for movement in movements {
        println!("{:?}", movement);
        for count in 0..movement.count {
            pos_head.x += movement.dx;
            pos_head.y += movement.dy;
            if (pos_head.x - pos_tail.x).abs() > 1 {
                pos_tail.x = (pos_head.x + pos_tail.x) / 2;
                pos_tail.y = pos_head.y;
            } else if (pos_head.y - pos_tail.y).abs() > 1 {
                pos_tail.y = (pos_head.y + pos_tail.y) / 2;
                pos_tail.x = pos_head.x;
            }
            println!("  H: {:?}, T: {:?}", pos_head, pos_tail);
            tail_positions.insert(pos_tail);
        }
    }
    println!("Tail occupied {} unique positions", tail_positions.len());
}

fn parse_line(line: String) -> Movement {
    let mut tokens = line.split(' ');
    let direction = match tokens.next() {
        Some("U") => (0, 1),
        Some("D") => (0, -1),
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