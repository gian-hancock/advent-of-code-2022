use std::collections::HashSet;
use std::fs;

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

    part_2(grid, height, width);
}

fn part_2(grid: Vec<i32>, height: usize, width: usize) {
    // # Iterate grid cells
    let mut max_scenic_score = (0, 0, 0);
    for row_idx in 0..height {
        for col_idx in 0..width {
            let cur_cell_idx = row_idx * width + col_idx;
            let cur_height = grid[cur_cell_idx];
            // ## Search upwards
            let mut view_distance_up = 0;
            for search_row_idx in (0..row_idx).rev() {
                view_distance_up += 1;
                let search_cell_idx = search_row_idx * width + col_idx;
                let search_height = grid[search_cell_idx];
                if search_height >= cur_height {
                    break;
                }
            }
            // ## Search downwards
            let mut view_distance_down = 0;
            for search_row_idx in row_idx + 1..height {
                view_distance_down += 1;
                let search_cell_idx = search_row_idx * width + col_idx;
                let search_height = grid[search_cell_idx];
                if search_height >= cur_height {
                    break;
                }
            }
            // ## Search leftwards
            let mut view_distance_left = 0;
            for search_col_idx in (0..col_idx).rev() {
                view_distance_left += 1;
                let search_cell_idx = row_idx * width + search_col_idx;
                let search_height = grid[search_cell_idx];
                if search_height >= cur_height {
                    break;
                }
            }
            // ## Search rightwards
            let mut view_distance_right = 0;
            for search_col_idx in col_idx+1..width {
                view_distance_right += 1;
                let search_cell_idx = row_idx * width + search_col_idx;
                let search_height = grid[search_cell_idx];
                if search_height >= cur_height {
                    break;
                }
            }
            let scenic_score =
                view_distance_up * view_distance_down * view_distance_left * view_distance_right;
            print!(
                "{:?}",
                (
                    view_distance_up,
                    view_distance_down,
                    view_distance_left,
                    view_distance_right,
                    scenic_score,
                )
            );
            if scenic_score > max_scenic_score.2 {
                max_scenic_score = (col_idx, row_idx, scenic_score);
            }
        }
        println!();
    }
    println!("{:?}", max_scenic_score);
}

fn part_1(grid: Vec<i32>, height: usize, width: usize) {
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

    println!("visible trees: {}", visible.len());
}
