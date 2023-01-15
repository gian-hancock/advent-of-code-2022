use day_14::{Map, Vec2, ORIGIN};

fn main() {
    let input = include_str!("../../input.txt");

    let mut map = Map::parse(input);

    // Simulate a single grain of sand. Returns whether the grain of sand fell of the edge of the
    // map.
    let mut simulate_grain = || {
        let mut pos = ORIGIN;
        loop {
            if pos.y > map.max.y {
                return true;
            }
            let candidate_positions = [
                pos.add(&Vec2::DOWN),
                pos.add(&Vec2::DOWN_LEFT),
                pos.add(&Vec2::DOWN_RIGHT),
            ];
            let mut has_moved = false;
            for candidate_position in &candidate_positions {
                if !map.points.contains_key(candidate_position) {
                    has_moved = true;
                    pos = candidate_position.clone();
                    break;
                }
            }
            if !has_moved {
                map.points.insert(pos, 'o');
                return false;
            }
        }
    };

    let mut grains = 0;
    while !simulate_grain() {
        grains += 1;
    }
    map.print();
    println!("grains: {}", grains);
}
