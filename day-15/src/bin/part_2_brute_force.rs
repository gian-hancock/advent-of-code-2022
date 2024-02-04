use day_15::{brute_force::solve, parse, test_case};

fn main() {
    let case = test_case::AOC_EXAMPLE;
    let result = solve(&mut parse(case.input), case.search_size);
    dbg!(result);
}