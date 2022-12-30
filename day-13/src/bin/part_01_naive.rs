use std::{cmp::Ordering, fs, iter::Peekable, str::Chars};

#[derive(Debug)]
enum Value {
    Integer(i32),
    List(Vec<Value>),
}

fn main() {
    const ITER_COUNT: usize = 10_000;
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
        let a = Value::parse_list(&mut lines.next().unwrap().chars().peekable());
        let b = Value::parse_list(&mut lines.next().unwrap().chars().peekable());
        if a.cmp(&b) != Ordering::Greater {
            correct_order_index_sum += i + 1;
        }
        if lines.next().is_none() {
            break;
        }
        i += 1;
    }
    correct_order_index_sum
}

impl Value {
    fn parse_list(chars: &mut Peekable<Chars>) -> Value {
        let opening_bracket = chars.next().unwrap();
        assert!(opening_bracket == '[');
        let mut result = Vec::new();
        loop {
            match chars.peek() {
                Some(']') => {
                    chars.next();
                    return Value::List(result);
                }
                Some('[') => result.push(Value::parse_list(chars)),
                Some(',') => {
                    chars.next();
                }
                Some(_) => result.push(Value::parse_integer(chars)),
                _ => panic!(),
            }
        }
    }

    fn parse_integer(chars: &mut Peekable<Chars>) -> Value {
        let mut buffer = String::new();
        loop {
            match chars.peek() {
                Some(c) if c.is_ascii_digit() => {
                    buffer.push(*c);
                    chars.next();
                }
                Some(_) => return Value::Integer(buffer.parse().unwrap()),
                _ => panic!(),
            }
        }
    }

    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Value::Integer(a), Value::Integer(b)) => a.cmp(b),
            (Value::List(a), Value::List(b)) => Value::cmp_list(a, b),
            (Value::Integer(a), Value::List(b)) => Value::cmp_list(&[Value::Integer(*a)], b),
            (Value::List(a), Value::Integer(b)) => Value::cmp_list(a, &[Value::Integer(*b)]),
        }
    }

    fn cmp_list(a: &[Value], b: &[Value]) -> Ordering {
        let mut iter_a = a.iter();
        let mut iter_b = b.iter();
        loop {
            let a = iter_a.next();
            let b = iter_b.next();
            match (a, b) {
                (None, None) => return Ordering::Equal,
                (None, Some(_)) => return Ordering::Less,
                (Some(_), None) => return Ordering::Greater,
                (Some(a), Some(b)) => {
                    let cmp = a.cmp(b);
                    if cmp != Ordering::Equal {
                        return cmp;
                    }
                }
            }
        }
    }
}
