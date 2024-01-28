use day_15::{column_skipping::solve, parse, test_case};

fn main() {
    let case = test_case::AOC_ACTUAL;
    let result = solve(&mut parse(case.input), case.search_size);
    dbg!(result);
}
