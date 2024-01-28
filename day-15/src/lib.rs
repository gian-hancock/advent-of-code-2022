use std::ops::Range;

pub mod border_intersection;
pub mod column_skipping;
pub mod range_exclusion;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Aabb {
    x: Range<i32>,
    y: Range<i32>,
}

impl Aabb {
    fn contains(&self, vec: &Vec2) -> bool {
        self.x.contains(&vec.x) && self.y.contains(&vec.y)
    }
}

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
    id: usize,
    pub pos: Vec2,
    pub range: i32,
}

pub mod test_case {
    use crate::{Sensor, Vec2};

    #[derive(Debug, Clone)]
    pub struct ConstTestCase {
        pub name: &'static str,
        pub sensors: &'static [Sensor],
        pub dimension: i32,
        pub expected_pos: Vec2,
    }

    impl From<&ConstTestCase> for TestCase {
        fn from(test_case: &ConstTestCase) -> Self {
            Self {
                name: test_case.name,
                sensors: test_case.sensors.to_vec(),
                dimension: test_case.dimension,
                expected_pos: test_case.expected_pos.clone(),
            }
        }
    }
    #[derive(Debug, Clone)]
    pub struct FileTestCase {
        pub name: &'static str,
        pub input: &'static str,
        pub search_size: i32,
        pub expected_pos: Vec2,
    }

    impl From<&FileTestCase> for TestCase {
        fn from(file_test_case: &FileTestCase) -> Self {
            Self {
                name: file_test_case.name,
                sensors: crate::parse(file_test_case.input),
                dimension: file_test_case.search_size,
                expected_pos: file_test_case.expected_pos.clone(),
            }
        }
    }

    #[derive(Debug, Clone)]
    pub struct TestCase {
        pub name: &'static str,
        pub sensors: Vec<Sensor>,
        pub dimension: i32,
        pub expected_pos: Vec2,
    }

    impl TestCase {
        pub fn expected_answer(&self) -> i64 {
            self.expected_pos.x as i64 * self.dimension as i64 + self.expected_pos.y as i64
        }

        pub fn rotated(&self) -> Self {
            fn rotate_vec(v: &Vec2, dim: i32) -> Vec2 {
                Vec2 {
                    x: v.y,
                    y: dim - v.x,
                }
            }
            let mut sensors = Vec::new();
            for sensor in self.sensors.iter() {
                sensors.push(Sensor {
                    id: sensor.id,
                    pos: rotate_vec(&sensor.pos, self.dimension),
                    range: sensor.range,
                });
            }
            TestCase {
                name: self.name,
                sensors,
                dimension: self.dimension,
                expected_pos: rotate_vec(&self.expected_pos, self.dimension),
            }
        }
    }

    pub const HALL: ConstTestCase = ConstTestCase {
        name: "Hall",
        sensors: &[
            Sensor {
                id: 0,
                pos: Vec2 { x: -1, y: 3 },
                range: 3,
            },
            Sensor {
                id: 1,
                pos: Vec2 { x: 3, y: 3 },
                range: 2,
            },
            Sensor {
                id: 2,
                pos: Vec2 { x: -1, y: -1 },
                range: 2,
            },
            Sensor {
                id: 3,
                pos: Vec2 { x: 3, y: -1 },
                range: 3,
            },
        ],
        dimension: 2,
        expected_pos: Vec2 { x: 1, y: 1 },
    };

    pub const MINIMAL: ConstTestCase = ConstTestCase {
        name: "Minimal",
        sensors: &[
            Sensor {
                id: 0,
                pos: Vec2 { x: 1, y: 1 },
                range: 1,
            },
            Sensor {
                id: 1,
                pos: Vec2 { x: 2, y: 0 },
                range: 2,
            },
        ],
        dimension: 2,
        expected_pos: Vec2 { x: 0, y: 2 },
    };

    pub const BOX: ConstTestCase = ConstTestCase {
        name: "Box",
        sensors: &[
            Sensor {
                id: 0,
                pos: Vec2 { x: 0, y: 0 },
                range: 1,
            },
            Sensor {
                id: 1,
                pos: Vec2 { x: 0, y: 2 },
                range: 1,
            },
            Sensor {
                id: 2,
                pos: Vec2 { x: 2, y: 0 },
                range: 1,
            },
            Sensor {
                id: 3,
                pos: Vec2 { x: 2, y: 2 },
                range: 1,
            },
        ],
        dimension: 2,
        expected_pos: Vec2 { x: 1, y: 1 },
    };

    pub const FLOWER: ConstTestCase = ConstTestCase {
        name: "Flower",
        sensors: &[
            Sensor {
                id: 0,
                pos: Vec2 { x: 0, y: 1 },
                range: 1,
            },
            Sensor {
                id: 1,
                pos: Vec2 { x: 0, y: 2 },
                range: 1,
            },
            Sensor {
                id: 2,
                pos: Vec2 { x: 1, y: 4 },
                range: 1,
            },
            Sensor {
                id: 3,
                pos: Vec2 { x: 2, y: 4 },
                range: 1,
            },
            Sensor {
                id: 4,
                pos: Vec2 { x: 4, y: 3 },
                range: 1,
            },
            Sensor {
                id: 5,
                pos: Vec2 { x: 4, y: 2 },
                range: 1,
            },
            Sensor {
                id: 6,
                pos: Vec2 { x: 3, y: 0 },
                range: 1,
            },
            Sensor {
                id: 7,
                pos: Vec2 { x: 2, y: 0 },
                range: 1,
            },
        ],
        dimension: 4,
        expected_pos: Vec2 { x: 2, y: 2 },
    };

    pub const SPLAT: ConstTestCase = ConstTestCase {
        name: "Splat",
        sensors: &[
            Sensor {
                id: 0,
                pos: Vec2 { x: -1, y: 0 },
                range: 1,
            },
            Sensor {
                id: 1,
                pos: Vec2 { x: 0, y: 2 },
                range: 1,
            },
            Sensor {
                id: 2,
                pos: Vec2 { x: 2, y: 3 },
                range: 1,
            },
            Sensor {
                id: 3,
                pos: Vec2 { x: 3, y: 1 },
                range: 1,
            },
            Sensor {
                id: 4,
                pos: Vec2 { x: 1, y: -1 },
                range: 1,
            },
            Sensor {
                id: 5,
                pos: Vec2 { x: 2, y: -1 },
                range: 1,
            },
        ],
        dimension: 2,
        expected_pos: Vec2 { x: 1, y: 1 },
    };

    pub const CORNER: ConstTestCase = ConstTestCase {
        name: "Corner",
        sensors: &[Sensor {
            id: 0,
            pos: Vec2 { x: 2, y: 2 },
            range: 3,
        }],
        dimension: 3,
        expected_pos: Vec2 { x: 0, y: 0 },
    };

    pub const EDGE_1: ConstTestCase = ConstTestCase {
        name: "Edge 1",
        sensors: &[
            Sensor {
                id: 0,
                pos: Vec2 { x: 1, y: 0 },
                range: 1,
            },
            Sensor {
                id: 1,
                pos: Vec2 { x: 1, y: 2 },
                range: 1,
            },
            Sensor {
                id: 2,
                pos: Vec2 { x: 2, y: 1 },
                range: 1,
            },
        ],
        dimension: 2,
        expected_pos: Vec2 { x: 0, y: 1 },
    };

    pub const EDGE_2: ConstTestCase = ConstTestCase {
        name: "Edge 2",
        sensors: &[
            Sensor {
                id: 0,
                pos: Vec2 { x: 0, y: -1 },
                range: 1,
            },
            Sensor {
                id: 1,
                pos: Vec2 { x: 2, y: 0 },
                range: 1,
            },
            Sensor {
                id: 2,
                pos: Vec2 { x: 1, y: 2 },
                range: 1,
            },
        ],
        dimension: 2,
        expected_pos: Vec2 { x: 0, y: 1 },
    };

    pub const SMALL_CONST: ConstTestCase = ConstTestCase {
        name: "Small",
        sensors: &[
            Sensor {
                id: 0,
                pos: Vec2 { x: 0, y: 0 },
                range: 1,
            },
            Sensor {
                id: 1,
                pos: Vec2 { x: 2, y: 2 },
                range: 1,
            },
            Sensor {
                id: 2,
                pos: Vec2 { x: 0, y: 2 },
                range: 0,
            },
            Sensor {
                id: 3,
                pos: Vec2 { x: 2, y: 0 },
                range: 0,
            },
        ],
        dimension: 2,
        expected_pos: Vec2 { x: 1, y: 1 },
    };

    pub const AOC_ACTUAL: FileTestCase = FileTestCase {
        name: "AOC Actual",
        input: include_str!("../test_cases/aoc_actual.txt"),
        search_size: 4_000_000,
        expected_pos: Vec2 {
            x: 3270298,
            y: 2638237,
        },
    };

    pub const AOC_EXAMPLE: FileTestCase = FileTestCase {
        name: "AOC Example",
        input: include_str!("../test_cases/aoc_example.txt"),
        search_size: 20,
        expected_pos: Vec2 { x: 14, y: 11 },
    };

    pub const SMALL_FILE: FileTestCase = FileTestCase {
        name: "Small 1",
        input: include_str!("../test_cases/small_1.txt"),
        search_size: 2,
        expected_pos: Vec2 { x: 1, y: 1 },
    };

    pub const FILE_TEST_CASES: &[FileTestCase] = &[AOC_ACTUAL, AOC_EXAMPLE, SMALL_FILE];
    pub const CONST_TEST_CASES: &[ConstTestCase] = &[
        HALL,
        MINIMAL,
        BOX,
        FLOWER,
        SPLAT,
        CORNER,
        EDGE_1,
        EDGE_2,
        SMALL_CONST,
    ];

    pub fn get_test_cases() -> impl Iterator<Item = TestCase> {
        let a = CONST_TEST_CASES.iter().map(Into::<TestCase>::into);
        let b = FILE_TEST_CASES.iter().map(Into::<TestCase>::into);
        a.chain(b)
    }
}

pub fn parse(input: &str) -> Vec<Sensor> {
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

        let range = sensor_pos.manhattan_distance(&beacon);
        sensors.push(Sensor {
            pos: sensor_pos,
            id: line_idx + 1,
            range,
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
                    sensor_line = Some(sensor.id);
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
