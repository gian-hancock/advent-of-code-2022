use day_15::{draw_map, Vec2};

// TODO: 
// const TEST_CASE: (&str, i32, Vec2, i64) = day_15::test_case::MINIMAL;
const TEST_CASE: (&str, i32, Vec2, i64) = day_15::test_case::AOC_ACTUAL;

fn main() {
    const DIMENSION: i32 = TEST_CASE.1;
    let mut sensors = day_15::parse(TEST_CASE.0, TEST_CASE.1);
    // draw_map(DIMENSION, 2, &|| &sensors);
    let result = day_15::solve_by_border_intersection(&mut sensors, TEST_CASE.1);
    dbg!(result);
}
