use crate::{Sensor, Vec2};

pub fn solve(sensors: &mut [Sensor], dimension: i32) -> (Vec2, i64) {
    const MAX_ITERS: i32 = 10_000_000;
    let mut count = 0;
    for x in 0..=dimension {
        'iter_points: for y in 0..=dimension {
            if count >= MAX_ITERS {
                panic!("Too many iterations: {count}");
            }
            count += 1;
            for sensor in sensors.iter() {
                if sensor.pos.manhattan_distance(&Vec2 { x, y }) <= sensor.range {
                    continue 'iter_points;
                }
            }
            let answer = x as i64 * dimension as i64 + y as i64;
            return (Vec2 { x, y }, answer);
        }
    }
    unreachable!("No solution found")
}

#[cfg(test)]
mod tests {
    use crate::{
        brute_force::solve,
        test_case::{self, TestCase},
    };

    fn test_case<T>(test_case: T)
    where
        TestCase: From<T>,
    {
        let mut test_case = TestCase::from(test_case);
        // Solve each test case in 4 different rotations.
        for r in 0..4 {
            println!("case: {:?}, rotation: {:?}", test_case.name, r);
            let mut sensors = test_case.sensors.to_vec();
            let result = solve(&mut sensors, test_case.dimension);
            assert_eq!(&result.0, &test_case.expected_pos);
            assert_eq!(result.1, test_case.expected_answer());
            test_case = test_case.rotated();
        }
    }

    #[test]
    fn const_test_cases() {
        for case in test_case::CONST_TEST_CASES {
            test_case(case);
        }
    }

    #[test]
    fn file_test_cases() {
        for case in test_case::FILE_TEST_CASES {
            if case.name == "AOC Actual" {
                // Skip AOC Actual case because it takes 3.5 days to run...
                continue;
            }
            test_case(case);
        }
    }
}
