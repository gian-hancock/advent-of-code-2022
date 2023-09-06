use day_15::Vec2;
use std::{collections::HashSet, ops::Sub};

const LINE: i32 = 2_000_000;

fn main() {
    let input = include_str!("../../test_cases/aoc_actual.txt");

    let mut min_x = 0;
    let mut max_x = 0;
    let mut visited_x = HashSet::new();
    let mut beacon_x = HashSet::new();
    for line in input.lines() {
        let mut words = line.split_ascii_whitespace().skip(2);
        let sensor = Vec2 {
            x: parse_int(words.next().unwrap(), 1),
            y: parse_int(words.next().unwrap(), 1),
        };
        let mut words = words.skip(4);
        let beacon = Vec2 {
            x: parse_int(words.next().unwrap(), 1),
            y: parse_int(words.next().unwrap(), 0),
        };
        let range = (sensor.x - beacon.x).abs() + (sensor.y - beacon.y).abs();
        min_x = min_x.min(sensor.x - range);
        max_x = max_x.max(sensor.x + range);

        if beacon.y == LINE {
            beacon_x.insert(beacon.x);
        }
        for x in (sensor.x - range)..=(sensor.x + range) {
            let sensor_manhattan_distance = (sensor.x - x).abs() + (sensor.y - LINE).abs();
            if sensor_manhattan_distance <= range {
                visited_x.insert(x);
            }
        }
    }
    dbg!(visited_x.sub(&beacon_x).len());
    dbg!(visited_x.len());

    fn parse_int(s: &str, trailing_chars: usize) -> i32 {
        s[2..s.len() - trailing_chars].parse().unwrap()
    }
}
