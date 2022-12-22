use std::collections::HashMap;
use std::fs;
use std::io;
use std::io::BufRead;

const SIZE_THRESHOLD: u32 = 100000;

fn main() {
    // # Parse input
    let input_file = fs::File::open("input.txt").unwrap();
    let input_lines = io::BufReader::new(input_file).lines();

    let mut cur_path: Vec<String> = Vec::new();
    let mut directory_sizes: HashMap<String, u32> = HashMap::new();
    for line in input_lines.map(|l| l.unwrap()) {
        println!("{}", line);
        if line.starts_with("$ ls") || line.starts_with("dir") { /* Noop */
        } else if line.starts_with("$ cd ") {
            // Handle change directory
            cur_path = handle_cd(cur_path, &line);
            println!("  cur_dir: {}", cur_path.join("/"));
        } else if line.chars().nth(0).unwrap().is_ascii_digit() {
            // Handle file listing
            let size: u32 = line.split(' ').nth(0).unwrap().parse().unwrap();
            for i in 0..cur_path.len() {
                let sub_path = &cur_path[0..=i];
                let path_name = sub_path.join("/");
                let directory_size = directory_sizes.entry(path_name.clone()).or_default();
                *directory_size += size;
                println!("  {:?} size: {}", path_name, directory_size);
            }
        } else {
            panic!();
        }
    }

    // # Part 1
    println!("dir sizes: {:?}", directory_sizes);
    println!(
        "part 1: {}",
        directory_sizes
            .values()
            .filter(|s| { **s <= SIZE_THRESHOLD })
            .sum::<u32>()
    );

    // # Part 2
    const TOTAL_SPACE: u32 = 70_000_000;
    const REQUIRED_SPACE: u32 = 30_000_000;
    let used_space = directory_sizes["/"];
    let remaining_space = TOTAL_SPACE - used_space;
    let min_space_to_free = REQUIRED_SPACE - remaining_space;
    println!(
        "used: {}, remaining: {}, to free: {}",
        used_space, remaining_space, min_space_to_free
    );
    let mut values: Vec<u32> = directory_sizes.drain().map(|kv| kv.1).collect();
    values.sort();
    println!(
        "part 2: {}",
        values
            .into_iter()
            .filter(|s| *s >= min_space_to_free)
            .next()
            .unwrap()
    );
}

fn handle_cd(mut cur_path: Vec<String>, line: &str) -> Vec<String> {
    // Change current directory
    let new_dir = &line[5..];
    if new_dir == ".." {
        cur_path.pop();
    } else {
        cur_path.push(new_dir.to_string());
    }
    cur_path
}
