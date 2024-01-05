use crate::{Sensor, Vec2};

pub fn solve(sensors: &mut Vec<Sensor>, search_area_size: i32) -> (Vec2, i64) {
    let mut pos = Vec2 { x: 0, y: 0 };
    'outer: loop {
        for sensor in sensors.iter() {
            if sensor.pos.manhattan_distance(&pos) <= sensor.range {
                // Advance x past the sensors range
                pos.x = sensor.pos.x + sensor.range - (sensor.pos.y - pos.y).abs() + 1;
                if pos.x > search_area_size {
                    // Wrap around
                    pos.x = 0;
                    pos.y += 1;
                }
                if pos.y > search_area_size {
                    panic!("No solution found");
                }
                continue 'outer;
            }
        }
        let result = pos.x as i64 * search_area_size as i64 + pos.y as i64;
        return (pos, result);
    }
}

#[cfg(test)]
mod tests {
    use crate::{column_skipping::solve, test_case::{TestCase, self}};

    fn test_case<T>(test_case: T) where TestCase: From<T> {
        let mut test_case = TestCase::from(test_case);
        for _ in 0..4 {
            let mut sensors = test_case
                .sensors
                .iter()
                .cloned()
                .collect();
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
            test_case(case);
        }
    }
}