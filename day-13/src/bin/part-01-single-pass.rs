use std::{fs, cmp::Ordering};
use std::iter::Peekable;

#[derive(Debug)]
struct Value {
    pub value: Option<i32>,
    pub level: i32,
}

fn main() {
    const ITER_COUNT: usize = 1;
    let mut total = 0;
    for _ in 0..ITER_COUNT {
        total += run();
    }
    println!("{}", total);
}

fn run() -> i32 {
    let input = fs::read_to_string("input.txt").unwrap();
    let mut lines = input.lines();
    let mut correct_order_index_sum = 0;
    let mut i = 0;
    loop {
        let line_a = lines.next().unwrap();
        let line_b = lines.next().unwrap();
        let scanner_a = Scanner::new(line_a.chars());
        let scanner_b = Scanner::new(line_b.chars());
        let is_correct_order = compare(scanner_a, scanner_b) != Ordering::Greater;
        if is_correct_order {
            correct_order_index_sum += i + 1;
        }
        if lines.next() == None {
            break;
        }
        i += 1;
    }
    correct_order_index_sum
}

fn compare(
    mut a: impl Iterator<Item = Value>,
    mut b: impl Iterator<Item = Value>,
) -> Ordering {
    while let Some(value_a) = a.next() {
        let value_b = b.next().unwrap();
        let mut ordering = value_a.value.unwrap_or(-1).cmp(&value_b.value.unwrap_or(-1));
        if ordering == Ordering::Equal {
            ordering = value_a.level.cmp(&value_b.level);
        }
        if ordering != Ordering::Equal {
            return ordering
        }
    }
    Ordering::Equal
}

struct Scanner<T: Iterator<Item = char>> {
    chars: Peekable<T>,
    level: i32,
}

impl<T: Iterator<Item = char>> Scanner<T> {
    fn new(chars: T) -> Scanner<T> where T: Iterator<Item = char> {
        Scanner { chars: chars.peekable(), level: 0 }
    }

    fn next_value(&mut self) -> Value {
        let mut buffer = String::new();
        loop {
            match self.chars.peek() {
                Some('[') => {
                    self.chars.next();
                    self.level += 1;
                }
                Some(']') => {
                    self.level -= 1;
                    if buffer.len() > 0 {
                        return Value {
                            value: Some(buffer.parse().unwrap()),
                            level: self.level + 1
                        }
                    } else {
                        self.chars.next();
                        return Value {
                            value: None,
                            level: self.level + 1
                        }
                    }
                }
                Some(',') => {
                    self.chars.next();
                    if let Ok(value) = buffer.parse() {
                        return Value { value: Some(value), level: self.level }
                    }
                }
                _ => {
                    buffer.push(self.chars.next().unwrap());
                }
            }
        }
    }
}

impl<T: Iterator<Item = char>> Iterator for Scanner<T> {
    type Item = Value;

    fn next(&mut self) -> Option<Self::Item> {
        let r = self.next_value();
        Some(r)
    }
}