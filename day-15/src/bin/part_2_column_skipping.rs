use day_15::test_case;

fn main() {
    let result = day_15::parse_and_solve_by_range_exclusion(
        test_case::AOC_ACTUAL.0,
        test_case::AOC_ACTUAL.1,
    );
    dbg!(result);
}

/*
thread 'tests::create_segments' panicked at 'assertion failed: `(left == right)`
  left: `[
    Segment { y_intercept: -1, bounds: Aabb { x: -1..1, y: -1..1 } },
    Segment { y_intercept: 1, bounds: Aabb { x: 0..2, y: 0..2 } }]`,
 right: `[
    Segment { y_intercept: 0, bounds: Aabb { x: -1..1, y: -1..1 } },
    Segment { y_intercept: 2, bounds: Aabb { x: 0..2, y: 0..2 } }]`', src\lib.rs:511:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
*/
