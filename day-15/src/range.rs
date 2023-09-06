use std::ops::Range;

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

    // TODO: Use binary search
    pub fn add_range(&mut self, range_to_add: &Range<i32>) {
        if range_to_add.is_empty() { return }
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

    // TODO: Use binary search
    pub fn subtract_range(&mut self, range_to_subtract: &Range<i32>) {
        if range_to_subtract.is_empty() { return }
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
        if left_range_to_truncate_idx >= 0 && left_range_to_truncate_idx < self.ranges.len() as i32 {
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

#[cfg(test)]
mod tests {
    use crate::range::{RangeOps, RangeSet};

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
        assert_eq!(
            range_set,
            RangeSet {
                ranges: vec![],
            }
        );
    
        // Test redundant subtraction on empty range set
        range_set.subtract_range(&(-1000..1000));
        assert_eq!(
            range_set,
            RangeSet {
                ranges: vec![],
            }
        );
    }    
}