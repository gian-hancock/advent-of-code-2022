use std::{
    collections::{HashSet, VecDeque},
    fs,
};

struct HeightMap {
    heights: Vec<i32>,
    width: i32,
    height: i32,
    start: (i32, i32),
    end: (i32, i32),
}

fn main() {
    let height_map = HeightMap::parse(fs::read_to_string("input.txt").unwrap());
    #[rustfmt::skip]
    println!("Part 1: {}", bfs(&height_map, height_map.start, 1, &|pos| pos == height_map.end));
    #[rustfmt::skip]
    println!(
        "Part 2: {}", 
        bfs(&height_map, height_map.end, -1, &|pos| height_map.get(pos.0, pos.1) == 0));
}

fn bfs(
    height_map: &HeightMap,
    start: (i32, i32),
    max_step: i32,
    end_condition: &dyn Fn((i32, i32)) -> bool,
) -> i32 {
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();
    queue.push_back((start, 0));
    visited.insert(start);
    while let Some((cur_pos, path_len)) = queue.pop_front() {
        if end_condition(cur_pos) {
            return path_len;
        };
        let cur_height = height_map.get(cur_pos.0, cur_pos.1);
        // ## Push adjacent positions to queue
        const DIRECTIONS: [(i32, i32); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];
        for direction in DIRECTIONS {
            let new_pos = (cur_pos.0 + direction.0, cur_pos.1 + direction.1);
            // Ignore this position if it is out of bounds, or has already been visited
            if new_pos.0 < 0
                || new_pos.0 >= height_map.width
                || new_pos.1 < 0
                || new_pos.1 >= height_map.height
                || visited.contains(&new_pos)
            {
                continue;
            }
            let new_height = height_map.get(new_pos.0, new_pos.1);

            if (new_height - cur_height) * max_step.signum() > max_step.abs() {
                continue;
            }
            visited.insert(new_pos);
            queue.push_back((new_pos, path_len + 1));
        }
    }
    panic!();
}

impl HeightMap {
    fn get(&self, x: i32, y: i32) -> i32 {
        self.heights[(y * self.width + x) as usize]
    }

    fn parse(data: String) -> HeightMap {
        let mut start = (0, 0);
        let mut end = (0, 0);
        let mut heights = Vec::new();
        let mut width = 0;
        let mut height = 0;
        for (y, line) in data.lines().enumerate() {
            height = y + 1;
            for (x, char) in line.chars().enumerate() {
                width = x + 1;
                let height = match char {
                    'S' => {
                        start = (x.try_into().unwrap(), y.try_into().unwrap());
                        0
                    }
                    'E' => {
                        end = (x.try_into().unwrap(), y.try_into().unwrap());
                        u32::from('z') - u32::from('a')
                    }
                    _ => u32::from(char) - u32::from('a'),
                };
                heights.push(height.try_into().unwrap());
            }
        }
        HeightMap {
            heights,
            width: width.try_into().unwrap(),
            height: height.try_into().unwrap(),
            start,
            end,
        }
    }
}
