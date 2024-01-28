use day_15::{border_intersection::solve, test_case::FileTestCase};

const TEST_CASE: FileTestCase = day_15::test_case::AOC_ACTUAL;

fn main() {
    let mut sensors = day_15::parse(TEST_CASE.input);
    let result = solve(&mut sensors, TEST_CASE.search_size);
    dbg!(result);
}
