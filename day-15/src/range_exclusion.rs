use std::ops::Range;

use crate::{Aabb, Sensor, Vec2};

/// Solve by excluding ranges of coordinates that cannot be the solution.
///
/// Warning: This function does not behave correctly for inputs with no solution, or multiple
/// solutions
pub fn solve(sensors_diag: &mut Vec<Aabb>, dimension: i32) -> (Vec2, i64) {
    sensors_diag.sort_by_key(|s| s.x.start);
    let sensors_xdiag_ordered = sensors_diag;

    let possible_x_diag_coords = exclude_x_diag_axis(sensors_xdiag_ordered, dimension);
    let possible_y_diag_coords = exclude_y_diag_axis(
        sensors_xdiag_ordered.iter(),
        possible_x_diag_coords.ranges[0].start,
        dimension,
    );

    assert_eq!(possible_x_diag_coords.ranges.len(), 1);
    assert_eq!(possible_y_diag_coords.ranges.len(), 1);
    // We may may get up to 3 candidate solutions in diagonal space, however only 1 of them will
    // correspond to an integer point in rectangular space. Find that one.
    for x_diag in possible_x_diag_coords.ranges[0].clone() {
        for y_diag in possible_y_diag_coords.ranges[0].clone() {
            if ((x_diag + y_diag) % 2) == (dimension % 2) {
                let result_pos = diagonal_to_rectangular(
                    &Vec2 {
                        x: x_diag,
                        y: y_diag,
                    },
                    dimension,
                );
                let result = result_pos.x as i64 * dimension as i64 + result_pos.y as i64;
                return (result_pos, result);
            }
        }
    }
    panic!("No solution found");
}

/// Exclude ranges of coordinates in the x_diag axis that cannot be the solution. Combinations
/// of overlapping sensors are used to exclude ranges of coordinates.
///
/// Preconditions:
/// - Sensors must be sorted by x_diag_range.start
/// - There must be at least 1 sensor.
fn exclude_x_diag_axis(sensors: &Vec<Aabb>, dimension: i32) -> RangeSet {
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
            sensors[i - 1].x.start <= sensors[i].x.start,
            "Sensors must be sorted by x_diag_range.start"
        );
    }

    // Initialize main loop
    let dimension_diag = dimension * 2;
    let mut to_visit: Vec<Chain> = sensors
        .iter()
        .enumerate()
        .map(|(i, s)| Chain {
            x_axis: s.x.clone(),
            y_axis: s.y.clone(),
            sensors_idxs: vec![i],
        })
        .collect();
    let mut range_set = RangeSet::new();
    range_set.add_range(&(0..(dimension_diag + 1)));

    // Main loop
    while let Some(current) = to_visit.pop() {
        // ===== Update range set based on the current chain ===== //
        // range from center of y_diag axis that is excluded
        let y_diag_exclude_range = (dimension_diag / 2 - current.y_axis.start)
            .min(current.y_axis.end - 1 - dimension_diag / 2);
        let x_diag_exclude_lower = (0..y_diag_exclude_range + 1).intersection(&current.x_axis);
        let x_diag_exclude_upper = ((dimension_diag - y_diag_exclude_range)..(dimension_diag + 1))
            .intersection(&current.x_axis);
        range_set.subtract_range(&x_diag_exclude_lower);
        range_set.subtract_range(&x_diag_exclude_upper);

        // ===== Add new chains to visit ===== //
        let current_first_sensor_idx = current.sensors_idxs.first().unwrap();
        for (adjacent_candidate_idx, adjacent_candidate) in
            ((current_first_sensor_idx + 1)..sensors.len()).map(|i| (i, &sensors[i]))
        {
            let y_diag_axis_union = adjacent_candidate.y.union(&current.y_axis);
            assert!(y_diag_axis_union.size() >= current.y_axis.size());
            let x_diag_axis_intersection = adjacent_candidate.x.intersection(&current.x_axis);
            assert!(x_diag_axis_intersection.size() <= current.x_axis.size());
            if x_diag_axis_intersection.size() <= 0 {
                if adjacent_candidate.x.start > current.x_axis.end {
                    // Adjacent candidate starts after current ends in xdiag axis. Adjacent
                    // candidates are ordered so all upcoming candidates will also have no
                    // intersection and can be skipped.
                    break;
                } else {
                    // Adjacent candidate starts before current starts in xdiag axis.
                    continue;
                }
            } else if y_diag_axis_union.size() == current.y_axis.size()
                || adjacent_candidate.y.intersection(&current.y_axis).size() < 0
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
fn exclude_y_diag_axis<'a, I: Iterator<Item = &'a Aabb>>(
    sensors: I,
    x_diag_coord: i32,
    dimension: i32,
) -> RangeSet {
    // Init main loop
    let mut result = RangeSet::new();
    let diag_midpoint = dimension;
    let x_diag_distance_from_midpoint = (x_diag_coord - diag_midpoint).abs();
    let y_diag_lower_bound = x_diag_distance_from_midpoint;
    let y_diag_upper_bound = dimension * 2 - x_diag_distance_from_midpoint;
    result.add_range(&(y_diag_lower_bound..y_diag_upper_bound + 1));

    // Main loop
    for sensor in sensors {
        if sensor.x.contains(&x_diag_coord) {
            let mut range_to_sub = sensor.y.clone();
            // // Depending on wither whether the xdiag coordinate is even or odd: the ydiag start
            // // and end coordinates need to be even or odd. Expand the range to the nearest
            // // odd/even coord by decrementing start and incrementing end as needed. This needs
            // // be done because in diagonal space, moving one step in the direction of the xdiag
            // // axis increases the ydiag coordinate by 2.
            // if range_to_sub.start % 2 != x_diag_coord % 2 + dimension % 2 {
            //     range_to_sub.start -= 1;
            // }
            // if range_to_sub.end % 2 == x_diag_coord % 2 + dimension % 2 {
            //     range_to_sub.end += 1;
            // }
            result.subtract_range(&range_to_sub);
        }
    }
    result
}

fn rectangular_to_diagonal(vec: &Vec2, dimension: i32) -> Vec2 {
    Vec2 {
        x: vec.x - vec.y + dimension,
        y: vec.x + vec.y,
    }
}

fn diagonal_to_rectangular(vec: &Vec2, dimension: i32) -> Vec2 {
    Vec2 {
        x: (-dimension + vec.x + vec.y) / 2,
        y: (dimension - vec.x + vec.y) / 2,
    }
}

pub trait RangeOps {
    fn overlaps(&self, other: &Self) -> bool;
    fn touches(&self, other: &Self) -> bool;
    fn after(&self, other: &Self) -> bool;
    fn before(&self, other: &Self) -> bool {
        other.after(self)
    }
    fn truncate_after(&self, truncate_at: i32) -> Self;
    fn truncate_before(&self, truncate_at: i32) -> Self;
    fn intersection(&self, other: &Self) -> Self;
    fn union(&self, other: &Self) -> Self;
    fn size(&self) -> i32;
}

impl RangeOps for std::ops::Range<i32> {
    fn overlaps(&self, other: &std::ops::Range<i32>) -> bool {
        self.start < other.end && self.end > other.start
    }

    fn touches(&self, other: &std::ops::Range<i32>) -> bool {
        self.start <= other.end && self.end >= other.start
    }

    fn after(&self, other: &std::ops::Range<i32>) -> bool {
        self.start >= other.end
    }

    fn truncate_after(&self, truncate_at_exclusive: i32) -> Range<i32> {
        Range {
            start: self.start,
            end: self.end.min(truncate_at_exclusive),
        }
    }

    fn truncate_before(&self, truncate_at_inclusive: i32) -> Range<i32> {
        Range {
            start: self.start.max(truncate_at_inclusive),
            end: self.end,
        }
    }

    fn intersection(&self, other: &std::ops::Range<i32>) -> Range<i32> {
        Range {
            start: self.start.max(other.start),
            end: self.end.min(other.end),
        }
    }

    fn union(&self, other: &std::ops::Range<i32>) -> Range<i32> {
        Range {
            start: self.start.min(other.start),
            end: self.end.max(other.end),
        }
    }

    fn size(&self) -> i32 {
        self.end - self.start
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RangeSet {
    pub ranges: Vec<Range<i32>>,
}

impl Default for RangeSet {
    fn default() -> Self {
        Self::new()
    }
}

impl RangeSet {
    pub fn new() -> Self {
        RangeSet { ranges: Vec::new() }
    }

    pub fn add_range(&mut self, range_to_add: &Range<i32>) {
        if range_to_add.is_empty() {
            return;
        }
        // Find all existing ranges which overlap with `range`
        let mut existing_ranges = self.ranges.iter().enumerate();
        // First iterate to first touching range
        let mut touching_range_start_idx = None;
        let mut insert_at_idx = 0;
        for (i, range) in existing_ranges.by_ref() {
            insert_at_idx = i;
            if range.touches(range_to_add) {
                // Found the forst touching range
                touching_range_start_idx = Some(i);
                insert_at_idx = i;
                break;
            } else if range_to_add.after(range) {
                // Found a that comes after `range_to_insert` and does not touch it. This means
                // that all of the subsequent ranges will also not touch and we can break early
                insert_at_idx = i + 1;
                break;
            }
        }
        match touching_range_start_idx {
            None => {
                // There are no touching ranges. Add this range to the list
                self.ranges.insert(insert_at_idx, range_to_add.clone());
            }
            Some(touching_range_start_idx) => {
                let mut touching_range_end_idx = touching_range_start_idx + 1;
                for (i, range) in existing_ranges {
                    if range.touches(range_to_add) {
                        touching_range_end_idx = i + 1;
                    } else {
                        break;
                    }
                }
                let merged = Range {
                    start: self.ranges[touching_range_start_idx].start,
                    end: self.ranges[touching_range_end_idx - 1].end,
                };
                self.ranges
                    .splice(touching_range_start_idx..touching_range_end_idx, [merged]);
            }
        }
    }

    pub fn subtract_range(&mut self, range_to_subtract: &Range<i32>) {
        if range_to_subtract.is_empty() {
            return;
        }
        // Find range before and after range_to_subtract
        let mut before_idx = -1;
        let mut after_idx = self.ranges.len() as i32;
        // TODO: Short circuit
        for (i, range) in self.ranges.iter().enumerate() {
            if range.before(range_to_subtract) {
                before_idx = i as i32;
            }
            if range.after(range_to_subtract) {
                after_idx = after_idx.min(i as i32);
            }
        }

        let mut replacements = Vec::new();
        let left_range_to_truncate_idx = before_idx + 1;
        if left_range_to_truncate_idx >= 0 && left_range_to_truncate_idx < self.ranges.len() as i32
        {
            let left_replacement = self.ranges[left_range_to_truncate_idx as usize]
                .truncate_after(range_to_subtract.start);
            if !left_replacement.is_empty() {
                replacements.push(left_replacement);
            }
        }
        let right_range_to_truncate_idx = (after_idx - 1) as usize;
        if right_range_to_truncate_idx < self.ranges.len() {
            let right_replacement =
                self.ranges[right_range_to_truncate_idx].truncate_before(range_to_subtract.end);
            if !right_replacement.is_empty() {
                replacements.push(right_replacement);
            }
        }
        self.ranges
            .splice((before_idx + 1) as usize..after_idx as usize, replacements);
    }
}

pub fn aabb_from_sensor(s: &Sensor, dimension: i32) -> Aabb {
    let diag_pos = rectangular_to_diagonal(&s.pos, dimension);
    Aabb {
        x: diag_pos.x - s.range..diag_pos.x + s.range + 1,
        y: diag_pos.y - s.range..diag_pos.y + s.range + 1,
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        range_exclusion::{
            aabb_from_sensor, diagonal_to_rectangular, rectangular_to_diagonal, solve, RangeOps,
            RangeSet,
        },
        test_case::{self, ConstTestCase, FileTestCase, TestCase},
        Vec2,
    };

    #[test]
    fn coordinate_space() {
        struct Case {
            dimension: i32,
            rectangular: Vec2,
            diagonal: Vec2,
        }
        let cases = vec![
            Case {
                dimension: 6,
                rectangular: Vec2 { x: 0, y: 0 },
                diagonal: Vec2 { x: 6, y: 0 },
            },
            Case {
                dimension: 6,
                rectangular: Vec2 { x: 0, y: 6 },
                diagonal: Vec2 { x: 0, y: 6 },
            },
            Case {
                dimension: 6,
                rectangular: Vec2 { x: 4, y: 2 },
                diagonal: Vec2 { x: 8, y: 6 },
            },
            Case {
                dimension: 4,
                rectangular: Vec2 { x: 0, y: 0 },
                diagonal: Vec2 { x: 4, y: 0 },
            },
            Case {
                dimension: 4,
                rectangular: Vec2 { x: 0, y: 4 },
                diagonal: Vec2 { x: 0, y: 4 },
            },
            Case {
                dimension: 4,
                rectangular: Vec2 { x: 2, y: 1 },
                diagonal: Vec2 { x: 5, y: 3 },
            },
        ];

        for case in cases {
            println!(
                "Testing {:?} -> {:?}; dim={:?}",
                case.rectangular, case.diagonal, case.dimension
            );
            assert_eq!(
                rectangular_to_diagonal(&case.rectangular, case.dimension),
                case.diagonal
            );
            println!(
                "Testing {:?} -> {:?}; dim={:?}",
                case.diagonal, case.rectangular, case.dimension
            );
            assert_eq!(
                diagonal_to_rectangular(&case.diagonal, case.dimension),
                case.rectangular
            );
        }
    }

    fn test_case<T>(test_case: T)
    where
        TestCase: From<T>,
    {
        let mut test_case = TestCase::from(test_case);
        for _ in 0..4 {
            let mut sensors = test_case
                .sensors
                .iter()
                .map(|s| aabb_from_sensor(s, test_case.dimension))
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

    #[test]
    fn test_range() {
        assert!((3..4).after(&(2..3)));
        assert!((2..3).before(&(3..4)));
        assert!(!(2..3).overlaps(&(3..4)));
        assert!((2..3).overlaps(&(2..4)));
        assert!((2..3).touches(&(3..4)));
        assert!(!(2..3).touches(&(4..5)));
        assert!(!(-100..-51).before(&(-75..4)));
        assert!(!(-75..4).after(&(-100..-51)));
        assert_eq!((0..100).truncate_after(50), 0..50);
        assert_eq!((0..100).truncate_before(50), 50..100);
        assert_eq!((0..100).truncate_after(100), 0..100);
        assert_eq!((0..100).truncate_before(0), 0..100);
        assert!((0..100).truncate_after(-100).len() == 0);
        dbg!((0..100).truncate_after(-100));
        assert!((0..100).truncate_before(200).len() == 0);
    }

    #[test]
    fn test_range_set() {
        // Test empty RangeSet
        let mut range_set = RangeSet::new();
        assert_eq!(range_set, RangeSet { ranges: vec![] });

        // Test adding first range
        range_set.add_range(&(2..3));
        assert_eq!(range_set, RangeSet { ranges: vec![2..3] });

        // Test adding non touching range at end
        range_set.add_range(&(4..5));
        assert_eq!(
            range_set,
            RangeSet {
                ranges: vec![2..3, 4..5]
            }
        );

        // Test adding range which causes all existing ranges to be merged
        range_set.add_range(&(3..4));
        assert_eq!(range_set, RangeSet { ranges: vec![2..5] });

        // Test adding redundant range
        range_set.add_range(&(2..4));
        assert_eq!(range_set, RangeSet { ranges: vec![2..5] });

        // Test adding redundant empty range
        range_set.add_range(&(10..-10));
        assert_eq!(range_set, RangeSet { ranges: vec![2..5] });

        // Test adding non touching range at start
        range_set.add_range(&(-100..-51));
        assert_eq!(
            range_set,
            RangeSet {
                ranges: vec![-100..-51, 2..5],
            }
        );

        // Test adding non touching range in middle
        range_set.add_range(&(-20..-15));
        assert_eq!(
            range_set,
            RangeSet {
                ranges: vec![-100..-51, -20..-15, 2..5],
            }
        );

        // Test redundant subtraction after all existing groups
        range_set.subtract_range(&(100..110));
        assert_eq!(
            range_set,
            RangeSet {
                ranges: vec![-100..-51, -20..-15, 2..5],
            }
        );

        // Test redundant subtraction before all existing groups
        range_set.subtract_range(&(-1000..-999));
        assert_eq!(
            range_set,
            RangeSet {
                ranges: vec![-100..-51, -20..-15, 2..5],
            }
        );

        // Test redundant subtraction of range between existing ranges
        range_set.subtract_range(&(0..1));
        assert_eq!(
            range_set,
            RangeSet {
                ranges: vec![-100..-51, -20..-15, 2..5],
            }
        );

        // Test redundant subtraction of empty range
        range_set.subtract_range(&(10..-10));
        assert_eq!(
            range_set,
            RangeSet {
                ranges: vec![-100..-51, -20..-15, 2..5],
            }
        );

        // Test subtraction which truncates a left and right range, and removes a range in the middle
        range_set.subtract_range(&(-75..4));
        assert_eq!(
            range_set,
            RangeSet {
                ranges: vec![-100..-75, 4..5],
            }
        );

        // Test subtraction in the middle of a range
        range_set.subtract_range(&(-90..-85));
        assert_eq!(
            range_set,
            RangeSet {
                ranges: vec![-100..-90, -85..-75, 4..5],
            }
        );

        // Test subtraction which removes a range in the middle
        range_set.subtract_range(&(-85..-75));
        assert_eq!(
            range_set,
            RangeSet {
                ranges: vec![-100..-90, 4..5],
            }
        );

        // Test subtraction which truncates left range only
        range_set.subtract_range(&(-200..-95));
        assert_eq!(
            range_set,
            RangeSet {
                ranges: vec![-95..-90, 4..5],
            }
        );

        // Test subtraction which removes right range
        range_set.subtract_range(&(0..100));
        assert_eq!(
            range_set,
            RangeSet {
                ranges: vec![-95..-90],
            }
        );

        // Test subtraction which removes last range
        range_set.subtract_range(&(-1000..1000));
        assert_eq!(range_set, RangeSet { ranges: vec![] });

        // Test redundant subtraction on empty range set
        range_set.subtract_range(&(-1000..1000));
        assert_eq!(range_set, RangeSet { ranges: vec![] });
    }
}
