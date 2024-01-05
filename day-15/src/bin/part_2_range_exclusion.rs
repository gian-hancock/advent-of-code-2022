use day_15::{
    range_exclusion::aabb_from_sensor,
    test_case::{self, TestCase},
    Aabb,
};

fn main() {
    let case = TestCase::from(&test_case::AOC_ACTUAL);
    let mut sensors: Vec<Aabb> = case
        .sensors
        .iter()
        .map(|s| aabb_from_sensor(s, case.dimension))
        .collect();
    let result = day_15::range_exclusion::solve(&mut sensors, case.dimension);
    dbg!(result);
}
