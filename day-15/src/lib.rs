use crate::range::{RangeOps, RangeSet};
use std::ops::Range;

pub mod range;

#[derive(Debug, Eq, Hash, PartialEq, Clone)]
pub struct Vec2 {
    pub x: i32,
    pub y: i32,
}

impl Vec2 {
    pub const fn manhattan_distance(&self, other: &Vec2) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }

    pub const fn sub(&self, other: &Vec2) -> Vec2 {
        Vec2 {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Sensor {
    line: usize,
    pub pos: Vec2,
    pub range: i32,
    x_diag_range: Range<i32>,
    y_diag_range: Range<i32>,
}

pub mod test_case {
    use crate::Vec2;

    pub const AOC_ACTUAL: (&str, i32, Vec2, i64) = (
        include_str!("../test_cases/aoc_actual.txt"),
        4_000_000,
        Vec2 {
            x: 3270298,
            y: 2638237,
        },
        13081194638237,
    );
    pub const AOC_EXAMPLE: (&str, i32, Vec2, i64) = (
        include_str!("../test_cases/aoc_example.txt"),
        20,
        Vec2 { x: 14, y: 11 },
        291,
    );

    pub const MINIMAL: (&str, i32, Vec2, i64) = (
        include_str!("../test_cases/minimal.txt"),
        3,
        Vec2 { x: 0, y: 0 },
        0,
    );

    pub const SMALL_1: (&str, i32, Vec2, i64) = (
        include_str!("../test_cases/small_1.txt"),
        3,
        Vec2 { x: 0, y: 0 },
        0,
    );
}

pub fn parse_and_solve_by_range_exclusion(input: &str, dimension: i32) -> (Vec2, i64) {
    solve_by_range_exclusion(&mut parse(input, dimension), dimension)
}

/// Solve by excluding ranges of coordinates that cannot be the solution.
///
/// Warning: This function does not behave correctly for inputs with no solution, or multiple
/// solutions
pub fn solve_by_range_exclusion(sensors: &mut Vec<Sensor>, dimension: i32) -> (Vec2, i64) {
    sensors.sort_by_key(|s| s.x_diag_range.start);
    let sensors_xdiag_ordered = sensors;

    let possible_x_diag_coords = exclude_x_diag_axis(sensors_xdiag_ordered, dimension);
    let possible_y_diag_coords = exclude_y_diag_axis(
        sensors_xdiag_ordered.iter(),
        possible_x_diag_coords.ranges[0].start,
        dimension,
    );

    let result_coord = diag_to_rectangular(
        &Vec2 {
            x: possible_x_diag_coords.ranges[0].start,
            y: possible_y_diag_coords.ranges[0].start,
        },
        dimension,
    );
    let answer = result_coord.x as i64 * dimension as i64 + result_coord.y as i64;
    return (result_coord, answer);

    /// Exclude ranges of coordinates in the x_diag axis that cannot be the solution. Combinations
    /// of overlapping sensors are used to exclude ranges of coordinates.
    ///
    /// Preconditions:
    /// - Sensors must be sorted by x_diag_range.start
    /// - There must be at least 1 sensor.
    fn exclude_x_diag_axis(sensors: &Vec<Sensor>, dimension: i32) -> RangeSet {
        #[derive(Debug, Clone)]
        struct Chain {
            x_axis: Range<i32>,
            y_axis: Range<i32>,
            sensors_idxs: Vec<usize>,
        }

        // Assert preconditions
        assert!(sensors.len() > 0, "There must be at least 1 sensor");
        for i in 1..sensors.len() {
            assert!(
                sensors[i - 1].x_diag_range.start <= sensors[i].x_diag_range.start,
                "Sensors must be sorted by x_diag_range.start"
            );
        }

        // Initialize main loop
        let dimension_diag = dimension * 2 - 1;
        let mut to_visit: Vec<Chain> = sensors
            .iter()
            .enumerate()
            .map(|(i, s)| Chain {
                x_axis: s.x_diag_range.clone(),
                y_axis: s.y_diag_range.clone(),
                sensors_idxs: vec![i],
            })
            .collect();
        let mut range_set = RangeSet::new();
        range_set.add_range(&(0..(dimension_diag)));

        // Main loop
        while let Some(current) = to_visit.pop() {
            // ===== Update range set based on the current chain ===== //
            let exclude_width = (dimension_diag / 2 - current.y_axis.start)
                .min(current.y_axis.end - dimension_diag / 2);
            let exclude_length = exclude_width + 1;
            let exclude_lower = (0..exclude_length).intersection(&current.x_axis);
            let exclude_upper =
                ((dimension_diag - exclude_length)..dimension_diag).intersection(&current.x_axis);
            range_set.subtract_range(&exclude_lower);
            range_set.subtract_range(&exclude_upper);

            // ===== Add new chains to visit ===== //
            let current_first_sensor_idx = current.sensors_idxs.first().unwrap();
            for (adjacent_candidate_idx, adjacent_candidate) in
                ((current_first_sensor_idx + 1)..sensors.len()).map(|i| (i, &sensors[i]))
            {
                let y_diag_axis_union = adjacent_candidate.y_diag_range.union(&current.y_axis);
                assert!(y_diag_axis_union.size() >= current.y_axis.size());
                let x_diag_axis_intersection = adjacent_candidate
                    .x_diag_range
                    .intersection(&current.x_axis);
                assert!(x_diag_axis_intersection.size() <= current.x_axis.size());
                if x_diag_axis_intersection.size() <= 0 {
                    if adjacent_candidate.x_diag_range.start > current.x_axis.end {
                        // Adjacent candidate starts after current ends in xdiag axis. Adjacent
                        // candidates are ordered so all upcoming candidates will also have no
                        // intersection and can be skipped.
                        break;
                    } else {
                        // Adjacent candidate starts before current starts in xdiag axis.
                        continue;
                    }
                } else if y_diag_axis_union.size() == current.y_axis.size()
                    || adjacent_candidate
                        .y_diag_range
                        .intersection(&current.y_axis)
                        .size()
                        < 0
                {
                    // No ydiag axis overlap.
                    continue;
                } else {
                    // xdiag and ydiag axis overlap. Add new chain to visit.
                    let mut new = current.clone();
                    new.sensors_idxs.push(adjacent_candidate_idx);
                    new.y_axis = y_diag_axis_union;
                    new.x_axis = x_diag_axis_intersection;
                    assert!(new.y_axis.start < new.y_axis.end);
                    to_visit.push(new);
                }
            }
        }
        range_set
    }

    /// Exclude ranges of coordinates in the ydiag axis that cannot be the solution given the
    /// solution's xdiag coordinate. Sensors are used to exclude ranges of coordinates one by one.
    /// There is no need to consider combinations of sensors because the xdiag coordinate is fixed.
    ///
    /// Preconditions:
    /// - Sensors must be sorted by x_diag_range.start
    fn exclude_y_diag_axis<'a, I: Iterator<Item = &'a Sensor>>(
        sensors: I,
        x_diag_coord: i32,
        dimension: i32,
    ) -> RangeSet {
        // Init main loop
        let mut result = RangeSet::new();
        let diag_midpoint = dimension - 1;
        let range_half_width = diag_midpoint - (-x_diag_coord + diag_midpoint).abs();
        result.add_range(&(diag_midpoint - range_half_width..diag_midpoint + range_half_width + 1));

        // Main loop
        for sensor in sensors {
            if sensor.x_diag_range.contains(&x_diag_coord) {
                let mut range_to_sub = sensor.y_diag_range.clone();
                // Depending on wither whether the xdiag coordinate is even or odd: the ydiag start
                // and end coordinates need to be even or odd. Expand the range to the nearest
                // odd/even coord by decrementing start and incrementing end as needed. This needs
                // be done because in diagonal space, moving one step in the direction of the xdiag
                // axis increases the ydiag coordinate by 2.
                if range_to_sub.start % 2 != x_diag_coord % 2 + dimension % 2 {
                    range_to_sub.start -= 1;
                }
                if range_to_sub.end % 2 == x_diag_coord % 2 + dimension % 2 {
                    range_to_sub.end += 1;
                }
                result.subtract_range(&range_to_sub);
            }
        }
        result
    }
}

pub fn parse_and_solve_by_column_skipping(intput: &str, dimension: i32) -> (Vec2, i64) {
    solve_by_column_skipping(&mut parse(intput, dimension), dimension)
}

pub fn solve_by_column_skipping(sensors: &mut Vec<Sensor>, dimension: i32) -> (Vec2, i64) {
    sensors.sort_by_key(|s| s.pos.y + s.range);
    let mut pos = Vec2 { x: 0, y: 0 };
    let mut iter_cnt = 0;
    let mut last_intersecting_sensor_idx = 0;

    let mut i = 0;
    let mut start_i = 0;
    loop {
        let sensor = &sensors[i];
        if iter_cnt != 0 && i == last_intersecting_sensor_idx {
            // We have reached a coord that's not in range of any sensor.
            break;
        }
        if sensor.pos.manhattan_distance(&pos) <= sensor.range {
            // Advance x past the sensors range
            pos.x = sensor.pos.x + sensor.range - (sensor.pos.y - pos.y).abs() + 1;
            if pos.x >= dimension {
                // Wrap around
                pos.x = 0;
                pos.y += 1;

                // Remove sensors from consideration which exist completely above the current y pos.
                while start_i < sensors.len()
                    && sensors[start_i].pos.y + sensors[start_i].range < pos.y
                {
                    start_i += 1;
                }
            }
            // Track last sensor that was used to advance x. This is used to detect when we have
            // reached a coord that's not in range of any sensor.
            last_intersecting_sensor_idx = i;
        }
        iter_cnt += 1;
        i += 1;
        if i >= sensors.len() {
            i = start_i;
        }
    }
    let answer = pos.x as i64 * dimension as i64 + pos.y as i64;
    let r = (pos, answer);
    r
}

pub fn solve_by_border_intersection(sensors: &mut Vec<Sensor>, dimension: i32) -> (Vec2, i64) {
    /*
    1. Generate pos_slope and neg_slope lines
    2. Find all intersections between pos_slope and neg_slope lines
        a. broad phase intersection detection (AABB). Optional, only as an optimisation
        b. line intersection
        c. line segment intersection (confirm line intersection is AABB)
     */

    // Generate all border line segemnts from sensors
    let mut pos_slope_segments = Vec::new();
    let mut neg_slope_segment = Vec::new();
    for sensor in sensors.iter() {
        pos_slope_segments.extend(create_positive_slope_segments(sensor));
        neg_slope_segment.extend(create_negative_slope_segments(sensor));
    }

    for pos_slope_segment in pos_slope_segments.iter() {
        for neg_slope_segment in neg_slope_segment.iter() {
            let intersection = segment_intersection(&pos_slope_segment, &neg_slope_segment);
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
                // iterate solution candidates which are is_some
                for pos in solution_candidates
                    .into_iter()
                    .filter(|c| c.is_some())
                    .map(|c| c.unwrap())
                {
                    if pos.x < 0 || pos.y < 0 || pos.x >= dimension || pos.y >= dimension {
                        // Intersection is outside of the map
                        continue;
                    }
                    let mut is_solution = true;
                    for sensor in sensors.iter() {
                        if pos.manhattan_distance(&sensor.pos) <= sensor.range {
                            is_solution = false;
                            break;
                        }
                    }
                    if is_solution {
                        let answer = pos.x as i64 * dimension as i64 + pos.y as i64;
                        return (pos, answer);
                    }
                }
            }
        }
    }

    todo!();
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
/// Returns Oprion<(intersection_point, is_intersection_in_center)>
fn segment_intersection(
    pos_slope_segment: &Segment,
    neg_slope_segment: &Segment,
) -> Option<(Vec2, bool)> {
    // TODO: Pre AABB check may be faster
    let y = (pos_slope_segment.y_intercept + neg_slope_segment.y_intercept) / 2;
    let result = Vec2 {
        x: -pos_slope_segment.y_intercept + y,
        y: y,
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
struct Aabb {
    x: Range<i32>,
    y: Range<i32>,
}

impl Aabb {
    fn contains(&self, vec: &Vec2) -> bool {
        self.x.contains(&vec.x) && self.y.contains(&vec.y)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Segment {
    y_intercept: i32,
    bounds: Aabb,
}

pub fn parse(input: &str, dimension: i32) -> Vec<Sensor> {
    fn parse_int(s: &str, trailing_chars: usize) -> i32 {
        (s[2..s.len() - trailing_chars]).parse().unwrap()
    }

    let mut sensors = Vec::new();
    for (line_idx, line) in input.lines().enumerate() {
        let mut words = line.split_ascii_whitespace().skip(2);
        let sensor_pos = Vec2 {
            x: parse_int(words.next().unwrap(), 1),
            y: parse_int(words.next().unwrap(), 1),
        };
        let mut words = words.skip(4);
        let beacon = Vec2 {
            x: parse_int(words.next().unwrap(), 1),
            y: parse_int(words.next().unwrap(), 0),
        };
        let sensor_pos_diag = rectangular_to_diag(&sensor_pos, dimension);

        let range = sensor_pos.manhattan_distance(&beacon);
        sensors.push(Sensor {
            pos: sensor_pos,
            line: line_idx + 1,
            range,
            x_diag_range: sensor_pos_diag.x - range..sensor_pos_diag.x + range + 1,
            y_diag_range: sensor_pos_diag.y - range..sensor_pos_diag.y + range + 1,
        });
    }
    sensors
}

/// Draw map for debugging.
///
/// Warnings:
/// - Breaks if there are more sensors than alphanumeric characters (36).
/// - Breaks if too many sensors overlap at the same coordinate.
#[allow(dead_code)]
pub fn draw_map<'a, T>(dimension: i32, border: i32, sensor_iter_fn: &dyn Fn() -> T)
where
    T: IntoIterator<Item = &'a Sensor>,
{
    let chars = [' ', '░', '▒', '▓', '█'];
    let start = (-border, -border);
    let end = (dimension + border, dimension + border);
    for y in start.1..end.1 {
        let mut line_1 = String::new();
        let mut line_2 = String::new();
        for x in start.0..end.0 {
            let mut overlap_cnt = 0;
            let is_outside = x < 0 || y < 0 || x >= dimension || y >= dimension;
            let mut is_border = false;
            let mut sensor_range = None;
            let mut sensor_line = None;
            for sensor in sensor_iter_fn() {
                if sensor.pos == (Vec2 { x, y }) {
                    sensor_range = Some(sensor.range);
                    sensor_line = Some(sensor.line);
                }
                let distance = sensor.pos.manhattan_distance(&Vec2 { x, y });
                if distance <= sensor.range {
                    overlap_cnt += 1;
                    if distance == sensor.range {
                        is_border = true;
                    }
                }
            }
            let cell_char = chars[overlap_cnt];
            line_1.push(cell_char);
            line_1.push(sensor_line.map_or(cell_char, |l| char::from_digit(l as u32, 36).unwrap()));
            line_1.push(if is_outside { '.' } else { cell_char });
            line_1.push(cell_char);

            line_2.push(cell_char);
            line_2.push(if is_border { '-' } else { cell_char });
            line_2
                .push(sensor_range.map_or(cell_char, |r| char::from_digit(r as u32, 36).unwrap()));
            line_2.push(cell_char);
        }
        println!("{line_1}");
        println!("{line_2}");
    }
}

fn rectangular_to_diag(vec: &Vec2, dimension: i32) -> Vec2 {
    Vec2 {
        x: vec.x + vec.y,
        y: vec.x - vec.y + dimension - 1,
    }
}

fn diag_to_rectangular(vec: &Vec2, dimension: i32) -> Vec2 {
    Vec2 {
        x: (-(dimension - 1) + vec.x + vec.y) / 2,
        y: (dimension - 1 + vec.x - vec.y) / 2,
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        parse_and_solve_by_column_skipping, parse_and_solve_by_range_exclusion,
        segment_intersection, test_case, Aabb, Segment, Sensor, Vec2,
    };

    #[test]
    fn create_segments() {
        assert_eq!(
            crate::create_positive_slope_segments(&Sensor {
                line: 0,
                pos: Vec2 { x: 0, y: 0 },
                range: 0,
                x_diag_range: 0..0,
                y_diag_range: 0..0
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
            crate::create_negative_slope_segments(&Sensor {
                line: 0,
                pos: Vec2 { x: 0, y: 0 },
                range: 0,
                x_diag_range: 0..0,
                y_diag_range: 0..0
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
            crate::create_positive_slope_segments(&Sensor {
                line: 0,
                pos: Vec2 { x: 2, y: 1 },
                range: 0,
                x_diag_range: 0..0,
                y_diag_range: 0..0
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
            crate::create_negative_slope_segments(&Sensor {
                line: 0,
                pos: Vec2 { x: 2, y: 1 },
                range: 0,
                x_diag_range: 0..0,
                y_diag_range: 0..0
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
            crate::create_positive_slope_segments(&Sensor {
                line: 0,
                pos: Vec2 { x: 1, y: 4 },
                range: 2,
                x_diag_range: 0..0,
                y_diag_range: 0..0
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
            crate::create_negative_slope_segments(&Sensor {
                line: 0,
                pos: Vec2 { x: 1, y: 4 },
                range: 2,
                x_diag_range: 0..0,
                y_diag_range: 0..0
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

    #[test]
    fn range_exclusion_aoc_actual_case() {
        assert_eq!(
            parse_and_solve_by_range_exclusion(test_case::AOC_ACTUAL.0, test_case::AOC_ACTUAL.1,),
            (test_case::AOC_ACTUAL.2, test_case::AOC_ACTUAL.3)
        );
    }

    #[test]
    fn range_exclusion_aoc_example_case() {
        assert_eq!(
            parse_and_solve_by_range_exclusion(test_case::AOC_EXAMPLE.0, test_case::AOC_EXAMPLE.1,),
            (test_case::AOC_EXAMPLE.2, test_case::AOC_EXAMPLE.3)
        );
    }

    #[test]
    fn column_skipping_aoc_actual_case() {
        assert_eq!(
            parse_and_solve_by_column_skipping(test_case::AOC_ACTUAL.0, test_case::AOC_ACTUAL.1,),
            (test_case::AOC_ACTUAL.2, test_case::AOC_ACTUAL.3)
        );
    }

    #[test]
    fn column_skipping_aoc_example_case() {
        assert_eq!(
            parse_and_solve_by_column_skipping(test_case::AOC_EXAMPLE.0, test_case::AOC_EXAMPLE.1,),
            (test_case::AOC_EXAMPLE.2, test_case::AOC_EXAMPLE.3)
        );
    }
}
