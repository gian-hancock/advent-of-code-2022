use std::collections::HashSet;
use std::{fs};

fn main() {
    // # Parse input file
    let input = fs::read_to_string("input.txt").unwrap();
    let width = input.lines().nth(0).unwrap().len();
    let height = input.len() / width;
    println!("Grid size: {}x{}", width, height);
    let grid: Vec<i32> = input
        .chars()
        .filter(|c| c.is_ascii_digit())
        .map(|c| c.to_digit(10).unwrap() as i32)
        .collect();
    println!("{:?}", grid);

    // # Process grid
    let mut visible = HashSet::new();
    // ## Scan horizontal
    for row_idx in 0..height {
        // ### Left to right
        let mut max_height = 0;
        for col_idx in 0..width {
            let cell_id = row_idx * width + col_idx;
            let cell_height = grid[cell_id];
            if cell_height >= max_height {
                max_height = cell_height + 1;
                visible.insert(cell_id);
            }
        }
        // ### Right to left
        let mut max_height = 0;
        for col_idx in (0..width).rev() {
            let cell_id = row_idx * width + col_idx;
            let cell_height = grid[cell_id];
            if cell_height >= max_height {
                max_height = cell_height + 1;
                visible.insert(cell_id);
            }
        }
    }
    // ## Scan vertical
    for col_idx in 0..width {
        // ### Top to bottom
        let mut max_height = 0;
        for row_index in 0..height {
            let cell_id = row_index * width + col_idx;
            let cell_height = grid[cell_id];
            if cell_height >= max_height {
                max_height = cell_height + 1;
                visible.insert(cell_id);
            }
        }
        // ### Bottom to top
        let mut max_height = 0;
        for row_index in (0..height).rev() {
            let cell_id = row_index * width + col_idx;
            let cell_height = grid[cell_id];
            if cell_height >= max_height {
                max_height = cell_height + 1;
                visible.insert(cell_id);
            }
        }
    }

    // # Print output
    for row_idx in 0..height {
        for col_idx in 0..width {
            let cell_id = row_idx * width + col_idx;
            if visible.contains(&cell_id) {
                print!("x");
            } else {
                print!(".");
            }
        }
        println!("");
    }

    // # Part 1
    println!("visible trees: {}", visible.len());
}
