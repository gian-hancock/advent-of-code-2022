use day_15::{test_case, parse, column_skipping::solve};

fn main() {
    let case = test_case::AOC_ACTUAL;
    let result = solve(&mut parse(case.input), case.search_size);
    dbg!(result);
}