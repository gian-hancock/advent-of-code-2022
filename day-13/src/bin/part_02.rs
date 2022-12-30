use std::fs;
use day_13::compare_packets;

fn main() {
    const DIVIDER_1: &str = "[[2]]";
    const DIVIDER_2: &str = "[[6]]";
    let input = fs::read_to_string("input.txt").unwrap();
    let mut packets: Vec<&str> = input
        .lines()
        .filter(|l| !l.is_empty())
        .collect();
    packets.extend([DIVIDER_1,DIVIDER_2]);
    packets.sort_by(|a, b| compare_packets(a, b));
    let product: usize = packets
        .iter()
        .enumerate()
        .filter(|(_, p)| (**p).eq(DIVIDER_1) || (**p).eq(DIVIDER_2))
        .map(|(i, _)| i+1)
        .product();
    println!("{}", product);
}