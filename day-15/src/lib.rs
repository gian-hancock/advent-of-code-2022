#[derive(Debug, Eq, Hash, PartialEq, Clone)]
pub struct Vec2 {
    pub x: i32,
    pub y: i32,
}

impl Vec2 {
}

pub fn parse(input: &str) -> Vec<(Vec2, i32)> {
    fn parse_int(s: &str, trailing_chars: usize) -> i32 {
        (&s[2..s.len()-trailing_chars]).parse().unwrap()
    }

    let mut sensors = Vec::new();
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
        sensors.push((sensor, range));
    }
    sensors
}