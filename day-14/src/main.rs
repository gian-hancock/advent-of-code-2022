use std::{collections::HashMap, iter::Peekable};

const ORIGIN: Vec2 = Vec2 { x: 500, y: 0 };

fn main() {
    let input = include_str!("../input_test.txt");

    // Parse input
    let (min_bound, max_bound, sections) = parse(input);

    // Build map
    let mut map = Map::new(min_bound, max_bound);
    for section in sections {
        for segment in section.windows(2) {
            map.add_segment(&segment[0], &segment[1]);
        }
    }

    // Simulate a single grain of sand. Returns whether the grain of sand fell of the edge of the 
    // map.
    let mut simulate_grain = ||  {
        let mut pos = ORIGIN;
        loop {
            if pos.y > map.max.y {
                return true;
            }
            let candidate_positions = [
                pos.add(&Vec2::DOWN),
                pos.add(&Vec2::DOWN_LEFT),
                pos.add(&Vec2::DOWN_RIGHT),
            ];
            let mut has_moved = false;
            for candidate_position in &candidate_positions {
                if !map.points.contains_key(&candidate_position) {
                    has_moved = true;
                    pos = candidate_position.clone();
                    break;
                }
            }
            if !has_moved {
                map.points.insert(pos, 'o');
                return false
            }
        }
    };

    let mut grains = 0;
    while !simulate_grain() {
        grains += 1;
    }  
    map.print();
    println!("grains: {}", grains);
}

fn parse(input: &str) -> (Vec2, Vec2, Vec<Vec<Vec2>>) {
    let mut min_bound = ORIGIN;
    let mut max_bound = ORIGIN;
    let mut sections: Vec<Vec<Vec2>> = Vec::new();
    for line in input.lines() {
        let mut points = Vec::new();
        let mut chars = line.chars().peekable();
        loop {
            let point = read_point(&mut chars);
            if !point.is_some() {
                break;
            }
            let point = point.unwrap();
            min_bound = min_bound.min(&point);
            max_bound = max_bound.max(&point);
            points.push(point);
        }
        sections.push(points);
    }

    (min_bound, max_bound, sections)
}

fn read_point(chars: &mut Peekable<impl Iterator<Item = char>>) -> Option<Vec2> {
    let mut read_int = || {
        // Consume all non int characters
        loop {
            match chars.peek() {
                Some(c) if c.is_ascii_digit() => break,
                Some(_) => {
                    chars.next();
                }
                None => break,
            }
        }
        let mut has_result = false;
        let mut result = 0;
        loop {
            let c = chars.next();
            match c {
                None => return if has_result { Some(result) } else { None },
                Some(c) if !c.is_ascii_digit() => {
                    return if has_result { Some(result) } else { None }
                }
                Some(c) => {
                    has_result = true;
                    result *= 10;
                    result += i32::try_from(c.to_digit(10).unwrap()).unwrap();
                }
            }
        }
    };

    if let Some(int) = read_int() {
        let p = Vec2 {
            x: int,
            y: read_int().unwrap(),
        };
        Some(p)
    } else {
        None
    }
}

#[derive(Debug, Eq, Hash, PartialEq, Clone)]
struct Vec2 {
    x: i32,
    y: i32,
}

impl Vec2 {
    const DOWN: Vec2 = Vec2 { x: 0, y: 1 };
    const DOWN_LEFT: Vec2 = Vec2 { x: -1, y: 1 };
    const DOWN_RIGHT: Vec2 = Vec2 { x: 1, y: 1 };

    fn min(&self, other: &Vec2) -> Vec2 {
        Vec2 {
            x: self.x.min(other.x),
            y: self.y.min(other.y),
        }
    }

    fn max(&self, other: &Vec2) -> Vec2 {
        Vec2 {
            x: self.x.max(other.x),
            y: self.y.max(other.y),
        }
    }

    fn add(&self, other: &Vec2) -> Vec2 {
        Vec2 {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

#[derive(Debug)]
struct Map {
    min: Vec2,
    max: Vec2,
    points: HashMap<Vec2, char>,
}

impl Map {
    pub(crate) fn new(min: Vec2, max: Vec2) -> Map {
        Map {
            min,
            max,
            points: HashMap::new(),
        }
    }

    pub(crate) fn add_segment(&mut self, a: &Vec2, b: &Vec2) {
        let min = a.min(b);
        let max = a.max(b);
        for x in min.x..=max.x {
            for y in min.y..=max.y {
                let p = Vec2 { x, y };
                self.min = self.min.min(&p);
                self.max = self.max.max(&p);
                self.points.insert(p, '#');
            }
        }
    }

    fn print(&self) {
        for y in self.min.y..=self.max.y {
            for x in self.min.x..=self.max.x {
                match self.points.get(&Vec2 { x, y }) {
                    Some(c) => print!("{}", c),
                    None => print!("."),
                }
            }
            println!();
        }
    }
}
