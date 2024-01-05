use day_15::{border_intersection::solve, test_case::FileTestCase};

// TODO:
// const TEST_CASE: (&str, i32, Vec2, i64) = day_15::test_case::MINIMAL;
const TEST_CASE: FileTestCase = day_15::test_case::AOC_ACTUAL;

fn main() {
    let mut sensors = day_15::parse(TEST_CASE.input);
    // draw_map(DIMENSION, 2, &|| &sensors);
    let result = solve(&mut sensors, TEST_CASE.search_size);
    dbg!(result);
}
