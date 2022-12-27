use std::{collections::VecDeque, fs};

const INPUT_PATH: &str = "input.txt";
const ROUND_CNT: usize = 20;

fn main() {
    // # Parse input
    let input = fs::read_to_string(INPUT_PATH).unwrap();
    let mut lines = input.lines();
    let mut monkeys = Vec::new();
    // ## Parse each Monkey
    while let Some(_) = lines.next() {
        // ### Parse starting items
        let line = &lines.next().unwrap()["  Starting items: ".len()..];
        let mut starting_items = VecDeque::new();
        for item in line.split(',') {
            let item = item.trim();
            starting_items.push_back(item.parse().unwrap());
        }

        // ### Parse operation
        let line = &lines.next().unwrap()["  Operation: new = old ".len()..];
        let mut tokens = line.split_ascii_whitespace();
        let operator = tokens.next().unwrap();
        let operand = tokens.next().unwrap();
        let monkey_op = if operand.eq("old") {
            Op {
                exponent: 2,
                multiply: 1,
                add: 0,
            }
        } else if operator.eq("+") {
            Op {
                exponent: 1,
                multiply: 1,
                add: operand.parse().unwrap(),
            }
        } else if operator.eq("*") {
            Op {
                exponent: 1,
                multiply: operand.parse().unwrap(),
                add: 0,
            }
        } else {
            panic!();
        };

        // ### Parse divisor
        let line = &lines.next().unwrap()["  Test: divisible by ".len()..];
        let divisor = line.parse().unwrap();

        // ### Parse Target A
        let line = &lines.next().unwrap()["    If true: throw to monkey ".len()..];
        let target_on_success = line.parse().unwrap();

        // ### Parse Target B
        let line = &lines.next().unwrap()["    If false: throw to monkey ".len()..];
        let target_on_fail = line.parse().unwrap();

        monkeys.push(Monkey {
            items: starting_items,
            operation: monkey_op,
            divisor: divisor,
            target_on_success,
            target_on_fail,
            inspected_cnt: 0,
        });
        lines.next();
    }

    println!("Initial state:");
    print_monkeys(monkeys.iter());
    println!();

    // # Process rounds
    for _ in 0..ROUND_CNT {
        for monkey_idx in 0..monkeys.len() {
            monkeys[monkey_idx].inspected_cnt += monkeys[monkey_idx].items.len();
            for _ in 0..monkeys[monkey_idx].items.len() {
                let monkey = &mut monkeys[monkey_idx];
                let op = &monkey.operation;
                let item = monkey.items.pop_front().unwrap();
                let new_value = (item.pow(op.exponent) * op.multiply + op.add) / 3;
                let new_monkey_idx = if new_value % monkeys[monkey_idx].divisor == 0 {
                    monkeys[monkey_idx].target_on_success
                } else {
                    monkeys[monkey_idx].target_on_fail
                };
                // println!("monkey[{}] = {} => monkey[{}] = {}", monkey_idx, item, new_monkey_idx, new_value);
                monkeys[new_monkey_idx].items.push_back(new_value);
            }
        }
        print_monkeys(monkeys.iter());
        println!();
    }
    monkeys.sort_by_key(|m| m.inspected_cnt);
    let shenanigans: usize = monkeys
        .iter()
        .map(|m| m.inspected_cnt)
        .rev()
        .take(2)
        .reduce(|a, i| a * i)
        .unwrap();
    println!("Shenanigans: {}", shenanigans);
}

fn print_monkeys<'a>(monkeys: impl Iterator<Item = &'a Monkey>) {
    for (i, monkey) in monkeys.enumerate() {
        println!(
            "{}: items: {:?}, inspected: {}",
            i, monkey.items, monkey.inspected_cnt
        );
    }
}

#[derive(Debug)]
struct Monkey {
    items: VecDeque<i32>,
    operation: Op,
    divisor: i32,
    target_on_success: usize,
    target_on_fail: usize,
    inspected_cnt: usize,
}

#[derive(Debug)]
struct Op {
    exponent: u32,
    multiply: i32,
    add: i32,
}
