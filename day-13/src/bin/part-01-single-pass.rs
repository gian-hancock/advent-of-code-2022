use std::str::Chars;
use std::{fs, cmp::Ordering};
use std::iter::Peekable;

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
        if compare_str(line_a, line_b) != Ordering::Greater {
            correct_order_index_sum += i + 1;
        }
        if lines.next() == None {
            break;
        }
        i += 1;
    }
    correct_order_index_sum
}

fn compare_str(a: &str, b: &str) -> Ordering {
    let mut chars_a = a.chars().peekable();
    let mut chars_b = b.chars().peekable();
    loop {
        let a_value = next_value(&mut chars_a);
        let b_value = next_value(&mut chars_b);
        let ordering = a_value.cmp(&b_value);
        if ordering != Ordering::Equal {
            return ordering
        }
    }
    
    fn next_value(chars: &mut Peekable<Chars>) -> (i32, i32) {
        let mut integer = None;
        let mut level = 0;
        loop {
            let c = chars.peek().unwrap();
            match c {
                ']' => {
                    match integer {
                        Some(i) => {
                            return (i, level);
                        }
                        _ => {
                            chars.next();
                            level -= 1;
                            return (-1, level);
                        }
                    }
                }
                '[' => {
                    chars.next();
                    level += 1
                }
                c if c.is_ascii_digit() => {
                    let integer = integer.get_or_insert(0);
                    *integer *= 10;
                    *integer += (*c as u32 - '0' as u32) as i32;
                    chars.next();
                }
                _ => {
                    chars.next();
                    match integer {
                        Some(i) => return (i, level),
                        _ => {}
                    }
                }
            }
        }
    }
}