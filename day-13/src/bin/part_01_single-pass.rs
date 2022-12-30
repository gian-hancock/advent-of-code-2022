use day_13::compare_packets;
use std::{cmp::Ordering, fs};

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
        let line_a = lines.next().unwrap();
        let line_b = lines.next().unwrap();
        if compare_packets(line_a, line_b) != Ordering::Greater {
            correct_order_index_sum += i + 1;
        }
        if lines.next().is_none() {
            break;
        }
        i += 1;
    }
    correct_order_index_sum
}
