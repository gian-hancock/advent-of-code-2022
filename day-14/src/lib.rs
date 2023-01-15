use std::{collections::HashMap, iter::Peekable};

pub const ORIGIN: Vec2 = Vec2 { x: 500, y: 0 };

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
pub struct Vec2 {
    pub x: i32,
    pub y: i32,
}

impl Vec2 {
    pub const DOWN: Vec2 = Vec2 { x: 0, y: 1 };
    pub const DOWN_LEFT: Vec2 = Vec2 { x: -1, y: 1 };
    pub const DOWN_RIGHT: Vec2 = Vec2 { x: 1, y: 1 };

    pub fn min(&self, other: &Vec2) -> Vec2 {
        Vec2 {
            x: self.x.min(other.x),
            y: self.y.min(other.y),
        }
    }

    pub fn max(&self, other: &Vec2) -> Vec2 {
        Vec2 {
            x: self.x.max(other.x),
            y: self.y.max(other.y),
        }
    }

    pub fn add(&self, other: &Vec2) -> Vec2 {
        Vec2 {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

#[derive(Debug)]
pub struct Map {
    pub min: Vec2,
    pub max: Vec2,
    pub points: HashMap<Vec2, char>,
}

impl Map {
    pub fn new(min: Vec2, max: Vec2) -> Map {
        Map {
            min,
            max,
            points: HashMap::new(),
        }
    }

    pub fn parse(input: &str) -> Map {
        let mut min_bound = ORIGIN;
        let mut max_bound = ORIGIN;
        let mut sections: Vec<Vec<Vec2>> = Vec::new();
        for line in input.lines() {
            let mut points = Vec::new();
            let mut chars = line.chars().peekable();
            loop {
                let point = read_point(&mut chars);
                if point.is_none() {
                    break;
                }
                let point = point.unwrap();
                min_bound = min_bound.min(&point);
                max_bound = max_bound.max(&point);
                points.push(point);
            }
            sections.push(points);
        }

        let mut map = Map::new(min_bound, max_bound);
        for section in sections {
            for segment in section.windows(2) {
                map.add_segment(&segment[0], &segment[1]);
            }
        }
        map
    }

    pub fn add_segment(&mut self, a: &Vec2, b: &Vec2) {
        let min = a.min(b);
        let max = a.max(b);
        for x in min.x..=max.x {
            for y in min.y..=max.y {
                let p = Vec2 { x, y };
                self.add_point(p, '#');
            }
        }
    }

    pub fn add_point(&mut self, p: Vec2, c: char) {
        self.min = self.min.min(&p);
        self.max = self.max.max(&p);
        self.points.insert(p, c);
    }

    pub fn print(&self) {
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
