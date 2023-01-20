use std::{collections::{HashSet}, ops::Sub, process::exit};
use day_15::{Vec2, self};

const DIMENSION: i32 = 4_000_000;

fn main() {
    let input = include_str!("../../input.txt");
    let sensors = day_15::parse(input);

    let mut iter_cnt = 0;
    let mut x = 0;
    let mut y = 0;
    loop {
        'middle: loop {
            let mut global_in_range_of_sensor = false;
            let mut next_x = x as i32 + 1;
            for (sensor, range) in sensors.iter() {
                if x >= DIMENSION { break 'middle; }
                let delta_y = (y as i32 - sensor.y).abs();
                let delta_x = (x as i32 - sensor.x).abs();
                let manhattan_dis = delta_x + delta_y;
                let in_range_of_sensor = manhattan_dis <= *range;
                global_in_range_of_sensor = global_in_range_of_sensor || in_range_of_sensor;
                if in_range_of_sensor {
                    let sensor_next_x = sensor.x + *range - delta_y;
                    next_x = next_x.max(sensor_next_x);
                    x = next_x;
                }
            }

            if !global_in_range_of_sensor {
                println!("{x}, {y}: {}", x as u64 * 4_000_000 + y as u64);
                exit(0);
            }
            iter_cnt += 1;
        }
        y += 1;
        x = 0;
    }
}
