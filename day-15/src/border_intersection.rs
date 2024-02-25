use crate::{Aabb, Sensor, Vec2};

pub fn solve(sensors: &mut Vec<Sensor>, dimension: i32) -> (Vec2, i64) {
    // check for solutions in corners
    for point in &[
        Vec2 { x: 0, y: 0 },
        Vec2 { x: 0, y: dimension },
        Vec2 {
            x: dimension,
            y: dimension,
        },
        Vec2 { x: dimension, y: 0 },
    ] {
        if let Some(solution) = is_solution(point, sensors.iter(), dimension) {
            return solution;
        }
    }

    // Generate all border line segments from sensors
    let mut pos_slope_segments = Vec::new();
    let mut neg_slope_segment = Vec::new();
    for sensor in sensors.iter() {
        pos_slope_segments.extend(create_positive_slope_segments(sensor));
        neg_slope_segment.extend(create_negative_slope_segments(sensor));
    }

    // Find intersections and check for solutions
    for pos_slope_segment in pos_slope_segments.iter() {
        for neg_slope_segment in neg_slope_segment.iter() {
            let intersection = segment_intersection(pos_slope_segment, neg_slope_segment);
            if let Some((pos, is_in_center)) = intersection {
                let solution_candidates = if is_in_center {
                    [
                        Some(pos.clone()),
                        Some(Vec2 {
                            x: pos.x + 1,
                            y: pos.y,
                        }),
                        Some(Vec2 {
                            x: pos.x,
                            y: pos.y + 1,
                        }),
                        Some(Vec2 {
                            x: pos.x + 1,
                            y: pos.y + 1,
                        }),
                    ]
                } else {
                    [Some(pos), None, None, None]
                };
                for pos in solution_candidates
                    .into_iter()
                    .flatten()
                {
                    if pos.x < 0 || pos.y < 0 || pos.x > dimension || pos.y > dimension {
                        // Intersection is outside of the map
                        continue;
                    }
                    if let Some(solution) = is_solution(&pos, sensors.iter(), dimension) {
                        return solution;
                    }
                }
            }
        }
    }

    panic!("no solution found!");

    fn is_solution<'a>(
        point: &Vec2,
        sensors: impl Iterator<Item = &'a Sensor>,
        dimension: i32,
    ) -> Option<(Vec2, i64)> {
        for sensor in sensors {
            if point.manhattan_distance(&sensor.pos) <= sensor.range {
                return None;
            }
        }
        let answer = point.x as i64 * dimension as i64 + point.y as i64;
        Some((point.clone(), answer))
    }
}

fn create_positive_slope_segments(sensor: &Sensor) -> [Segment; 2] {
    [
        Segment {
            y_intercept: sensor.pos.y - sensor.pos.x - (sensor.range + 1),
            bounds: Aabb {
                x: (sensor.pos.x..sensor.pos.x + (sensor.range + 1) + 1),
                y: (sensor.pos.y - (sensor.range + 1)..sensor.pos.y + 1),
            },
        },
        Segment {
            y_intercept: sensor.pos.y - sensor.pos.x + (sensor.range + 1),
            bounds: Aabb {
                x: (sensor.pos.x - (sensor.range + 1)..sensor.pos.x + 1),
                y: (sensor.pos.y..sensor.pos.y + 1 + (sensor.range + 1)),
            },
        },
    ]
}

fn create_negative_slope_segments(sensor: &Sensor) -> [Segment; 2] {
    [
        Segment {
            y_intercept: sensor.pos.y + sensor.pos.x - (sensor.range + 1),
            bounds: Aabb {
                x: (sensor.pos.x - (sensor.range + 1)..sensor.pos.x + 1),
                y: (sensor.pos.y - (sensor.range + 1)..sensor.pos.y + 1),
            },
        },
        Segment {
            y_intercept: sensor.pos.y + sensor.pos.x + (sensor.range + 1),
            bounds: Aabb {
                x: (sensor.pos.x..sensor.pos.x + (sensor.range + 1) + 1),
                y: (sensor.pos.y..sensor.pos.y + 1 + (sensor.range + 1)),
            },
        },
    ]
}

/// Simplified line intersection algorithm for lines with slope 1 and -1.
/// Returns Option<(intersection_point, is_intersection_in_center)>
fn segment_intersection(
    pos_slope_segment: &Segment,
    neg_slope_segment: &Segment,
) -> Option<(Vec2, bool)> {
    let y = (pos_slope_segment.y_intercept + neg_slope_segment.y_intercept) / 2;
    let result = Vec2 {
        x: -pos_slope_segment.y_intercept + y,
        y,
    };
    if pos_slope_segment.bounds.contains(&result) && neg_slope_segment.bounds.contains(&result) {
        Some((
            result,
            pos_slope_segment.y_intercept % 2 != neg_slope_segment.y_intercept % 2,
        ))
    } else {
        None
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Segment {
    y_intercept: i32,
    bounds: Aabb,
}

#[cfg(test)]
mod tests {
    use crate::{
        border_intersection::{
            create_negative_slope_segments, create_positive_slope_segments, segment_intersection,
            solve, Segment,
        },
        test_case::{self, TestCase},
        Aabb, Sensor, Vec2,
    };

    fn test_case<T>(test_case: T)
    where
        TestCase: From<T>,
        T: std::fmt::Debug,
    {
        let mut test_case = TestCase::from(test_case);
        // Test each case in 4 rotations
        for _i in 0..4 {
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
            test_case(case);
        }
    }

    #[test]
    fn create_segments() {
        assert_eq!(
            create_positive_slope_segments(&Sensor {
                id: 0,
                pos: Vec2 { x: 0, y: 0 },
                range: 0,
            }),
            [
                Segment {
                    y_intercept: -1,
                    bounds: Aabb { x: 0..2, y: -1..1 }
                },
                Segment {
                    y_intercept: 1,
                    bounds: Aabb { x: -1..1, y: 0..2 }
                }
            ]
        );

        assert_eq!(
            create_negative_slope_segments(&Sensor {
                id: 0,
                pos: Vec2 { x: 0, y: 0 },
                range: 0,
            }),
            [
                Segment {
                    y_intercept: -1,
                    bounds: Aabb { x: -1..1, y: -1..1 }
                },
                Segment {
                    y_intercept: 1,
                    bounds: Aabb { x: 0..2, y: 0..2 }
                }
            ]
        );

        assert_eq!(
            create_positive_slope_segments(&Sensor {
                id: 0,
                pos: Vec2 { x: 2, y: 1 },
                range: 0,
            }),
            [
                Segment {
                    y_intercept: -2,
                    bounds: Aabb { x: 2..4, y: 0..2 }
                },
                Segment {
                    y_intercept: 0,
                    bounds: Aabb { x: 1..3, y: 1..3 }
                }
            ]
        );

        assert_eq!(
            create_negative_slope_segments(&Sensor {
                id: 0,
                pos: Vec2 { x: 2, y: 1 },
                range: 0,
            }),
            [
                Segment {
                    y_intercept: 2,
                    bounds: Aabb { x: 1..3, y: 0..2 }
                },
                Segment {
                    y_intercept: 4,
                    bounds: Aabb { x: 2..4, y: 1..3 }
                }
            ]
        );

        assert_eq!(
            create_positive_slope_segments(&Sensor {
                id: 0,
                pos: Vec2 { x: 1, y: 4 },
                range: 2,
            }),
            [
                Segment {
                    y_intercept: 0,
                    bounds: Aabb { x: 1..5, y: 1..5 }
                },
                Segment {
                    y_intercept: 6,
                    bounds: Aabb { x: -2..2, y: 4..8 }
                }
            ]
        );

        assert_eq!(
            create_negative_slope_segments(&Sensor {
                id: 0,
                pos: Vec2 { x: 1, y: 4 },
                range: 2,
            }),
            [
                Segment {
                    y_intercept: 2,
                    bounds: Aabb { x: -2..2, y: 1..5 },
                },
                Segment {
                    y_intercept: 8,
                    bounds: Aabb { x: 1..5, y: 4..8 },
                }
            ]
        );
    }

    #[test]
    fn intersect_segments() {
        assert_eq!(
            segment_intersection(
                &Segment {
                    y_intercept: 0,
                    bounds: Aabb { x: 0..4, y: 0..4 },
                },
                &Segment {
                    y_intercept: 3,
                    bounds: Aabb { x: 0..4, y: 0..4 },
                },
            ),
            Some((Vec2 { x: 1, y: 1 }, true))
        );

        assert_eq!(
            segment_intersection(
                &Segment {
                    y_intercept: 0,
                    bounds: Aabb { x: 0..4, y: 0..4 },
                },
                &Segment {
                    y_intercept: 4,
                    bounds: Aabb { x: 0..4, y: 0..4 },
                },
            ),
            Some((Vec2 { x: 2, y: 2 }, false))
        );

        assert_eq!(
            segment_intersection(
                &Segment {
                    y_intercept: 0,
                    bounds: Aabb { x: 0..4, y: 0..4 },
                },
                &Segment {
                    y_intercept: 4,
                    bounds: Aabb { x: 3..6, y: -1..2 },
                },
            ),
            None
        );

        assert_eq!(
            segment_intersection(
                &Segment {
                    y_intercept: 0,
                    bounds: Aabb { x: 0..2, y: 0..2 },
                },
                &Segment {
                    y_intercept: 4,
                    bounds: Aabb { x: 0..4, y: 0..4 },
                },
            ),
            None
        );
    }
}
